//! GFF3 conversion and output for microbiorust
//!
//! Converts [`record::Record`] into GFF3 format for writing to disk
//! Currently does not parse GFF3, for that use rust-bio

use crate::record::{Feature, Record};
use std::collections::BTreeMap;
use std::fs::OpenOptions;
use std::io::{self, Write};

// ── GFF3 field 9 (attributes column) ─────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
pub struct GffAttributes {
    pub id:        String,
    pub name:      String,
    pub locus_tag: String,
    pub gene:      String,
    pub product:   String,
    // uncomment when needed:
    // pub inference: String,
    // pub parent:    String,
    // pub db_xref:   String,
}

impl GffAttributes {
    pub fn new(
        id:        String,
        name:      String,
        locus_tag: String,
        gene:      String,
        product:   String,
    ) -> Self {
        GffAttributes { id, name, locus_tag, gene, product }
    }

    pub fn to_field9(&self) -> String {
        let mut parts = Vec::new();
        if !self.id.is_empty()        { parts.push(format!("ID={}",        self.id));        }
        if !self.name.is_empty()      { parts.push(format!("Name={}",      self.name));      }
        if !self.gene.is_empty()      { parts.push(format!("gene={}",      self.gene));      }
        if !self.locus_tag.is_empty() { parts.push(format!("locus_tag={}", self.locus_tag)); }
        if !self.product.is_empty()   { parts.push(format!("product={}",   self.product));   }
        parts.join(";")
    }
}

// ── GFF3 data line ────────────────────────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
pub struct GffRecord {
    pub seqid:      String,
    pub source:     String,
    pub type_val:   String,
    pub start:      u32,
    pub end:        u32,
    pub score:      f64,
    pub strand:     String,
    pub phase:      u8,
    pub attributes: GffAttributes,  // owned, no lifetime needed
}

impl GffRecord {
    pub fn new(
        seqid:      String,
        source:     String,
        type_val:   String,
        start:      u32,
        end:        u32,
        score:      f64,
        strand:     String,
        phase:      u8,
        attributes: GffAttributes,
    ) -> Self {
        GffRecord { seqid, source, type_val, start, end, score, strand, phase, attributes }
    }

    /// Writes one tab-separated GFF3 line — no debug formatting on numerics
    pub fn to_gff_line(&self) -> String {
        format!(
            "{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}",
            self.seqid,
            self.source,
            self.type_val,
            self.start,        // plain u32, not {:?}
            self.end,          // plain u32, not {:?}
            self.score,
            self.strand,
            self.phase,
            self.attributes.to_field9(),
        )
    }
}

// ── Conversion from Record ────────────────────────────────────────────────────

fn strand_char(val: &i8) -> String {
    match val {
        1  => "+".to_string(),
        -1 => "-".to_string(),
        _  => {
            eprintln!("unexpected strand value {}", val);
            ".".to_string()
        }
    }
}

fn codon_start_to_phase(val: &u8) -> u8 {
    match val {
        1 => 0,
        2 => 1,
        3 => 2,
        _ => {
            eprintln!("unexpected codon_start value {}", val);
            0
        }
    }
}

fn record_to_gff_records(
    record:      &Record,
    source_name: &str,
    offset:      u32,          // cumulative offset for multi-contig files
) -> Vec<GffRecord> {
    let mut out = Vec::new();

    for locus_tag in record.cds.attributes.keys() {

        let start = record.cds.get_start(locus_tag)
            .and_then(|r| r.get_value())
            .unwrap_or_else(|| { eprintln!("start missing for {}", locus_tag); 0 });

        let stop = record.cds.get_stop(locus_tag)
            .and_then(|r| r.get_value())
            .unwrap_or_else(|| { eprintln!("stop missing for {}", locus_tag); 0 });

        let gene = record.cds.get_gene(locus_tag)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown".to_string());

        let product = record.cds.get_product(locus_tag)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "unknown product".to_string());

        let strand = record.cds.get_strand(locus_tag)
            .map(strand_char)
            .unwrap_or_else(|| ".".to_string());

        let phase = record.cds.get_codon_start(locus_tag)
            .map(codon_start_to_phase)
            .unwrap_or(0);

        let attrs = GffAttributes::new(
            locus_tag.to_string(),
            source_name.to_string(),
            locus_tag.to_string(),
            gene,
            product,
        );

        out.push(GffRecord::new(
            source_name.to_string(),
            ".".to_string(),
            "CDS".to_string(),
            start + offset,
            stop  + offset,
            0.0,
            strand,
            phase,
            attrs,
        ));
    }
    out
}

// ── Public write function — single entry point ────────────────────────────────

pub fn gff_write(
    seq_region:  &BTreeMap<String, (u32, u32)>,
    records:     &[Record],
    filename:    &str,
    include_dna: bool,
) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(filename)?;

    // header
    if file.metadata()?.len() == 0 {
        writeln!(file, "##gff-version 3")?;
    }
    for (name, (start, end)) in seq_region.iter() {
        writeln!(file, "##sequence-region\t{}\t{}\t{}", name, start, end)?;
    }

    // data lines
    let mut offset: u32 = 0;
    for ((source_name, (_seq_start, seq_end)), record) in
        seq_region.iter().zip(records.iter())
    {
        for gff_rec in record_to_gff_records(record, source_name, offset) {
            writeln!(file, "{}", gff_rec.to_gff_line())?;
        }
        offset = *seq_end;
    }

    // optional FASTA section
    if include_dna {
        writeln!(file, "##FASTA")?;
        for record in records {
            writeln!(file, "{}", record.sequence)?;
        }
    }

    Ok(())
}

use microBioRust::gbk::{gbk_write, gff_write, RangeValue, Record};
use std::{
     fs::File,
     collections::BTreeMap,
};

pub fn main() -> Result<(), anyhow::Error> {
         let filename = format!("new_record.gbk");
         if std::path::Path::new(&filename).exists() {
           std::fs::remove_file(&filename)?;
           }
	    let mut record = Record::new();
	    let mut seq_region: BTreeMap<String, (u32,u32)> = BTreeMap::new();
         //example from E.coli K12
	    seq_region.insert("source_1".to_string(), (1,897));
         //Add the source into SourceAttributes
         record.source_map
	         .set_counter("source_1".to_string())
	         .set_start(RangeValue::Exact(1))
	         .set_stop(RangeValue::Exact(897))
	         .set_organism("Escherichia coli".to_string())
	         .set_mol_type("DNA".to_string())
	         .set_strain("K-12 substr. MG1655".to_string())
		 .set_type_material("type strain of Escherichia coli K12".to_string())
	         .set_db_xref("PRJNA57779".to_string());
         //Add the features into FeatureAttributes, here we are setting two features, i.e. coding sequences or genes
	    record.cds
                  .set_counter("b3304".to_string())
                  .set_start(RangeValue::Exact(1))
                  .set_stop(RangeValue::Exact(354))
                  .set_gene("rplR".to_string())
                  .set_product("50S ribosomal subunit protein L18".to_string())
                  .set_codon_start(1)
                  .set_strand(-1);
	    record.cds
                  .set_counter("b3305".to_string())
                  .set_start(RangeValue::Exact(364))
                  .set_stop(RangeValue::Exact(897))
                  .set_gene("rplF".to_string())
                  .set_product("50S ribosomal subunit protein L6".to_string())
                  .set_codon_start(1)
                  .set_strand(-1);
         //Add the sequences for the coding sequence (CDS) into SequenceAttributes
	    record.seq_features
	         .set_counter("b3304".to_string())
		 .set_start(RangeValue::Exact(1))
                 .set_stop(RangeValue::Exact(354))
                 .set_sequence_ffn("ATGGATAAGAAATCTGCTCGTATCCGTCGTGCGACCCGCGCACGCCGCAAGCTCCAGGAG
CTGGGCGCAACTCGCCTGGTGGTACATCGTACCCCGCGTCACATTTACGCACAGGTAATT
GCACCGAACGGTTCTGAAGTTCTGGTAGCTGCTTCTACTGTAGAAAAAGCTATCGCTGAA
CAACTGAAGTACACCGGTAACAAAGACGCGGCTGCAGCTGTGGGTAAAGCTGTCGCTGAA
CGCGCTCTGGAAAAAGGCATCAAAGATGTATCCTTTGACCGTTCCGGGTTCCAATATCAT
GGTCGTGTCCAGGCACTGGCAGATGCTGCCCGTGAAGCTGGCCTTCAGTTCTAA".to_string())
                 .set_sequence_faa("MDKKSARIRRATRARRKLQELGATRLVVHRTPRHIYAQVIAPNGSEVLVAASTVEKAIAE
QLKYTGNKDAAAAVGKAVAERALEKGIKDVSFDRSGFQYHGRVQALADAAREAGLQF".to_string())
                 .set_codon_start(1)
                 .set_strand(-1);
	    record.seq_features
	         .set_counter("bb3305".to_string())
		 .set_start(RangeValue::Exact(364))
                 .set_stop(RangeValue::Exact(897))
                 .set_sequence_ffn("ATGTCTCGTGTTGCTAAAGCACCGGTCGTTGTTCCTGCCGGCGTTGACGTAAAAATCAAC
GGTCAGGTTATTACGATCAAAGGTAAAAACGGCGAGCTGACTCGTACTCTCAACGATGCT
GTTGAAGTTAAACATGCAGATAATACCCTGACCTTCGGTCCGCGTGATGGTTACGCAGAC
GGTTGGGCACAGGCTGGTACCGCGCGTGCCCTGCTGAACTCAATGGTTATCGGTGTTACC
GAAGGCTTCACTAAGAAGCTGCAGCTGGTTGGTGTAGGTTACCGTGCAGCGGTTAAAGGC
AATGTGATTAACCTGTCTCTGGGTTTCTCTCATCCTGTTGACCATCAGCTGCCTGCGGGT
ATCACTGCTGAATGTCCGACTCAGACTGAAATCGTGCTGAAAGGCGCTGATAAGCAGGTG
ATCGGCCAGGTTGCAGCGGATCTGCGCGCCTACCGTCGTCCTGAGCCTTATAAAGGCAAG
GGTGTTCGTTACGCCGACGAAGTCGTGCGTACCAAAGAGGCTAAGAAGAAGTAA".to_string())
                 .set_sequence_faa("MSRVAKAPVVVPAGVDVKINGQVITIKGKNGELTRTLNDAVEVKHADNTLTFGPRDGYAD
GWAQAGTARALLNSMVIGVTEGFTKKLQLVGVGYRAAVKGNVINLSLGFSHPVDHQLPAG
ITAECPTQTEIVLKGADKQVIGQVAADLRAYRRPEPYKGKGVRYADEVVRTKEAKKK".to_string())
                 .set_codon_start(1)
                 .set_strand(-1);
         //Add the full sequence of the entire record into the record.sequence
	    record.sequence = "TTAGAACTGAAGGCCAGCTTCACGGGCAGCATCTGCCAGTGCCTGGACACGACCATGATA
TTGGAACCCGGAACGGTCAAAGGATACATCTTTGATGCCTTTTTCCAGAGCGCGTTCAGC
GACAGCTTTACCCACAGCTGCAGCCGCGTCTTTGTTACCGGTGTACTTCAGTTGTTCAGC
GATAGCTTTTTCTACAGTAGAAGCAGCTACCAGAACTTCAGAACCGTTCGGTGCAATTAC
CTGTGCGTAAATGTGACGCGGGGTACGATGTACCACCAGGCGAGTTGCGCCCAGCTCCTG
GAGCTTGCGGCGTGCGCGGGTCGCACGACGGATACGAGCAGATTTCTTATCCATAGTGTT
ACCTTACTTCTTCTTAGCCTCTTTGGTACGCACGACTTCGTCGGCGTAACGAACACCCTT
GCCTTTATAAGGCTCAGGACGACGGTAGGCGCGCAGATCCGCTGCAACCTGGCCGATCAC
CTGCTTATCAGCGCCTTTCAGCACGATTTCAGTCTGAGTCGGACATTCAGCAGTGATACC
CGCAGGCAGCTGATGGTCAACAGGATGAGAGAAACCCAGAGACAGGTTAATCACATTGCC
TTTAACCGCTGCACGGTAACCTACACCAACCAGCTGCAGCTTCTTAGTGAAGCCTTCGGT
AACACCGATAACCATTGAGTTCAGCAGGGCACGCGCGGTACCAGCCTGTGCCCAACCGTC
TGCGTAACCATCACGCGGACCGAAGGTCAGGGTATTATCTGCATGTTTAACTTCAACAGC
ATCGTTGAGAGTACGAGTCAGCTCGCCGTTTTTACCTTTGATCGTAATAACCTGACCGTT
GATTTTTACGTCAACGCCGGCAGGAACAACGACCGGTGCTTTAGCAACACGAGACAT".to_string();
           gbk_write(seq_region, vec![record], &filename);
	   return Ok(());
      }

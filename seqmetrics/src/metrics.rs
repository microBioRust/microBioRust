use clap::Parser;
use std::fs::File;
use microBioRust_microSeqIO::gbk::Reader;
use std::io;
use std::collections::HashMap;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Arguments {
  #[clap(short, long)]
  filename: String,
}


// Define a macro to generate the getters for each amino acid
macro_rules! molweight_amino_acid_getters {
    ($struct_name:ident, $( ($field:ident, $full_name:ident, $three_letter:ident, $single_letter:ident) ),* ) => {
        impl $struct_name {
            $(
                // Full name getter
                fn $full_name(&self) -> f64 {
                    self.$field
                }
                // Three-letter code getter
                fn $three_letter(&self) -> f64 {
                    self.$field
                }
                // Single-letter code getter
                fn $single_letter(&self) -> f64 {
                    self.$field
                }
            )*
        }
    };
}

struct MolWeights {
    Alanine: f64,
    Arginine: f64,
    Asparagine: f64,
    Aspartate: f64,
    Cysteine: f64,
    Glutamate: f64,
    Glutamine: f64,
    Glycine: f64,
    Histidine: f64,
    Isoleucine: f64,
    Leucine: f64,
    Lysine: f64,
    Methionine: f64,
    Phenylalanine: f64,
    Proline: f64,
    Serine: f64,
    Threonine: f64,
    Tryptophan: f64,
    Tyrosine: f64,
    Valine: f64,
}

impl MolWeights {
    fn new() -> Self {
       Self {
              //masses from NIST chemistry webbook US Dept of commerce
              Alanine: 89.0932, //C3H7NO2
              Arginine: 174.2010, //C6H14N4O2
              Asparagine: 132.1179, //C4H8N2O3
              Aspartate: 133.1027, //C4H7NO4
              Cysteine: 121.158, //C3H7NO2S
              Glutamate: 147.1293, //C5H9NO4
              Glutamine: 146.1445, //C5H10N2O3
              Glycine: 75.0666, //C2H5NO2
              Histidine: 155.1546, //C6H9N3O2
              Isoleucine: 131.1729, //C6H13NO2
	      Leucine: 131.1729, //C6H13NO2
              Lysine: 146.1876, //C6H14N2O2
              Methionine: 149.211, //C5H11NO2S
              Phenylalanine: 165.1891, //C9H11NO2
              Proline: 115.1305, //C5H9NO2
              Serine: 105.0926, //C3H7NO2
              Threonine: 119.1192, //C4H9NO3
              Tryptophan: 204.2252, //C11H12N2O2
              Tyrosine: 181.1885, //C9H11NO3
              Valine: 117.1463, //C5H11NO2
             }
      }
}

pub fn molecular_weight(protein_seq: &str) -> f64 {
    let amino_weights: MolWeights = MolWeights::new();
    molweight_amino_acid_getters!(MolWeights,
             (Alanine, alanine, Ala, A),
             (Arginine, arginine, Arg, R),
             (Asparagine, asparagine, Asn, N),
             (Aspartate, aspartate, Asp, D),
             (Cysteine, cysteine, Cys, C),
             (Glutamine, glutamine, Gln, Q),
             (Glutamate, glutamate, Glu, E),
             (Glycine, glycine, Gly, G),
             (Histidine, histidine, His, H),
             (Isoleucine, isoleucine, Ile, I),
             (Leucine, leucine, Leu, L),
             (Lysine, lysine, Lys, K),
             (Methionine, methionine, Met, M),
             (Phenylalanine, phenylalanine, Phe, F),
             (Proline, proline, Pro, P),
             (Serine, serine, Ser, S),
             (Threonine, threonine, Thr, T),
             (Tryptophan, trytophan, Trp, W),
             (Tyrosine, tyrosine, Tyr, Y),
             (Valine, valine, Val, V)
             );
    let mut total_weight = 0.0;
    for ch in protein_seq.chars() {
       match ch {
           'A' => total_weight += amino_weights.A(),
	   'R' => total_weight += amino_weights.R(),
	   'N' => total_weight += amino_weights.N(),
	   'D' => total_weight += amino_weights.D(),
	   'C' => total_weight += amino_weights.C(),
	   'Q' => total_weight += amino_weights.Q(),
	   'E' => total_weight += amino_weights.E(),
	   'G' => total_weight += amino_weights.G(),
	   'H' => total_weight += amino_weights.H(),
	   'I' => total_weight += amino_weights.I(),
	   'L' => total_weight += amino_weights.L(),
	   'K' => total_weight += amino_weights.K(),
	   'M' => total_weight += amino_weights.M(),
	   'F' => total_weight += amino_weights.F(),
	   'P' => total_weight += amino_weights.P(),
	   'S' => total_weight += amino_weights.S(),
	   'T' => total_weight += amino_weights.T(),
	   'W' => total_weight += amino_weights.W(),
	   'Y' => total_weight += amino_weights.Y(),
	   'V' => total_weight += amino_weights.V(),
	   _ => continue,
	   }
      }
   let result_weight = total_weight - ((protein_seq.len() - 1) as f64 * 18.02);
   result_weight
}


    
#[cfg(test)]
mod tests {
    use super::*;

   #[test]
   pub fn collect_molecular_weight() -> Result<(), anyhow::Error> {
            let file_gbk = File::open("test_output.gbk")?;
            let mut reader = Reader::new(file_gbk);
            let mut records = reader.records();
	    let mut counts: HashMap<char, u64> = HashMap::new();
	    let mut molecular_weight_total: f64 = 0.0;
            loop {  
                match records.next() {  
                    Some(Ok(mut record)) => {
                       //println!("next record");
                       //println!("Record id: {:?}", record.id);
                       for (k, v) in &record.cds.attributes {
                           match record.seq_features.get_sequence_faa(&k) {
                                     Some(value) => { let seq_faa = value.to_string();
				                      println!("k is {:?} v is {:?} seq faa is {:?}", &k, &v, &seq_faa);
				                      molecular_weight_total = molecular_weight(&seq_faa);
                                                      println!(">{}|{}\n{}", &record.id, &k, molecular_weight_total);
                                                      },
                                     _ => (),
                                     };
                       
                           }
                    },
                    Some(Err(e)) => { println!("theres an err {:?}", e); },
                    None => {
                       println!("finished iteration");
                             break; },
                    }
               }
            return Ok(());
   }
}

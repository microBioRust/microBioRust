use microBioRust::embl::Reader;
use std::fs::File;
use clap::Parser;

#[derive(Parser,Default,Debug)]
#[clap(author="LCrossman",version,about="extracting protein fasta from gbk file")]
pub struct Arguments {
     #[clap(short,long)]
     filename: String,
}

///An example to print protein sequence fasta from either a single or multi-genbank file
fn main() -> Result<(), anyhow::Error> {
            //collect filename from --filename input
            let args = Arguments::parse();
            let file_embl = File::open(&args.filename).expect("could not open file");
            //create reader
            let mut reader = Reader::new(file_embl);
            //create records structure
            let mut records = reader.records();
            let mut read_counter: u32 = 0;
            loop {  
                match records.next() {
                    Some(Ok(mut record)) => {
                       //println!("next");
                       //println!("Record id: {:?}", record.id);
                       for (k,_v) in record.cds.attributes {
                           match record.seq_features.get_sequence_faa(&k) {
                                 Some(value) => { let seq_faa = value.to_string();
                                                  println!(">{}|{}\n{}", &record.id, &k, seq_faa);
                                                },
                                  _ => (),
                                  };
                       
                           }
                       read_counter+=1;
                    },
                    Some(Err(e)) => { println!("theres an err {:?}", e); },
                    None => break,
                    }
               }
            println!("Total records processed: {}", read_counter);
            Ok(())
}

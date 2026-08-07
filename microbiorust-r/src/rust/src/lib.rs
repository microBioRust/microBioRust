use extendr_api::prelude::*;
use microBioRust_seqmetrics::metrics::{
    amino_counts, amino_percentage, hydrophobicity, molecular_weight,
};
use once_cell::sync::Lazy;
use std::collections::HashMap;

#[extendr]
fn r_molecular_weight(protein_seq: &str) -> f64 {
    molecular_weight(protein_seq)
}

#[extendr]
fn r_hydrophobicity(protein_seq: &str, window_size: i32) -> Vec<f64> {
    hydrophobicity(protein_seq, window_size as usize)
}

#[extendr]
fn r_amino_counts(protein_seq: &str) -> Robj {
    let counts = amino_counts(protein_seq);
    let mut letter_chars: Vec<_> = counts.keys().cloned().collect();
    letter_chars.sort();
    let letter_code: Vec<String> = letter_chars.iter().map(|c| c.to_string()).collect();
    let values: Vec<u64> = letter_chars.iter().map(|k| counts[k]).collect();
    amino_counts(protein_seq);
    data_frame!(AminoAcid = letter_code, Count = values)
}

#[extendr]
fn r_amino_percentage(protein_seq: &str) -> Robj {
    let pcts = amino_percentage(protein_seq);
    let mut letter_chars: Vec<_> = pcts.keys().cloned().collect();
    letter_chars.sort();
    let letter_code: Vec<String> = letter_chars.iter().map(|c| c.to_string()).collect();
    let values: Vec<f64> = letter_chars.iter().map(|k| pcts[k]).collect();
    data_frame!(AminoAcid = letter_code, Percentage = values)
}

#[extendr]
fn r_aromaticity(seq: &str) -> f64 {
    //get the percentages from your existing function
    let results = amino_percentage(seq);

    //define the aromatic amino acids
    let aromatic_aas = ['Y', 'W', 'F'];

    //sum the relative frequencies
    aromatic_aas
        .iter()
        .filter_map(|&amino| results.get(&amino)) // safely get percentages if they exist
        .map(|&perc| perc / 100.0) // convert percentage to a frequency (0 to 1)
        .sum()
}
//load the underlying dipeptide stability values (see microBioRust_seqmetrics for more details)
static WEIGHTS_DATA: &str = include_str!("dipeptide_stability_values.csv");

static INSTABILITY_WEIGHTS: Lazy<HashMap<String, f64>> = Lazy::new(|| {
    let mut weights = HashMap::new();
    for line in WEIGHTS_DATA.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let parts: Vec<&str> = line.split(',').collect();
        if parts.len() == 2 {
            let key = parts[0].trim().to_string();
            if let Ok(val) = parts[1].trim().replace('—', "-").parse::<f64>() {
                weights.insert(key, val);
            }
        }
    }
    weights
});
//calculate the instability index of a protein from the sequence
pub fn instability_index(seq: &str) -> f64 {
    let chars: Vec<char> = seq.chars().collect();
    let mut total = 0.0;

    for window in chars.windows(2) {
        let pair = format!("{}{}", window[0], window[1]);
        if let Some(val) = INSTABILITY_WEIGHTS.get(&pair) {
            total += val;
        }
    }
    total
}

#[extendr]
fn r_instability_index(seq: &str) -> f64 {
    instability_index(seq)
}

extendr_module! {
    mod microbiorustr;
    fn r_molecular_weight;
    fn r_hydrophobicity;
    fn r_amino_counts;
    fn r_amino_percentage;
    fn r_aromaticity;
    fn r_instability_index;
}

use crate::models::AminoAcid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AminoAcidResponse {
    pub amino_acid: AminoAcid,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidNameResponse {
    pub name: String,
    pub short_name: String,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidSideChainResponse {
    pub name: String,
    pub side_chain: String,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidMolecularWeightResponse {
    pub name: String,
    pub molecular_weight: f64,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidCodonResponse {
    pub name: String,
    pub codons: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidCodonCountResponse {
    pub name: String,
    pub codon_count: usize,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidShortNameResponse {
    pub name: String,
    pub short_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct AminoAcidAbbreviationResponse {
    pub name: String,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize)]
pub struct RootResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
}

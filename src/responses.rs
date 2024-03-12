// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{
    Deserialize,
    Serialize,
};

use crate::models::AminoAcid;

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidResponse {
    pub amino_acid: AminoAcid,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidNameResponse {
    pub name:         String,
    pub short_name:   String,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidSideChainResponse {
    pub name:       String,
    pub side_chain: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidMolecularWeightResponse {
    pub name:             String,
    pub molecular_weight: f64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidCodonResponse {
    pub name:   String,
    pub codons: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidCodonCountResponse {
    pub name:        String,
    pub codon_count: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidShortNameResponse {
    pub name:       String,
    pub short_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidAbbreviationResponse {
    pub name:         String,
    pub abbreviation: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

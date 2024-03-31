// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use serde::{
    Deserialize,
    Serialize,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct AminoAcidDetailResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name:             Option<String>,
    pub short_name:       Option<String>,
    pub abbreviation:     Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side_chain:       Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub molecular_weight: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codons:           Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub codon_count:      Option<usize>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RootResponse {
    pub message: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    pub error: String,
}

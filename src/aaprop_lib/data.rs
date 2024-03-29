// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::AminoAcid;

const AMINO_ACID_DATA: &str = include_str!("amino_acid_data.json");

pub fn amino_acids() -> Vec<AminoAcid> {
    let mut amino_acids: Vec<AminoAcid> = Vec::new();
    let all_amino_acids: Vec<AminoAcid> = serde_json::from_str(AMINO_ACID_DATA).unwrap();
    for amino_acid in all_amino_acids {
        amino_acids.push(amino_acid);
    }
    amino_acids
}

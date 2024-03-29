// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::models::{
    AminoAcid,
    SideChain,
};

pub const AMINO_ACID_DATA: [AminoAcid; 20] = [
    AminoAcid::new(
        "Alanine",
        "Ala",
        "A",
        SideChain::Nonpolar,
        89.09,
        &["GCU", "GCC", "GCA", "GCG"],
    ),
    AminoAcid::new(
        "Arginine",
        "Arg",
        "R",
        SideChain::Positive,
        174.20,
        &["CGU", "CGC", "CGA", "CGG", "AGA", "AGG"],
    ),
    AminoAcid::new(
        "Asparagine",
        "Asn",
        "N",
        SideChain::Polar,
        132.12,
        &["AAU", "AAC"],
    ),
    AminoAcid::new(
        "Aspartic Acid",
        "Asp",
        "D",
        SideChain::Acidic,
        133.10,
        &["GAU", "GAC"],
    ),
    AminoAcid::new(
        "Cysteine",
        "Cys",
        "C",
        SideChain::Nonpolar,
        121.16,
        &["UGU", "UGC"],
    ),
    AminoAcid::new(
        "Glutamic Acid",
        "Glu",
        "E",
        SideChain::Acidic,
        147.13,
        &["GAA", "GAG"],
    ),
    AminoAcid::new(
        "Glutamine",
        "Gln",
        "Q",
        SideChain::Polar,
        146.15,
        &["CAA", "CAG"],
    ),
    AminoAcid::new(
        "Glycine",
        "Gly",
        "G",
        SideChain::Nonpolar,
        75.07,
        &["GGU", "GGC", "GGA", "GGG"],
    ),
    AminoAcid::new(
        "Histidine",
        "His",
        "H",
        SideChain::Positive,
        155.16,
        &["CAU", "CAC"],
    ),
    AminoAcid::new(
        "Isoleucine",
        "Ile",
        "I",
        SideChain::Nonpolar,
        131.17,
        &["AUU", "AUC", "AUA"],
    ),
    AminoAcid::new(
        "Leucine",
        "Leu",
        "L",
        SideChain::Nonpolar,
        131.17,
        &["UUA", "UUG", "CUU", "CUC", "CUA", "CUG"],
    ),
    AminoAcid::new(
        "Lysine",
        "Lys",
        "K",
        SideChain::Positive,
        146.19,
        &["AAA", "AAG"],
    ),
    AminoAcid::new(
        "Methionine",
        "Met",
        "M",
        SideChain::Nonpolar,
        149.21,
        &["AUG"],
    ),
    AminoAcid::new(
        "Phenylalanine",
        "Phe",
        "F",
        SideChain::Nonpolar,
        165.19,
        &["UUU", "UUC"],
    ),
    AminoAcid::new(
        "Proline",
        "Pro",
        "P",
        SideChain::Nonpolar,
        115.13,
        &["CCU", "CCC", "CCA", "CCG"],
    ),
    AminoAcid::new(
        "Serine",
        "Ser",
        "S",
        SideChain::Polar,
        105.09,
        &["UCU", "UCC", "UCA", "UCG", "AGU", "AGC"],
    ),
    AminoAcid::new(
        "Threonine",
        "Thr",
        "T",
        SideChain::Polar,
        119.12,
        &["ACU", "ACC", "ACA", "ACG"],
    ),
    AminoAcid::new(
        "Tryptophan",
        "Trp",
        "W",
        SideChain::Nonpolar,
        204.23,
        &["UGG"],
    ),
    AminoAcid::new(
        "Tyrosine",
        "Tyr",
        "Y",
        SideChain::Polar,
        181.19,
        &["UAU", "UAC"],
    ),
    AminoAcid::new(
        "Valine",
        "Val",
        "V",
        SideChain::Nonpolar,
        117.15,
        &["GUU", "GUC", "GUA", "GUG"],
    ),
];

#[must_use]
pub fn find_by_abbreviation(abbreviation: &str) -> Option<AminoAcid> {
    AMINO_ACID_DATA
        .iter()
        .find(|amino_acid| amino_acid.get_abbreviation() == abbreviation)
        .copied()
}

#[must_use]
pub fn find_by_name(name: &str) -> Option<AminoAcid> {
    AMINO_ACID_DATA
        .iter()
        .find(|amino_acid| amino_acid.get_name() == name)
        .copied()
}

#[must_use]
pub fn find_by_short_name(short_name: &str) -> Option<AminoAcid> {
    AMINO_ACID_DATA
        .iter()
        .find(|amino_acid| amino_acid.get_short_name() == short_name)
        .copied()
}

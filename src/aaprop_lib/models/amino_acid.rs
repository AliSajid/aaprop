// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

use serde::{
    Deserialize,
    Serialize,
};

use crate::SideChain;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct AminoAcid {
    name:             String,
    short_name:       String,
    abbreviation:     String,
    side_chain:       SideChain,
    molecular_weight: f64,
    codons:           Vec<String>,
}

impl AminoAcid {
    #[must_use]
    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    #[must_use]
    pub fn get_short_name(&self) -> String {
        self.short_name.clone()
    }

    #[must_use]
    pub fn get_abbreviation(&self) -> String {
        self.abbreviation.clone()
    }

    #[must_use]
    pub fn get_side_chain(&self) -> SideChain {
        self.side_chain.clone()
    }

    #[must_use]
    pub const fn get_molecular_weight(&self) -> f64 {
        self.molecular_weight
    }

    #[must_use]
    pub fn get_codons(&self) -> Vec<String> {
        self.codons.clone()
    }

    #[must_use]
    pub fn get_codon_count(&self) -> usize {
        self.codons.len()
    }
}

impl Display for AminoAcid {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}\tShort Name: {}\tAbbreviation: {}\tSide Chain: {}\tMolecular Weight: \
             {}\tCodons: [{}]",
            self.name,
            self.short_name,
            self.abbreviation,
            self.side_chain,
            self.molecular_weight,
            self.codons.join(", ")
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_name() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_name(), "Alanine");
    }
    #[test]
    fn test_get_short_name() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_short_name(), "Ala");
    }
    #[test]
    fn test_get_abbreviation() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_abbreviation(), "A");
    }
    #[test]
    fn test_get_side_chain() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_side_chain(), SideChain::Nonpolar);
    }
    #[test]
    fn test_get_molecular_weight() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_molecular_weight(), 89.09);
    }
    #[test]
    fn test_get_codons() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_codons(), vec!["GCU", "GCC", "GCA", "GCG"]);
    }
    #[test]
    fn test_get_codon_count() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(amino_acid.get_codon_count(), 4);
    }

    #[test]
    fn test_fmt() {
        let amino_acid = AminoAcid {
            name:             "Alanine".to_owned(),
            short_name:       "Ala".to_owned(),
            abbreviation:     "A".to_owned(),
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           vec!["GCU", "GCC", "GCA", "GCG"]
                .into_iter()
                .map(|s| s.to_owned())
                .collect(),
        };
        assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular \
             Weight: 89.09\tCodons: [GCU, GCC, GCA, GCG]"
        );
    }
}

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

use crate::{
    CodonSet,
    SideChain,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub struct AminoAcid {
    name:             &'static str,
    short_name:       &'static str,
    abbreviation:     &'static str,
    side_chain:       SideChain,
    molecular_weight: f64,
    codons:           CodonSet,
}

impl AminoAcid {
    pub const fn new(
        name: &'static str,
        short_name: &'static str,
        abbreviation: &'static str,
        side_chain: SideChain,
        molecular_weight: f64,
        codons: &'static [&'static str],
    ) -> Self {
        let codons = CodonSet::new(codons);
        Self {
            name,
            short_name,
            abbreviation,
            side_chain,
            molecular_weight,
            codons,
        }
    }

    #[must_use]
    pub const fn get_name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub const fn get_short_name(&self) -> &'static str {
        self.short_name
    }

    #[must_use]
    pub const fn get_abbreviation(&self) -> &'static str {
        self.abbreviation
    }

    #[must_use]
    pub const fn get_side_chain(&self) -> SideChain {
        self.side_chain
    }

    #[must_use]
    pub const fn get_molecular_weight(&self) -> f64 {
        self.molecular_weight
    }

    #[must_use]
    pub const fn get_codons(&self) -> CodonSet {
        self.codons
    }

    #[must_use]
    pub const fn get_codon_count(&self) -> usize {
        self.codons.get_num_codons()
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
            self.codons
        )
    }
}

#[cfg(test)]
mod tests {
    use rstest::{
        fixture,
        rstest,
    };

    use super::*;

    #[fixture]
    fn amino_acid() -> AminoAcid {
        let codon_set = CodonSet::new(&["GCU", "GCC", "GCA", "GCG"]);
        AminoAcid {
            name:             "Alanine",
            short_name:       "Ala",
            abbreviation:     "A",
            side_chain:       SideChain::Nonpolar,
            molecular_weight: 89.09,
            codons:           codon_set,
        }
    }

    #[rstest]
    fn test_get_name(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_name(), "Alanine");
    }
    #[rstest]
    fn test_get_short_name(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_short_name(), "Ala");
    }
    #[rstest]
    fn test_get_abbreviation(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_abbreviation(), "A");
    }
    #[rstest]
    fn test_get_side_chain(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_side_chain(), SideChain::Nonpolar);
    }
    #[rstest]
    fn test_get_molecular_weight(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_molecular_weight(), 89.09);
    }
    #[rstest]
    fn test_get_codons(amino_acid: AminoAcid) {
        let codon_set = CodonSet::new(&["GCU", "GCC", "GCA", "GCG"]);
        assert_eq!(amino_acid.get_codons(), codon_set);
    }
    #[rstest]
    fn test_get_codon_count(amino_acid: AminoAcid) {
        assert_eq!(amino_acid.get_codon_count(), 4);
    }

    #[rstest]
    fn test_fmt(amino_acid: AminoAcid) {
        assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular \
             Weight: 89.09\tCodons: [GCU, GCC, GCA, GCG]"
        );
    }
}

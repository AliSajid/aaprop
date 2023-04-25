mod models {
    use serde::{Deserialize, Serialize};
    use std::fmt::{self, Display, Formatter};
    use std::str::FromStr;
    use std::string::ToString;
    use strum_macros::EnumString;

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, EnumString)]
    #[strum(ascii_case_insensitive)]
    enum SideChain {
        Nonpolar,
        Polar,
        Acidic,
        Basic,
        Positive,
    }

    impl Display for SideChain {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.to_owned())
        }
    }

    #[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
    pub struct AminoAcid {
        name: String,
        short_name: String,
        abbreviation: String,
        side_chain: SideChain,
        molecular_weight: f64,
        codon: Vec<String>,
    }

    impl AminoAcid {
        #[must_use]
        pub fn new(
            name: &str,
            short_name: &str,
            abbreviation: &str,
            side_chain: &str,
            molecular_weight: f64,
            codon: &[&str],
        ) -> Self {
            Self {
                name: name.to_string(),
                short_name: short_name.to_string(),
                abbreviation: abbreviation.to_string(),
                side_chain: SideChain::from_str(side_chain).unwrap(),
                molecular_weight,
                codon: codon.iter().map(ToString::to_string).collect(),
            }
        }
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
        pub fn get_codon(&self) -> Vec<String> {
            self.codon.clone()
        }
        #[must_use]
        pub fn get_codon_string(&self) -> String {
            self.codon.join(", ")
        }
        #[must_use]
        pub fn get_codon_count(&self) -> usize {
            self.codon.len()
        }
    }

    impl Display for AminoAcid {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(
            f,
            "Name: {}\tShort Name: {}\tAbbreviation: {}\tSide Chain: {}\tMolecular Weight: {}\tCodon: {}",
            self.name,
            self.short_name,
            self.abbreviation,
            self.side_chain,
            self.molecular_weight,
            self.codon.join(", ")
        )
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_get_name() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_name(), "Alanine");
        }
        #[test]
        fn test_get_short_name() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_short_name(), "Ala");
        }
        #[test]
        fn test_get_abbreviation() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_abbreviation(), "A");
        }
        #[test]
        fn test_get_side_chain() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_side_chain(), SideChain::Nonpolar);
        }
        #[test]
        fn test_get_molecular_weight() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCU", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_molecular_weight(), 89.09);
        }
        #[test]
        fn test_get_codon() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon(), vec!["GCT", "GCC", "GCA", "GCG"]);
        }
        #[test]
        fn test_get_codon_string() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon_string(), "GCT, GCC, GCA, GCG");
        }
        #[test]
        fn test_get_codon_count() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(amino_acid.get_codon_count(), 4);
        }

        #[test]
        fn test_fmt() {
            let amino_acid = AminoAcid::new(
                "Alanine",
                "Ala",
                "A",
                "Nonpolar",
                89.09,
                &["GCT", "GCC", "GCA", "GCG"],
            );
            assert_eq!(
            format!("{}", amino_acid),
            "Name: Alanine\tShort Name: Ala\tAbbreviation: A\tSide Chain: Nonpolar\tMolecular Weight: 89.09\tCodon: GCT, GCC, GCA, GCG"
        );
        }
    }
}

mod data {
    use super::models;
}

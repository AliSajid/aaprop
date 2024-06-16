// SPDX-FileCopyrightText: 2023 - 2024 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct CodonSet {
    num_codons: usize,
    first:      Option<&'static str>,
    second:     Option<&'static str>,
    third:      Option<&'static str>,
    fourth:     Option<&'static str>,
    fifth:      Option<&'static str>,
    sixth:      Option<&'static str>,
}

impl CodonSet {
    #[must_use]
    pub const fn new(codons: &'static [&'static str]) -> Self {
        let num_codons = codons.len();
        let first = if codons.is_empty() {
            None
        } else {
            Some(codons[0])
        };
        let second = if codons.len() > 1 {
            Some(codons[1])
        } else {
            None
        };
        let third = if codons.len() > 2 {
            Some(codons[2])
        } else {
            None
        };
        let fourth = if codons.len() > 3 {
            Some(codons[3])
        } else {
            None
        };
        let fifth = if codons.len() > 4 {
            Some(codons[4])
        } else {
            None
        };
        let sixth = if codons.len() > 5 {
            Some(codons[5])
        } else {
            None
        };

        Self {
            num_codons,
            first,
            second,
            third,
            fourth,
            fifth,
            sixth,
        }
    }

    #[must_use]
    pub const fn get_num_codons(&self) -> usize {
        self.num_codons
    }

    #[must_use]
    pub const fn get_all(&self) -> [&'static str; 6] {
        let mut result = ["", "", "", "", "", ""];
        let mut i = 0;
        while i < self.num_codons {
            let codon = match i {
                0 => self.first,
                1 => self.second,
                2 => self.third,
                3 => self.fourth,
                4 => self.fifth,
                5 => self.sixth,
                _ => unreachable!(),
            };
            if let Some(codon) = codon {
                result[i] = codon;
            }
            i += 1;
        }
        result
    }
}

impl Display for CodonSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut codons = String::new();
        for i in 0..self.num_codons {
            let codon = match i {
                0 => self.first,
                1 => self.second,
                2 => self.third,
                3 => self.fourth,
                4 => self.fifth,
                5 => self.sixth,
                _ => unreachable!(),
            };
            if let Some(codon) = codon {
                codons.push_str(codon);
                codons.push_str(", ");
            }
        }
        write!(f, "{}", codons.trim_end_matches(", "))
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case::zero_codons(&[], 0)]
    #[case::one_codon(&["AUG"], 1)]
    #[case::two_codons(&["AUG", "GCU"], 2)]
    #[case::three_codons(&["AUG", "GCU", "UAA"], 3)]
    #[case::four_codons(&["AUG", "GCU", "UAA", "UAG"], 4)]
    #[case::five_codons(&["AUG", "GCU", "UAA", "UAG", "UGA"], 5)]
    #[case::six_codons(&["AUG", "GCU", "UAA", "UAG", "UGA", "UCC"], 6)]
    fn test_new(#[case] codons: &'static [&'static str], #[case] codons_len: usize) {
        let codon_set = CodonSet::new(codons);

        assert_eq!(codon_set.get_num_codons(), codons_len);
    }

    #[rstest]
    #[case::zero_codons(&[], ["", "", "", "", "", ""])]
    #[case::one_codon(&["AUG"], ["AUG", "", "", "", "", ""])]
    #[case::two_codons(&["AUG", "GCU"], ["AUG", "GCU", "", "", "", ""])]
    #[case::three_codons(&["AUG", "GCU", "UAA"], ["AUG", "GCU", "UAA", "", "", ""])]
    #[case::four_codons(&["AUG", "GCU", "UAA", "UAG"], ["AUG", "GCU", "UAA", "UAG", "", ""])]
    #[case::five_codons(&["AUG", "GCU", "UAA", "UAG", "UGA"], ["AUG", "GCU", "UAA", "UAG", "UGA", ""])]
    #[case::six_codons(&["AUG", "GCU", "UAA", "UAG", "UGA", "UCC"], ["AUG", "GCU", "UAA", "UAG", "UGA", "UCC"])]
    fn test_get_all(#[case] codons: &'static [&'static str], #[case] expected: [&'static str; 6]) {
        let codon_set = CodonSet::new(codons);

        assert_eq!(codon_set.get_all(), expected);
    }

    #[rstest]
    #[case::zero_codons(&[], "")]
    #[case::one_codon(&["AUG"], "AUG")]
    #[case::two_codons(&["AUG", "GCU"], "AUG, GCU")]
    #[case::three_codons(&["AUG", "GCU", "UAA"], "AUG, GCU, UAA")]
    #[case::four_codons(&["AUG", "GCU", "UAA", "UAG"], "AUG, GCU, UAA, UAG")]
    #[case::five_codons(&["AUG", "GCU", "UAA", "UAG", "UGA"], "AUG, GCU, UAA, UAG, UGA")]
    #[case::six_codons(&["AUG", "GCU", "UAA", "UAG", "UGA", "UCC"], "AUG, GCU, UAA, UAG, UGA, UCC")]
    fn test_display(#[case] codons: &'static [&'static str], #[case] expected: &str) {
        let codon_set = CodonSet::new(codons);

        assert_eq!(codon_set.to_string(), expected);
    }
}

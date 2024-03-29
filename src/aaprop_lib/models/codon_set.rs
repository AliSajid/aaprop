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
                _ => None,
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
                _ => None,
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
    use rstest::{
        fixture,
        rstest,
    };

    use super::*;

    #[fixture]
    fn codons() -> &'static [&'static str] {
        &["AUG", "GCU", "UAA", "UAG", "UGA"]
    }

    #[rstest]
    fn test_new(codons: &'static [&'static str]) {
        let codon_set = CodonSet::new(codons);

        assert_eq!(codon_set.get_num_codons(), 5);
    }

    #[rstest]
    fn test_display(codons: &'static [&'static str]) {
        let codon_set = CodonSet::new(codons);

        assert_eq!(codon_set.to_string(), "AUG, GCU, UAA, UAG, UGA");
    }
}

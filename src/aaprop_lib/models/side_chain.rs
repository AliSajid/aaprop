// SPDX-FileCopyrightText: 2023 - 2025 Ali Sajid Imami
//
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use std::fmt::{
    self,
    Display,
    Formatter,
};

#[derive(Debug, Clone, PartialEq, Eq, Copy)]
pub enum SideChain {
    Nonpolar,
    Polar,
    Acidic,
    Basic,
    Positive,
}

impl Display for SideChain {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_side_chain_display() {
        assert_eq!(format!("{}", SideChain::Nonpolar), "Nonpolar");
        assert_eq!(format!("{}", SideChain::Polar), "Polar");
        assert_eq!(format!("{}", SideChain::Acidic), "Acidic");
        assert_eq!(format!("{}", SideChain::Basic), "Basic");
        assert_eq!(format!("{}", SideChain::Positive), "Positive");
    }
}

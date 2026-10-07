// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use crate::OpPolarity;
use crate::OpType;
use crate::BaseOp;

// ---- OpType ---------------------------------------------------------------
/*
impl From<OpType> for bool {
    #[inline(always)]
    fn from(ot: OpType) -> bool {
        match ot {
            OpType::AND => false,
            OpType::OR  => true,
        }
    }
}

impl From<bool> for OpType {
    #[inline(always)]
    fn from(b: bool) -> OpType {
        match b {
            false => OpType::AND,
            true  => OpType::OR
        }
    }
}
*/
impl ops::Not for OpType {
    type Output = OpType;

    #[inline(always)]
    fn not(self) -> OpType {
        match self {
            OpType::AND       => OpType::OR,
            OpType::OR        => OpType::AND,
            OpType::UNSET     => OpType::UNSET
        }
    }
}

impl std::fmt::Display for OpType {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            OpType::AND       => "AND",
            OpType::OR        => "OR",
            OpType::UNSET     => "UNSET"
        };
        f.write_str(name)
    }
}

// ---- OpPolarity -----------------------------------------------------------

impl From<OpPolarity> for bool {
    #[inline(always)]
    fn from(ot: OpPolarity) -> bool {
        match ot {
            OpPolarity::KEEP => false,
            OpPolarity::NOT  => true,
        }
    }
}

impl From<bool> for OpPolarity {
    #[inline(always)]
    fn from(b: bool) -> OpPolarity {
        match b {
            false => OpPolarity::KEEP,
            true  => OpPolarity::NOT
        }
    }
}

impl ops::Not for OpPolarity {
    type Output = OpPolarity;

    #[inline(always)]
    fn not(self) -> OpPolarity {
        OpPolarity::from(!bool::from(self))
    }
}

impl std::fmt::Display for OpPolarity {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(if OpPolarity::KEEP == *self { &"" } else { &"NOT" })
         .finish()
    }
}

// ---- BaseOp ---------------------------------------------------------------

impl ops::Not for BaseOp {
    type Output = BaseOp;

    #[inline(always)]
    fn not(self) -> BaseOp {
        BaseOp {
            isset: !self.isset,
            mask:   self.mask
        }
    }
}

impl std::cmp::PartialEq for BaseOp {
    #[inline(always)]
    fn eq(&self, other:&BaseOp) -> bool {
        self.mask == other.mask
            && (self.isset & self.mask) == (other.isset & other.mask)
    }
}

impl std::cmp::PartialEq<u32> for BaseOp {
    #[inline(always)]
    fn eq(&self, other:&u32) -> bool {
        (self.isset & self.mask) == *other
    }
}

impl std::hash::Hash for BaseOp {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.mask);
        state.write_u32(self.isset & self.mask)
    }
}

impl std::fmt::Debug for BaseOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.isset)
         .field(&self.mask)
         .finish()
    }
}

impl std::fmt::Display for BaseOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.isset)
         .field(&self.mask)
         .finish()
    }
}

impl BaseOp {
    /// Checks if the current BaseOp is valid in the context of another BaseOp.
    /// The operation is valid if the overlapping bits (defined by the mask) of both
    /// BaseOp instances do not conflict and neither mask is zero.
    #[inline(always)]
    pub fn is_valid(self, other:BaseOp) -> bool {
        ((self.mask & other.mask & (self.isset ^ other.isset)) == 0)
         && (0 != self.mask) && (0 != other.mask)
    }

    /// Combines two BaseOp instances into a new BaseOp.
    /// The combination is valid if there are no conflicting bits in the overlapping
    /// mask regions of both BaseOp instances. If there are conflicts or if any mask
    /// is zero, the resulting BaseOp will represent an invalid state.
    pub fn combine(self, other:BaseOp) -> BaseOp {
        let invalid =
            ((self.mask & other.mask & (self.isset ^ other.isset)) != 0)
         || (0 == self.mask) || (0 == other.mask);
        BaseOp {
            isset: (self.isset & self.mask) | (other.isset & other.mask),
            mask:  (!(-(invalid as i32)) as u32) & (self.mask | other.mask)
        }
    }

    /// Computes the difference between the current BaseOp and a given value.
    /// The difference is calculated by XORing the isset bits with the given value,
    /// then masking the result with the mask to retain only the relevant bits.
    #[inline(always)]
    pub fn diff(self, val:u32) -> u32 {
        (self.isset ^ val) & self.mask
    }

    /// Extracts the relevant bits from the current BaseOp.
    /// The extraction is done by ANDing the isset bits with the mask.
    #[inline(always)]
    pub fn extract(self) -> u32 {
        self.isset & self.mask
    }

}

// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use crate::BaseOp;
use crate::AndOp;

impl ops::Not for AndOp {
    type Output = AndOp;

    #[inline(always)]
    fn not(self) -> AndOp {
        AndOp { data: !self.data }
    }
}

impl ops::BitAndAssign<AndOp> for AndOp {
    #[inline(always)]
    fn bitand_assign(&mut self, rhs: Self) {
        *self = AndOp { data: self.data.combine(rhs.data) }
    }
}

impl ops::BitAnd<AndOp> for AndOp {
    type Output = AndOp;

    #[inline(always)]
    fn bitand(self, other: AndOp) -> AndOp {
        AndOp { data: self.data.combine(other.data) }
    }
}

impl std::fmt::Debug for AndOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.data.isset)
         .field(&self.data.mask)
         .finish()
    }
}

impl std::fmt::Display for AndOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.data.isset)
         .field(&self.data.mask)
         .finish()
    }
}


impl AndOp {
    #[inline(always)]
    pub fn new(isset: u32, mask: u32) -> Self {
        Self {  data: BaseOp { isset, mask } }
    }

    #[inline(always)]
    pub fn is_valid(self) -> bool {
        self.data.mask != 0
    }

    #[inline(always)]
    pub fn is_zero(self) -> bool {
        self.data.mask == 0
    }

    #[inline(always)]
    pub fn eval(self, val:u32) -> bool {
        (self.data.isset ^ val) & self.data.mask == 0
    }
}


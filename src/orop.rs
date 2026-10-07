// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use crate::BaseOp;
use crate::OrOp;
use std::fmt::Debug;
use std::fmt::Display;

impl ops::Not for OrOp {
    type Output = OrOp;

    #[inline(always)]
    fn not(self) -> OrOp {
        OrOp { data: !self.data }
    }
}

impl ops::BitOrAssign<OrOp> for OrOp {
    #[inline(always)]
    fn bitor_assign(&mut self, rhs: Self) {
        *self = OrOp { data: self.data.combine(rhs.data) }
    }
}

impl ops::BitOr<OrOp> for OrOp {
    type Output = OrOp;

    #[inline(always)]
    fn bitor(self, other: OrOp) -> OrOp {
        OrOp { data: self.data.combine(other.data) }
    }
}

impl Debug for OrOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.data.isset)
         .field(&self.data.mask)
         .finish()
    }
}

impl Display for OrOp {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.data.isset)
         .field(&self.data.mask)
         .finish()
    }
}

impl OrOp {
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
        !(self.data.isset ^ val) & self.data.mask != 0
    }
}


// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use crate::{BaseOp, UnionOp};

impl PartialEq for UnionOp {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl std::hash::Hash for UnionOp {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        std::hash::Hash::hash(&self.data, state);
    }
}

impl ops::Not for UnionOp {
    type Output = Self;

    #[inline(always)]
    fn not(self) -> Self {
        Self { data: !self.data }
    }
}

impl std::fmt::Debug for UnionOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
            .field(&self.data.isset)
            .field(&self.data.mask)
            .finish()
    }
}

impl UnionOp {
    #[inline(always)]
    pub fn new(isset: u32, mask: u32) -> Self {
        Self { data: BaseOp { isset, mask } }
    }

    /// Match all masked bits (the previous AndOp interpretation).
    #[inline(always)]
    pub fn eval_and(self, val: u32) -> bool {
        (self.data.isset ^ val) & self.data.mask == 0
    }

    /// Match any inverted masked bit (the previous OrOp interpretation).
    #[inline(always)]
    pub fn eval_or(self, val: u32) -> bool {
        !(self.data.isset ^ val) & self.data.mask != 0
    }
}

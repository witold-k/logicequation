// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use core::iter;
use std::ops;
use crate::OpNodeRef;
use crate::OpNodeRefVec;

impl ops::Index<usize> for OpNodeRefVec {
    type Output = OpNodeRef;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodevec[..self.size][index]
    }
}

impl ops::IndexMut<usize> for OpNodeRefVec {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodevec[..self.size][index]
    }
}

impl std::fmt::Debug for OpNodeRefVec {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.nodevec)
         .finish()
    }
}

impl std::fmt::Display for OpNodeRefVec {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.nodevec)
         .finish()
    }
}

impl OpNodeRefVec {
    #[inline(always)]
    pub fn new0() -> OpNodeRefVec {
        let v: [OpNodeRef; 32] = [OpNodeRef { index: 0 }; 32];
        OpNodeRefVec { nodevec: v, size: 0 }
    }

    pub fn new1(noderef1: OpNodeRef) -> OpNodeRefVec {
        let mut v: [OpNodeRef; 32] = [OpNodeRef { index: 0 }; 32];
        v[0] = noderef1;
        OpNodeRefVec { nodevec: v, size: 1 }
    }

    pub fn new2(noderef1: OpNodeRef, noderef2: OpNodeRef) -> OpNodeRefVec {
        let mut v: [OpNodeRef; 32] = [OpNodeRef { index: 0 }; 32];
        v[0] = noderef1;
        v[1] = noderef2;
        OpNodeRefVec { nodevec: v, size: 2 }
    }

    pub fn new3(noderef1: OpNodeRef, noderef2: OpNodeRef, noderef3: OpNodeRef) -> OpNodeRefVec {
        let mut v: [OpNodeRef; 32] = [OpNodeRef { index: 0 }; 32];
        v[0] = noderef1;
        v[1] = noderef2;
        v[2] = noderef3;
        OpNodeRefVec { nodevec: v, size: 3 }
    }

    pub fn new4(
        noderef1: OpNodeRef, noderef2: OpNodeRef,
        noderef3: OpNodeRef, noderef4: OpNodeRef
    ) -> OpNodeRefVec {
        let mut v: [OpNodeRef; 32] = [OpNodeRef { index: 0 }; 32];
        v[0] = noderef1;
        v[1] = noderef2;
        v[2] = noderef3;
        v[3] = noderef4;
        OpNodeRefVec { nodevec: v, size: 4 }
    }

    #[inline(always)]
    pub fn iter(&self) -> impl iter::DoubleEndedIterator<Item = &OpNodeRef> {
        self.nodevec.iter().take(self.size)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.size
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    #[inline(always)]
    pub fn resize(&mut self, size: usize) {
        assert!(size <= self.nodevec.len(), "OpNodeRefVec capacity exceeded");
        // Newly exposed slots contain the default OpNodeRef (index 0).
        if size > self.size {
            self.nodevec[self.size..size].fill(OpNodeRef { index: 0 });
        }
        self.size = size;
    }

    #[inline(always)]
    pub fn push(&mut self, node: OpNodeRef) {
        assert!(self.size < self.nodevec.len(), "OpNodeRefVec capacity exceeded");
        self.nodevec[self.size] = node;
        self.size += 1;
    }

    /// Rotates the elements of the vector to the left by `rol` positions.
    /// If the vector has 1 or 0 elements, the rotation has no effect.
    /// The rotation is performed by first taking the elements from the position
    /// `size - rol` to the end, then appending the elements from the start to
    /// `size - rol`.
    pub fn rotate_left(&self, rol: usize) -> OpNodeRefVec {
        let size = self.len();
        if size <= 1 { return *self; }
        let rol = rol % size;
        let mut vec = OpNodeRefVec::new0();
        for node in self.iter().skip(rol) { vec.push(*node); }
        for node in self.iter().take(rol) { vec.push(*node); }
        vec
    }

    /// Rotates the elements to the right by `ror` positions.
    pub fn rotate_right(&self, ror: usize) -> OpNodeRefVec {
        let size = self.len();
        if size <= 1 { return *self; }
        let ror = ror % size;
        let mut vec = OpNodeRefVec::new0();
        for node in self.iter().skip(size - ror) { vec.push(*node); }
        for node in self.iter().take(size - ror) { vec.push(*node); }
        vec
    }

    /// Shifts the elements of the vector to the left by `shl` positions, filling
    /// the vacated positions with the `unset` value. The shift is performed by
    /// first pushing `shl` instances of `unset`, then appending the original
    /// elements up to the new size, which is the original size plus `shl`.
    /// If the new size exceeds 32, the excess elements are discarded.
    pub fn shift_left(&self, shl: usize, unset: OpNodeRef) -> OpNodeRefVec {
        let mut size = self.len();
        let shl = shl.min(32);
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();

        for _ in 0..shl {
            vec.push(unset);
        }
        let mut sum = size + shl;
        if sum > 32 {
            sum -= 32;
            size -= sum;
        }
        let src_iter = self.iter().take(size);
        for s in src_iter {
            vec.push(*s);
        }

        vec
    }

    /// Shifts the elements of the vector to the right by `shr` positions, filling
    /// the vacated positions with the `unset` value. The shift is performed by
    /// first appending the original elements from `shr` to the end, then pushing
    /// `shr` instances of `unset`.
    pub fn shift_right(&self, shr: usize, unset: OpNodeRef) -> OpNodeRefVec {
        let size = self.len();
        let shr = shr.min(32);
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();

        let src_iter = self.iter().take(size).skip(shr);
        for s in src_iter {
            vec.push(*s);
        }

        for _ in 0..shr {
            vec.push(unset);
        }

        vec
    }

}

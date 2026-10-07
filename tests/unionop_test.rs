// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::UnionOp;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[test]
fn test_unionop_evaluation_matches_and_or_interpretations() {
    let op = UnionOp::new(0b0101, 0b0111);
    for value in 0..16u32 {
        assert_eq!(op.eval_and(value), ((0b0101 ^ value) & 0b0111) == 0);
        assert_eq!(op.eval_or(value), (!(0b0101 ^ value) & 0b0111) != 0);
    }
}

#[test]
fn test_unionop_not_and_equality_hash() {
    let op = UnionOp::new(0b0101, 0b0111);
    let equivalent = UnionOp::new(0b1101, 0b0111);
    assert_eq!(op, equivalent);
    let mut first = DefaultHasher::new();
    let mut second = DefaultHasher::new();
    op.hash(&mut first);
    equivalent.hash(&mut second);
    assert_eq!(first.finish(), second.finish());
    let inverted = !op;
    assert_eq!(inverted.data.mask, op.data.mask);
    assert_eq!(inverted.data.isset, !op.data.isset);
    assert_eq!(!!op, op);
}

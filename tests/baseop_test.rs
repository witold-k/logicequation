// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_optype_fmt() {
        let t: OpType = OpType::AND;
        print!("OpType {}", t);
        print!("OpType {:?}", t);
    }

    #[test]
    fn test_optype_not() {
        assert_eq!(!OpType::AND, OpType::OR);
        assert_eq!(!OpType::OR, OpType::AND);
    }

    #[test]
    fn test_oppolarity_fmt() {
        let t: OpPolarity = OpPolarity::KEEP;
        print!("OpPolarity {}", t);
        print!("OpPolarity {:?}", t);
    }

    #[test]
    fn test_oppolarity_from() {
        assert_eq!(OpPolarity::from(true), OpPolarity::NOT);
        assert_eq!(OpPolarity::from(false), OpPolarity::KEEP);
    }

    #[test]
    fn test_oppolarity_not() {
        assert_eq!(!OpPolarity::KEEP, OpPolarity::NOT);
        assert_eq!(!OpPolarity::NOT, OpPolarity::KEEP);
    }

    #[test]
    fn test_baseop_not() {
        let bo = BaseOp { isset: 5, mask: 7 };
        let nbo = !bo;
        assert_eq!(nbo.isset, !bo.isset);
        assert_eq!(bo.mask,  nbo.mask);
    }

    #[test]
    fn test_baseop_combine_1() {
        let bo1 = BaseOp { isset: 5, mask: 7 };
        let bo2 = BaseOp { isset: 31, mask: 0 };
        let com1 = bo1.combine(bo2);
        let com2 = bo2.combine(bo1);
        assert!(!bo1.is_valid(bo2));
        assert_eq!(com1, com2);
        assert_eq!(com1, BaseOp { isset:5, mask:0 });
    }

    #[test]
    fn test_baseop_diff() {
        let bo = BaseOp { isset: 5,  mask: 7 };
        let diff = bo.diff(0x31);
        assert_eq!(diff, 4);
    }

    #[test]
    fn test_baseop_fmt() {
        let bo = BaseOp { isset: 5,  mask: 7 };
        print!("BaseOp {}", bo);
        print!("BaseOp {:?}", bo);
    }

    #[test]
    fn test_baseop_equality_considers_mask() {
        let a = BaseOp { isset: 0b0001, mask: 0b0001 };
        let b = BaseOp { isset: 0b0001, mask: 0b0011 };
        assert_ne!(a, b);

        // Bits outside the mask remain irrelevant.
        let c = BaseOp { isset: 0b0101, mask: 0b0001 };
        assert_eq!(a, c);

        // Different masks are different even when the masked value is zero.
        assert_ne!(
            BaseOp { isset: 0, mask: 0 },
            BaseOp { isset: 0, mask: 1 }
        );
    }

    #[test]
    fn test_baseop_hash_consistent_with_equality() {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        fn hash(op: BaseOp) -> u64 {
            let mut hasher = DefaultHasher::new();
            op.hash(&mut hasher);
            hasher.finish()
        }

        let a = BaseOp { isset: 0b0001, mask: 0b0001 };
        let equivalent = BaseOp { isset: 0b0101, mask: 0b0001 };
        assert_eq!(a, equivalent);
        assert_eq!(hash(a), hash(equivalent));

        let different_mask = BaseOp { isset: 0b0001, mask: 0b0011 };
        assert_ne!(a, different_mask);
        assert_ne!(hash(a), hash(different_mask));
    }
}

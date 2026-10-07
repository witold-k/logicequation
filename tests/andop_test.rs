// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

//use logicequation::baseop::BaseOp;
use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_andop_not() {
        let bo = AndOp::new(5, 7);
        let nbo = !bo;
        assert_eq!(nbo.data.isset, !bo.data.isset);
        assert_eq!(bo.data.mask,  nbo.data.mask);
    }

    #[test]
    fn test_andop_combine_1() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 0);
        let com1 = bo1 & bo2;
        let com2 = bo2 & bo1;
        assert_eq!(com1, com2);
        assert_eq!(bo1.data.is_valid(bo2.data), false);
        assert_eq!(com1, AndOp::new(7, 0));
        assert_eq!(com1.data.mask, 0);
        assert_eq!(com1.is_zero(), true);
        assert_eq!(com1.is_valid(), false);
    }

    #[test]
    fn test_andop_combine_2() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 2);
        let com1 = bo1 & bo2;
        let com2 = bo2 & bo1;
        assert_eq!(com1, com2);
        assert_eq!(com1, AndOp::new(7, 0));
        assert_eq!(com1.is_zero(), true);
        assert_eq!(com1.is_valid(), false);
    }

    #[test]
    fn test_andop_combine_3() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 5);
        let com1 = bo1 & bo2;
        let com2 = bo2 & bo1;
        assert_eq!(com1, com2);
        assert_eq!(com1, AndOp::new(5, 7));
        assert_eq!(com1.is_zero(), false);
        assert_eq!(com1.is_valid(), true);
    }

    #[test]
    fn test_andop_assign_combine_1() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 0);
        let mut com1 = bo1;
        com1 &= bo2;
        let mut com2 = bo2;
        com2 &= bo1;
        assert_eq!(com1, com2);
        assert_eq!(bo1.data.is_valid(bo2.data), false);
        assert_eq!(com1, AndOp::new(7, 0));
        assert_eq!(com1.data.mask, 0);
        assert_eq!(com1.is_zero(), true);
        assert_eq!(com1.is_valid(), false);
    }

    #[test]
    fn test_andop_assign_combine_2() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 2);
        let mut com1 = bo1;
        com1 &= bo2;
        let mut com2 = bo2;
        com2 &= bo1;
        assert_eq!(com1, com2);
        assert_eq!(com1, AndOp::new(7, 0));
        assert!(com1.is_zero());
        assert!(!com1.is_valid());
    }

    #[test]
    fn test_andop_assign_combine_3() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(31, 5);
        let mut com1 = bo1;
        com1 &= bo2;
        let mut com2 = bo2;
        com2 &= bo1;
        assert_eq!(com1, com2);
        assert_eq!(com1, AndOp::new(5, 7));
        assert!(!com1.is_zero());
        assert!(com1.is_valid());
    }


    #[test]
    fn test_andop_eq() {
        let bo1 = AndOp::new(5, 7);
        let bo2 = AndOp::new(0x45, 7);
        assert_eq!(bo1, bo2);
    }

    #[test]
    fn test_fmt() {
        let op = AndOp::new(5, 7);
        print!("AndOp {}", op);
        print!("AndOp {:?}", op);
    }
}


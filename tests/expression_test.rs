// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_expression_eval() {
        let mut expr = Expression::default();

        assert_eq!(expr.len(), 0);

        let noderef = expr.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let mut node = expr.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b and a; push andop
        node.push_op(0b01, 0b11); // b and not a; push andop
        drop(node);
        expr.push_node(noderef);

        assert_eq!(expr.len(), 1);

        let noderef = expr.create_node(OpType::AND, OpPolarity::KEEP, 0, 2);
        let mut node = expr.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b or a; push andop
        node.push_op(0b01, 0b11); // b ot not a; push andop
        drop(node);
        expr.push_node(noderef);

        assert_eq!(expr.len(), 2);

        assert_eq!(expr.eval(0).extract(), 0b10);
        assert_eq!(expr.eval(1).extract(), 0b01);
        assert_eq!(expr.eval(2).extract(), 0b01);
        assert_eq!(expr.eval(3).extract(), 0b10);
    }

    #[test]
    fn test_expression_not() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b and a; push andop
        node.push_op(0b01, 0b11); // b and not a; push andop
        drop(node);
        expra.push_node(noderef);

        let expr = expra.not();

        // for easier debugging first masked
        assert_eq!(expr.eval(0).extract(), 0b1);
        assert_eq!(expr.eval(1).extract(), 0b0);
        assert_eq!(expr.eval(2).extract(), 0b0);
        assert_eq!(expr.eval(3).extract(), 0b1);
    }

    #[test]
    fn test_expression_not_shl() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b and a; push andop
        node.push_op(0b01, 0b11); // b and not a; push andop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let expr = exprb << 1; //;| expra;

        assert_eq!(expr.eval(0).extract(), 0b10);
        assert_eq!(expr.eval(1).extract(), 0b00);
        assert_eq!(expr.eval(2).extract(), 0b00);
        assert_eq!(expr.eval(3).extract(), 0b10);
    }

    #[test]
    fn test_expression_not_shl_or() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b and a; push andop
        node.push_op(0b01, 0b11); // b and not a; push andop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let mut exprc = exprb.clone() << 1;
        let expr_or1 = exprc.or(&expra);
        let expr_or2 = exprc.clone() | expra.clone();

        assert_eq!(expr_or1.state, ExpressionState::OK);
        assert_eq!(expr_or2.state, ExpressionState::OK);

        assert_eq!(expra.eval(0).extract(), 0b0);
        assert_eq!(expra.eval(1).extract(), 0b1);
        assert_eq!(expra.eval(2).extract(), 0b1);
        assert_eq!(expra.eval(3).extract(), 0b0);

        assert_eq!(exprb.eval(0).extract(), 0b1);
        assert_eq!(exprb.eval(1).extract(), 0b0);
        assert_eq!(exprb.eval(2).extract(), 0b0);
        assert_eq!(exprb.eval(3).extract(), 0b1);

        assert_eq!(exprc.eval(0).extract(), 0b10);
        assert_eq!(exprc.eval(1).extract(), 0b00);
        assert_eq!(exprc.eval(2).extract(), 0b00);
        assert_eq!(exprc.eval(3).extract(), 0b10);

        assert_eq!(expr_or1.eval(0).extract(), 0b10);
        assert_eq!(expr_or1.eval(1).extract(), 0b01);
        assert_eq!(expr_or1.eval(2).extract(), 0b01);
        assert_eq!(expr_or1.eval(3).extract(), 0b10);

        assert_eq!(expr_or2.eval(0).extract(), 0b10);
        assert_eq!(expr_or2.eval(1).extract(), 0b01);
        assert_eq!(expr_or2.eval(2).extract(), 0b01);
        assert_eq!(expr_or2.eval(3).extract(), 0b10);
    }

    #[test]
    fn test_expression_not_shl_nor() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b and a; push andop
        node.push_op(0b01, 0b11); // b and not a; push andop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let mut exprc = exprb.clone() << 1;
        let expr_nor1 = exprc.nor(&expra);
        let expr_nor2 = (exprc.clone() | expra.clone()).not();

        assert_eq!(expr_nor1.state, ExpressionState::OK);
        assert_eq!(expr_nor2.state, ExpressionState::OK);

        assert_eq!(expra.eval(0).extract(), 0b0);
        assert_eq!(expra.eval(1).extract(), 0b1);
        assert_eq!(expra.eval(2).extract(), 0b1);
        assert_eq!(expra.eval(3).extract(), 0b0);

        assert_eq!(exprb.eval(0).extract(), 0b1);
        assert_eq!(exprb.eval(1).extract(), 0b0);
        assert_eq!(exprb.eval(2).extract(), 0b0);
        assert_eq!(exprb.eval(3).extract(), 0b1);

        assert_eq!(exprc.eval(0).extract(), 0b10);
        assert_eq!(exprc.eval(1).extract(), 0b00);
        assert_eq!(exprc.eval(2).extract(), 0b00);
        assert_eq!(exprc.eval(3).extract(), 0b10);

        assert_eq!(expr_nor1.eval(0).extract(), 0b01);
        assert_eq!(expr_nor1.eval(1).extract(), 0b10);
        assert_eq!(expr_nor1.eval(2).extract(), 0b10);
        assert_eq!(expr_nor1.eval(3).extract(), 0b01);

        assert_eq!(expr_nor2.eval(0).extract(), 0b01);
        assert_eq!(expr_nor2.eval(1).extract(), 0b10);
        assert_eq!(expr_nor2.eval(2).extract(), 0b10);
        assert_eq!(expr_nor2.eval(3).extract(), 0b01);
    }

    #[test]
    fn test_expression_not_shl_and() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::AND, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b or a; push orop
        node.push_op(0b01, 0b11); // b or not a; push orop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let mut exprc = exprb.clone() << 1;
        let expr_and1 = exprc.and(&expra);
        let expr_and2 = exprc.clone() & expra.clone();

        assert_eq!(expr_and1.state, ExpressionState::OK);
        assert_eq!(expr_and2.state, ExpressionState::OK);

        assert_eq!(expra.eval(0).extract(), 0b1);
        assert_eq!(expra.eval(1).extract(), 0b0);
        assert_eq!(expra.eval(2).extract(), 0b0);
        assert_eq!(expra.eval(3).extract(), 0b1);

        assert_eq!(exprb.eval(0).extract(), 0b0);
        assert_eq!(exprb.eval(1).extract(), 0b1);
        assert_eq!(exprb.eval(2).extract(), 0b1);
        assert_eq!(exprb.eval(3).extract(), 0b0);

        assert_eq!(exprc.eval(0).extract(), 0b00);
        assert_eq!(exprc.eval(1).extract(), 0b10);
        assert_eq!(exprc.eval(2).extract(), 0b10);
        assert_eq!(exprc.eval(3).extract(), 0b00);

        assert_eq!(expr_and1.eval(0).extract(), 0b01);
        assert_eq!(expr_and1.eval(1).extract(), 0b10);
        assert_eq!(expr_and1.eval(2).extract(), 0b10);
        assert_eq!(expr_and1.eval(3).extract(), 0b01);

        assert_eq!(expr_and2.eval(0).extract(), 0b01);
        assert_eq!(expr_and2.eval(1).extract(), 0b10);
        assert_eq!(expr_and2.eval(2).extract(), 0b10);
        assert_eq!(expr_and2.eval(3).extract(), 0b01);
    }

    #[test]
    fn test_expression_not_shl_nand() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::AND, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b or a; push orop
        node.push_op(0b01, 0b11); // b or not a; push orop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let mut exprc = exprb.clone() << 1;
        let expr_nand1 = exprc.nand(&expra);
        let expr_nand2 = (exprc.clone() & expra.clone()).not();
        let mut expr_nand3 = expr_nand2.rotate_left(1);
        let expr_nand4 = expr_nand3.nand(&expr_nand2);

        assert_eq!(expr_nand1.state, ExpressionState::OK);
        assert_eq!(expr_nand2.state, ExpressionState::OK);

        assert_eq!(expra.eval(0).extract(), 0b1);
        assert_eq!(expra.eval(1).extract(), 0b0);
        assert_eq!(expra.eval(2).extract(), 0b0);
        assert_eq!(expra.eval(3).extract(), 0b1);

        assert_eq!(exprb.eval(0).extract(), 0b0);
        assert_eq!(exprb.eval(1).extract(), 0b1);
        assert_eq!(exprb.eval(2).extract(), 0b1);
        assert_eq!(exprb.eval(3).extract(), 0b0);

        assert_eq!(exprc.eval(0).extract(), 0b00);
        assert_eq!(exprc.eval(1).extract(), 0b10);
        assert_eq!(exprc.eval(2).extract(), 0b10);
        assert_eq!(exprc.eval(3).extract(), 0b00);

        assert_eq!(expr_nand1.eval(0).extract(), 0b10);
        assert_eq!(expr_nand1.eval(1).extract(), 0b01);
        assert_eq!(expr_nand1.eval(2).extract(), 0b01);
        assert_eq!(expr_nand1.eval(3).extract(), 0b10);

        assert_eq!(expr_nand2.eval(0).extract(), 0b10);
        assert_eq!(expr_nand2.eval(1).extract(), 0b01);
        assert_eq!(expr_nand2.eval(2).extract(), 0b01);
        assert_eq!(expr_nand2.eval(3).extract(), 0b10);

        assert_eq!(expr_nand3.eval(0).extract(), 0b01);
        assert_eq!(expr_nand3.eval(1).extract(), 0b10);
        assert_eq!(expr_nand3.eval(2).extract(), 0b10);
        assert_eq!(expr_nand3.eval(3).extract(), 0b01);

        assert_eq!(expr_nand4.eval(0).extract(), 0b11);
        assert_eq!(expr_nand4.eval(1).extract(), 0b11);
        assert_eq!(expr_nand4.eval(2).extract(), 0b11);
        assert_eq!(expr_nand4.eval(3).extract(), 0b11);
    }

    #[test]
    fn test_expression_not_shl_xor() {
        let mut expra = Expression::default();

        let noderef = expra.create_node(OpType::AND, OpPolarity::KEEP, 0, 2);
        let mut node = expra.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // not b or a; push orop
        node.push_op(0b01, 0b11); // b or not a; push orop
        drop(node);
        expra.push_node(noderef);

        let exprb = expra.not();
        let mut exprc = exprb.clone() << 1;
        let expr_xor1 = exprc.xor(&expra);
        let expr_xor2 = exprc.clone() ^ expra.clone();

        assert_eq!(expr_xor1.state, ExpressionState::OK);
        assert_eq!(expr_xor2.state, ExpressionState::OK);

        assert_eq!(expra.eval(0).extract(), 0b1);
        assert_eq!(expra.eval(1).extract(), 0b0);
        assert_eq!(expra.eval(2).extract(), 0b0);
        assert_eq!(expra.eval(3).extract(), 0b1);

        assert_eq!(exprb.eval(0).extract(), 0b0);
        assert_eq!(exprb.eval(1).extract(), 0b1);
        assert_eq!(exprb.eval(2).extract(), 0b1);
        assert_eq!(exprb.eval(3).extract(), 0b0);

        assert_eq!(exprc.eval(0).extract(), 0b00);
        assert_eq!(exprc.eval(1).extract(), 0b10);
        assert_eq!(exprc.eval(2).extract(), 0b10);
        assert_eq!(exprc.eval(3).extract(), 0b00);

        assert_eq!(expr_xor1.eval(0).extract(), 0b01);
        assert_eq!(expr_xor1.eval(1).extract(), 0b10);
        assert_eq!(expr_xor1.eval(2).extract(), 0b10);
        assert_eq!(expr_xor1.eval(3).extract(), 0b01);

        assert_eq!(expr_xor2.eval(0).extract(), 0b01);
        assert_eq!(expr_xor2.eval(1).extract(), 0b10);
        assert_eq!(expr_xor2.eval(2).extract(), 0b10);
        assert_eq!(expr_xor2.eval(3).extract(), 0b01);
    }

    #[test]
    fn test_expression_add() {
        let expr = Expression::default();

        let expra = expr.new_input();
        let exprb = expr.new_input();
        let exprc = expra.clone().add(0, &exprb, 8, 8).unwrap();

        assert_eq!(exprc.eval((3 << 8) | 5).extract(), 8);
    }

    #[test]
    fn test_expression_add_carry_and_overflow() {
        let input = Expression::default();
        let mut a = input.new_input();
        let b = input.new_input();
        let sum = a.add(0, &b, 8, 8).unwrap();

        for (left, right) in [(0u32, 0u32), (1, 2), (7, 1), (255, 1), (127, 128)] {
            let value = left | (right << 8);
            assert_eq!(sum.eval(value).extract(), (left + right) & 0xff);
        }
    }

    #[test]
    fn test_expression_add_rejects_different_trees() {
        let mut a = Expression::default().new_input();
        let b = Expression::default().new_input();
        assert!(a.add(0, &b, 8, 8).is_err());
    }

    #[test]
    fn test_expression_sub_with_borrow_and_underflow() {
        let input = Expression::default();
        let mut a = input.new_input();
        let b = input.new_input();
        let difference = a.sub(0, &b, 8, 8).unwrap();

        for (left, right) in [
            (0u32, 0u32), (5, 3), (3, 5), (8, 1),
            (0, 1), (0, 255), (255, 255), (128, 127),
        ] {
            let value = left | (right << 8);
            assert_eq!(
                difference.eval(value).extract(),
                left.wrapping_sub(right) & 0xff,
                "incorrect subtraction for {left} - {right}"
            );
        }
    }

    #[test]
    fn test_expression_sub_full_width_and_zero_width() {
        let input = Expression::default();
        let mut a = input.new_input();
        let b = input.new_input();

        let full = a.sub(0, &b, 0, 32).unwrap();
        for value in [0u32, 1, 7, u32::MAX] {
            assert_eq!(full.eval(value).extract(), 0);
        }

        let empty = a.sub(32, &b, 32, 0).unwrap();
        assert_eq!(empty.len(), 0);
    }

    #[test]
    fn test_expression_sub_rejects_different_trees() {
        let mut a = Expression::default().new_input();
        let b = Expression::default().new_input();
        assert!(a.sub(0, &b, 8, 8).is_err());
    }

    #[test]
    #[should_panic(expected = "left operand range is out of bounds")]
    fn test_expression_sub_rejects_out_of_bounds_range() {
        let input = Expression::default();
        let mut a = input.new_input();
        let b = input.new_input();
        let _ = a.sub(30, &b, 0, 8);
    }
    #[test]
    fn test_mul_full_two_four_bit_operands() {
        let context = Expression::default();
        let input = context.new_input();
        let mut a = context.new_empty();
        let mut b = context.new_empty();
        for i in 0..4 {
            a.push_node(input[i]);
            b.push_node(input[i + 4]);
        }

        let (low, high) = a.mul_full(&b).unwrap();
        assert_eq!(low.len(), 32);
        assert_eq!(high.len(), 32);

        for x in 0u32..16 {
            for y in 0u32..16 {
                let input_value = x | (y << 4);
                let result = (low.eval(input_value).extract() as u64)
                    | ((high.eval(input_value).extract() as u64) << 32);
                assert_eq!(result, (x as u64) * (y as u64),
                    "incorrect product for {x} * {y}");
            }
        }
    }

    #[test]
    fn test_mul_full_zero_width() {
        let context = Expression::default();
        let mut empty = context.new_empty();
        let input = context.new_input();
        let (low, high) = empty.mul_full(&input).unwrap();
        assert_eq!(low.eval(u32::MAX).extract(), 0);
        assert_eq!(high.eval(u32::MAX).extract(), 0);
    }

    #[test]
    fn test_mul_full_rejects_different_trees() {
        let mut a = Expression::default().new_input();
        let b = Expression::default().new_input();
        assert!(a.mul_full(&b).is_err());
    }

    #[test]
    fn test_mul_full_carry_save_exhaustive_four_bits() {
        let context = Expression::default();
        let input = context.new_input();
        let mut a = context.new_empty();
        let mut b = context.new_empty();
        for bit in 0..4 {
            a.push_node(input[bit]);
            b.push_node(input[bit + 4]);
        }
        let (low, high) = a.mul_full_carry_save(&b).unwrap();
        assert_eq!(low.len(), 32);
        assert_eq!(high.len(), 32);

        for x in 0u32..16 {
            for y in 0u32..16 {
                let value = x | (y << 4);
                let actual = (low.eval(value).extract() as u64)
                    | ((high.eval(value).extract() as u64) << 32);
                assert_eq!(actual, (x as u64) * (y as u64), "{x} * {y}");
            }
        }
    }

    #[test]
    fn test_mul_full_empty_operand() {
        let context = Expression::default();
        let mut empty = context.new_empty();
        let input = context.new_input();
        let (low, high) = empty.mul_full_carry_save(&input).unwrap();
        assert_eq!(low.eval(u32::MAX).extract(), 0);
        assert_eq!(high.eval(u32::MAX).extract(), 0);
    }

    #[test]
    fn test_mul_full_different_trees() {
        let mut a = Expression::default().new_input();
        let b = Expression::default().new_input();
        assert!(a.mul_full_carry_save(&b).is_err());
    }

    #[test]
    fn test_div_rem_unsigned_four_bits() {
        let context = Expression::default();
        let input = context.new_input();
        let mut dividend = context.new_empty();
        let mut divisor = context.new_empty();
        for bit in 0..4 {
            dividend.push_node(input[bit]);
            divisor.push_node(input[bit + 4]);
        }
        let (q, r) = dividend.div_rem(&divisor).unwrap();
        for a in 0u32..16 {
            for b in 0u32..16 {
                let input_value = a | (b << 4);
                let expected_q = if b == 0 { u32::MAX } else { a / b };
                let expected_r = if b == 0 { a } else { a % b };
                assert_eq!(q.eval(input_value).extract(), expected_q, "{a} / {b}");
                assert_eq!(r.eval(input_value).extract(), expected_r, "{a} % {b}");
            }
        }
    }

    #[test]
    fn test_div_rem_rejects_different_trees() {
        let mut a = Expression::default().new_input();
        let b = Expression::default().new_input();
        assert!(a.div_rem(&b).is_err());
    }

    #[test]
    fn test_tree_not_same_stays_invalid_after_unary_operations() {
        let mut left = Expression::default().new_input();
        let right = Expression::default().new_input();

        let invalid = left.and(&right);
        assert_eq!(invalid.state, ExpressionState::TreeNotSame);
        assert_eq!(invalid.len(), 0);

        let mut for_not = invalid.clone();
        assert_eq!(for_not.not().state, ExpressionState::TreeNotSame);
        assert_eq!(invalid.rotate_left(1).state, ExpressionState::TreeNotSame);
        assert_eq!(invalid.rotate_right(1).state, ExpressionState::TreeNotSame);
        assert_eq!(invalid.shift_left(1).state, ExpressionState::TreeNotSame);
        assert_eq!(invalid.shift_right(1).state, ExpressionState::TreeNotSame);
    }


}

// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cell::{RefCell, RefMut};
use std::rc::Rc;
use std::ops;
use crate::BaseOp;
use crate::Expression;
use crate::ExpressionResult;
use crate::ExpressionState;
use crate::OpPolarity;
use crate::OpNode;
use crate::OpNodeRef;
use crate::OpNodeRefVec;
use crate::OpTree;
use crate::OpType;

impl std::fmt::Debug for ExpressionState {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(if ExpressionState::OK == *self { &"OK" } else { &"TreeNotSame" })
         .finish()
    }
}

impl std::fmt::Display for ExpressionState {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(if ExpressionState::OK == *self { &"OK" } else { &"TreeNotSame" })
         .finish()
    }
}

// ---------------------------------------------------------------------------

impl ops::Index<usize> for Expression {
    type Output = OpNodeRef;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.roots[index]
    }
}

impl ops::IndexMut<usize> for Expression {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.roots[index]
    }
}

impl ops::BitAnd for Expression {
    type Output = Self;

    #[inline(always)]
    fn bitand(mut self, rhs: Self) -> Self::Output {
        self.and(&rhs)
    }
}

impl ops::BitOr for Expression {
    type Output = Self;

    #[inline(always)]
    fn bitor(mut self, rhs: Self) -> Self::Output {
        self.or(&rhs)
    }
}

impl ops::BitXor for Expression {
    type Output = Self;

    #[inline(always)]
    fn bitxor(mut self, rhs: Self) -> Self::Output {
        self.xor(&rhs)
    }
}

impl ops::Shl<usize> for Expression {
    type Output = Self;

    #[inline(always)]
    fn shl(self, rhs: usize) -> Self::Output {
        self.shift_left(rhs)
    }
}

impl ops::Shr<usize> for Expression {
    type Output = Self;

    #[inline(always)]
    fn shr(self, lhs: usize) -> Self::Output {
        self.shift_right(lhs)
    }
}

impl std::fmt::Debug for Expression {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tree = Rc::clone(&self.tree);
        f.debug_tuple("")
         .field(&*tree)
         .field(&self.state)
         .finish()
    }
}

impl std::fmt::Display for Expression {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let tree = Rc::clone(&self.tree);
        f.debug_tuple("")
         .field(&*tree)
         .field(&self.state)
         .finish()
    }
}

impl Default for Expression {
    fn default() -> Self {
        let mut tree: OpTree = OpTree::with_capacity(32);
        let zero_node = tree.create_zero_node();
        let one_node = tree.create_one_node();
        let unset_node = tree.create_unset_node();
        let roots = OpNodeRefVec::new0();
        Self {
            tree:  Rc::from(RefCell::from(tree)),
            roots,
            zero:  zero_node,
            one:   one_node,
            unset: unset_node,
            state: ExpressionState::OK
        }
    }

}

impl Expression {
    pub fn new_empty(&self) -> Self {
        let roots = OpNodeRefVec::new0();
        Self {
            tree:  self.tree.clone(),
            roots,
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: ExpressionState::OK
        }
    }

    /// Creates a 32-bit view of the input value (least significant bit first).
    pub fn new_input(&self) -> Self {
        let mut roots = OpNodeRefVec::new0();
        let mut tree = self.tree.borrow_mut();
        for bit in 0..32 {
            let mask = 1u32 << bit;
            let node = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
            tree.get_node_mut(node).push_op(mask, mask);
            roots.push(node);
        }
        Self {
            tree: self.tree.clone(),
            roots,
            zero: self.zero,
            one: self.one,
            unset: self.unset,
            state: ExpressionState::OK
        }
    }

    #[inline(always)]
    pub fn valid(&self) -> bool {
        ExpressionState::OK == self.state
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.roots.len()
    }

    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.roots.is_empty()
    }

    #[inline(always)]
    pub fn push_node(&mut self, node: OpNodeRef) {
        self.roots.push(node);
    }

    #[inline(always)]
    pub fn same_tree(&self, other: &Expression) -> bool {
        Rc::ptr_eq(&self.tree, &other.tree)
    }

    #[inline(always)]
    pub fn create_node(
        &mut self, optype: OpType, oppolarity: OpPolarity,
        subnode_count: usize, subop_count: usize
    ) -> OpNodeRef {
        let mut tree = self.tree.borrow_mut();
        tree.create_node(optype, oppolarity, subnode_count, subop_count)
    }

    #[inline(always)]
    pub fn create_zero_node(&mut self) -> OpNodeRef {
        let mut tree = self.tree.borrow_mut();
        tree.create_zero_node()
    }

    #[inline(always)]
    pub fn get_node_mut(&mut self, node: OpNodeRef) -> RefMut<'_, OpNode> {
        RefMut::map(self.tree.borrow_mut(), |tree| &mut tree.nodes[node.index])
    }

    #[inline(always)]
    pub fn not(&mut self) -> Expression {
        let mut tree = self.tree.borrow_mut();
        let roots = tree.not_nodevec(&self.roots);
        Expression {
            tree:  Rc::clone(&self.tree),
            roots,
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: self.state
        }
    }

    #[inline(always)]
    pub fn and(&mut self, expr: &Expression) -> Expression {
        if self.same_tree(expr) {
            let mut tree = self.tree.borrow_mut();
            let roots = tree.and_nodevec(&self.roots, &expr.roots);
            Expression {
                tree:  Rc::clone(&self.tree),
                roots,
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::OK
            }
        }
        else {
            Expression {
                tree:  Rc::clone(&self.tree),
                roots: OpNodeRefVec::new0(),
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::TreeNotSame
            }
        }
    }

    #[inline(always)]
    pub fn nand(&mut self, expr: &Expression) -> Expression {
        if self.same_tree(expr) {
            let mut tree = self.tree.borrow_mut();
            let roots = tree.nand_nodevec(&self.roots, &expr.roots);
            Expression {
                tree:  Rc::clone(&self.tree),
                roots,
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::OK
            }
        }
        else {
            Expression {
                tree:  Rc::clone(&self.tree),
                roots: OpNodeRefVec::new0(),
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::TreeNotSame
            }
        }
    }

    #[inline(always)]
    pub fn or(&mut self, expr: &Expression) -> Expression {
        if self.same_tree(expr) {
            let mut tree = self.tree.borrow_mut();
            let roots = tree.or_nodevec(&self.roots, &expr.roots);
            Expression {
                tree:  Rc::clone(&self.tree),
                roots,
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::OK
            }
        }
        else {
            Expression {
                tree:  Rc::clone(&self.tree),
                roots: OpNodeRefVec::new0(),
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::TreeNotSame
            }
        }
    }

    #[inline(always)]
    pub fn nor(&mut self, expr: &Expression) -> Expression {
        if self.same_tree(expr) {
            let mut tree = self.tree.borrow_mut();
            let roots = tree.nor_nodevec(&self.roots, &expr.roots);
            Expression {
                tree:  Rc::clone(&self.tree),
                roots,
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::OK
            }
        }
        else {
            Expression {
                tree:  Rc::clone(&self.tree),
                roots: OpNodeRefVec::new0(),
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::TreeNotSame
            }
        }
    }

    #[inline(always)]
    pub fn xor(&mut self, expr: &Expression) -> Expression {
        if self.same_tree(expr) {
            let mut tree = self.tree.borrow_mut();
            let roots = tree.xor_nodevec(&self.roots, &expr.roots);
            Expression {
                tree:  Rc::clone(&self.tree),
                roots,
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::OK
            }
        }
        else {
            Expression {
                tree:  Rc::clone(&self.tree),
                roots: OpNodeRefVec::new0(),
                zero:  self.zero,
                one:   self.one,
                unset: self.unset,
                state: ExpressionState::TreeNotSame
            }
        }
    }

    #[inline(always)]
    pub fn rotate_left(&self, rol: usize) -> Expression {
        let tree_rc = Rc::clone(&self.tree);
        Expression {
            tree:  tree_rc,
            roots: self.roots.rotate_left(rol),
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: self.state
        }
    }

    #[inline(always)]
    pub fn rotate_right(&self, rol: usize) -> Expression {
        let tree_rc = Rc::clone(&self.tree);
        Expression {
            tree:  tree_rc,
            roots: self.roots.rotate_right(rol),
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: self.state
        }
    }

    #[inline(always)]
    pub fn shift_left(&self, shl: usize) -> Expression {
        let tree_rc = Rc::clone(&self.tree);
        Expression {
            tree:  tree_rc,
            roots: self.roots.shift_left(shl, self.unset),
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: self.state
        }
    }

    #[inline(always)]
    pub fn shift_right(&self, shr: usize) -> Expression {
        let tree_rc = Rc::clone(&self.tree);
        Expression {
            tree:  tree_rc,
            roots: self.roots.shift_right(shr, self.unset),
            zero:  self.zero,
            one:   self.one,
            unset: self.unset,
            state: self.state
        }
    }

    /* C: Carry
    Cin  A B A+B+Cin S Cout
     0   0 0    0    0  0
     0   0 1    1    1  0
     0   1 0    1    1  0
     0   1 1    2    0  1
     1   0 0    1    1  0
     1   0 1    2    0  1
     1   1 0    2    0  1
     1   1 1    3    1  1
    */
    /// Adds two bit ranges and returns the low `bitwidth` result bits.
    /// Input ranges are selected by `argshiftl` and `argshiftr`.
    pub fn add(&mut self, argshiftl: usize, expr: &Expression, argshiftr: usize, bitwidth: usize) -> ExpressionResult {
        if !self.same_tree(expr) {
            return Err(ExpressionState::TreeNotSame);
        }
        assert!(bitwidth <= 32, "addition result cannot exceed 32 bits");
        assert!(argshiftl <= self.len() && bitwidth <= self.len() - argshiftl,
                "left operand range is out of bounds");
        assert!(argshiftr <= expr.len() && bitwidth <= expr.len() - argshiftr,
                "right operand range is out of bounds");

        let mut ret = self.new_empty();
        let mut carry = self.zero;
        let mut tree = self.tree.borrow_mut();

        for i in 0..bitwidth {
            let a = self.roots[argshiftl + i];
            let b = expr.roots[argshiftr + i];
            let sum = tree.xor3_node(a, b, carry);
            ret.roots.push(sum);

            // Carry out = (a AND b) OR (carry AND (a XOR b)).
            let ab = tree.and_node(a, b);
            let axb = tree.xor_node(a, b);
            let cx = tree.and_node(carry, axb);
            carry = tree.or_node(ab, cx);
        }
        Ok(ret)
    }

    /// Subtracts two bit ranges and returns the low `bitwidth` result bits.
    /// Input ranges are selected by `argshiftl` and `argshiftr`.
    /// The final borrow is discarded (modulo 2^bitwidth).
    pub fn sub(&mut self, argshiftl: usize, expr: &Expression, argshiftr: usize, bitwidth: usize) -> ExpressionResult {
        if !self.same_tree(expr) {
            return Err(ExpressionState::TreeNotSame);
        }
        assert!(bitwidth <= 32, "subtraction result cannot exceed 32 bits");
        assert!(argshiftl <= self.len() && bitwidth <= self.len() - argshiftl,
                "left operand range is out of bounds");
        assert!(argshiftr <= expr.len() && bitwidth <= expr.len() - argshiftr,
                "right operand range is out of bounds");

        let mut ret = self.new_empty();
        let mut borrow = self.zero;
        let mut tree = self.tree.borrow_mut();

        for i in 0..bitwidth {
            let a = self.roots[argshiftl + i];
            let b = expr.roots[argshiftr + i];
            let axb = tree.xor_node(a, b);
            let difference = tree.xor_node(axb, borrow);
            ret.roots.push(difference);

            // Borrow out = (!a AND b) OR (borrow AND !(a XOR b)).
            let not_a = tree.not_node(a);
            let not_a_and_b = tree.and_node(not_a, b);
            let equal = tree.not_node(axb);
            let borrow_and_equal = tree.and_node(borrow, equal);
            borrow = tree.or_node(not_a_and_b, borrow_and_equal);
        }
        Ok(ret)
    }

    /// Builds an unsigned multiplier for two bit vectors (up to 32 bits each).
    /// Returns the full 64-bit product as (low 32 bits, high 32 bits).
    /// Both operands must belong to the same expression tree.
    ///
    /// The result is symbolic: partial products and carry logic are stored
    /// as nodes and evaluated only when an input value is supplied.
    pub fn mul_full(&mut self, expr: &Expression) -> Result<(Expression, Expression), ExpressionState> {
        if !self.same_tree(expr) {
            return Err(ExpressionState::TreeNotSame);
        }

        let mut bits = [self.zero; 64];
        let mut tree = self.tree.borrow_mut();

        // Schoolbook multiplication: add each shifted partial-product row.
        // The position immediately above a row is still zero, so its final
        // carry can be stored there without another adder stage.
        for j in 0..expr.len() {
            let mut carry = self.zero;
            for i in 0..self.len() {
                let pos = i + j;
                let partial = tree.and_node(self.roots[i], expr.roots[j]);
                let old = bits[pos];
                let old_xor_partial = tree.xor_node(old, partial);
                bits[pos] = tree.xor_node(old_xor_partial, carry);

                let both = tree.and_node(old, partial);
                let carry_xor = tree.and_node(carry, old_xor_partial);
                carry = tree.or_node(both, carry_xor);
            }
            if !self.is_empty() {
                bits[self.len() + j] = carry;
            }
        }
        drop(tree);

        let mut low = self.new_empty();
        let mut high = self.new_empty();
        for bit in bits.iter().take(32) {
            low.roots.push(*bit);
        }
        for bit in bits.iter().skip(32) {
            high.roots.push(*bit);
        }
        Ok((low, high))
    }

    /// Builds an unsigned, full-width multiplier using carry-save reduction.
    ///
    /// The 64-bit result is returned as (low, high), each 32 bits wide.
    /// Both operands must share an OpTree and may contain up to 32 bits.
    /// Each reduction round contains independent full adders; only the final
    /// carry-propagate addition is serial. No hardware registers are inserted.
    pub fn mul_full_carry_save(&mut self, expr: &Expression) -> Result<(Expression, Expression), ExpressionState> {
        if !self.same_tree(expr) {
            return Err(ExpressionState::TreeNotSame);
        }

        let mut tree = self.tree.borrow_mut();
        let zero = self.zero;
        let mut columns: Vec<Vec<OpNodeRef>> = vec![Vec::new(); 64];

        // Partial products: column i+j receives a[i] AND b[j].
        for i in 0..self.len() {
            for j in 0..expr.len() {
                columns[i + j].push(tree.and_node(self.roots[i], expr.roots[j]));
            }
        }

        // Wallace-style carry-save stages: reduce every triple (x,y,z)
        // into sum=x XOR y XOR z and carry=majority(x,y,z), shifted
        // one column left. Carries join the *next* stage, not this one.
        while columns.iter().any(|column| column.len() > 2) {
            let mut next: Vec<Vec<OpNodeRef>> = vec![Vec::new(); 64];
            for (bit, column) in columns.iter().enumerate() {
                let mut chunks = column.chunks_exact(3);
                for triple in &mut chunks {
                    let x = triple[0];
                    let y = triple[1];
                    let z = triple[2];
                    let xy = tree.xor_node(x, y);
                    let sum = tree.xor_node(xy, z);
                    next[bit].push(sum);

                    if bit + 1 < 64 {
                        let xy_and = tree.and_node(x, y);
                        let z_and_xy = tree.and_node(z, xy);
                        let carry = tree.or_node(xy_and, z_and_xy);
                        next[bit + 1].push(carry);
                    }
                }
                next[bit].extend_from_slice(chunks.remainder());
            }
            columns = next;
        }

        // One final carry-propagate adder for the two remaining rows.
        let mut result = [zero; 64];
        let mut carry = zero;
        for (bit, column) in columns.iter().enumerate() {
            let x = column.first().copied().unwrap_or(zero);
            let y = column.get(1).copied().unwrap_or(zero);
            let xy = tree.xor_node(x, y);
            result[bit] = tree.xor_node(xy, carry);

            if bit + 1 < 64 {
                let xy_and = tree.and_node(x, y);
                let carry_and_xy = tree.and_node(carry, xy);
                carry = tree.or_node(xy_and, carry_and_xy);
            }
        }
        drop(tree);

        let mut low = self.new_empty();
        let mut high = self.new_empty();
        for node in result.iter().take(32) {
            low.roots.push(*node);
        }
        for node in result.iter().skip(32) {
            high.roots.push(*node);
        }
        Ok((low, high))
    }

    /// Unsigned restoring division over symbolic bit vectors.
    ///
    /// Returns (quotient, remainder), each as a 32-bit Expression.
    /// The divisor is zero-extended to the working width. Division by zero
    /// produces an all-ones quotient and the unchanged dividend.
    pub fn div_rem(&mut self, divisor: &Expression) -> Result<(Expression, Expression), ExpressionState> {
        if !self.same_tree(divisor) {
            return Err(ExpressionState::TreeNotSame);
        }

        let zero = self.zero;
        let width = self.len().max(divisor.len()) + 1;
        let mut tree = self.tree.borrow_mut();
        let mut remainder = vec![zero; width];
        let mut quotient = [zero; 32];

        // Unrolled restoring division. The extra high bit ensures that
        // comparisons remain correct even when the shifted remainder grows.
        for position in (0..self.len()).rev() {
            let mut shifted = vec![zero; width];
            shifted[0] = self.roots[position];
            shifted[1..width].copy_from_slice(&remainder[..width - 1]);

            // Compute shifted - divisor and its final unsigned borrow.
            let mut difference = Vec::with_capacity(width);
            let mut borrow = zero;
            for (bit, &a) in shifted.iter().enumerate() {
                let b = if bit < divisor.len() { divisor.roots[bit] } else { zero };
                let axb = tree.xor_node(a, b);
                difference.push(tree.xor_node(axb, borrow));

                let not_a = tree.not_node(a);
                let first = tree.and_node(not_a, b);
                let equal = tree.not_node(axb);
                let second = tree.and_node(borrow, equal);
                borrow = tree.or_node(first, second);
            }

            let can_subtract = tree.not_node(borrow);
            quotient[position] = can_subtract;
            let not_subtract = borrow;
            for bit in 0..width {
                let yes = tree.and_node(can_subtract, difference[bit]);
                let no = tree.and_node(not_subtract, shifted[bit]);
                remainder[bit] = tree.or_node(yes, no);
            }
        }

        // Define divide-by-zero behavior without requiring a runtime error
        // for a symbolic divisor.
        let mut divisor_nonzero = zero;
        for bit in 0..divisor.len() {
            divisor_nonzero = tree.or_node(divisor_nonzero, divisor.roots[bit]);
        }
        let divisor_zero = tree.not_node(divisor_nonzero);
        let mut quotient_result = [zero; 32];
        let mut remainder_result = [zero; 32];
        for bit in 0..32 {
            quotient_result[bit] = tree.or_node(divisor_zero, quotient[bit]);
            let rem_bit = if bit < remainder.len() { remainder[bit] } else { zero };
            let original = if bit < self.len() { self.roots[bit] } else { zero };
            let normal_rem = tree.and_node(divisor_nonzero, rem_bit);
            let zero_rem = tree.and_node(divisor_zero, original);
            remainder_result[bit] = tree.or_node(normal_rem, zero_rem);
        }
        drop(tree);

        let mut q = self.new_empty();
        let mut r = self.new_empty();
        for bit in 0..32 {
            q.roots.push(quotient_result[bit]);
            r.roots.push(remainder_result[bit]);
        }
        Ok((q, r))
    }

    #[inline(always)]
    pub fn eval(&self, val: u32) -> BaseOp {
        let tree = self.tree.borrow();
        (*tree).eval_vec(&self.roots, val)
    }

}

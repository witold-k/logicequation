// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::ops;
use std::vec::Vec;
use std::default::Default;
use crate::UnionOp;
use crate::OpNode;
use crate::OpNodeRef;
use crate::OpType;
use crate::OpPolarity;

/// Represents a logical operation node in an expression tree.
/// An OpNode contains an operation type (`optype`), a polarity (`oppolarity`),
/// and vectors for subnodes and suboperations (`subnodes` and `subops`).
///
/// ### Mathematical Principle
/// The `OpNode` structure uses bitwise operations to evaluate logical expressions.
/// The `optype` determines the logical operation (AND, OR, UNSET), and `oppolarity`
/// determines whether the result should be inverted.
///
/// ### Evaluation Logic (`eval`)
/// The evaluation logic in `eval_plain` method:
/// - For `AND` operations, it checks if all suboperations are true.
/// - For `OR` operations, it checks if any suboperation is true.
/// - `UNSET` represents an absent operand; when evaluated as a bit it is false.
///
/// The `eval` method applies the polarity to the result of `eval_plain`.
///
/// ### Invariants
/// - `subops` must be preallocated before adding operations.
/// - `subnodes` stores indexes of subnodes, which are managed by the containing `OpTree`.
impl Default for OpNode {
    #[inline(always)]
    fn default() -> Self {
        OpNode {
            optype:     OpType::AND,
            oppolarity: OpPolarity::KEEP,
            subnodes:   Vec::new(),
            subops:     Vec::new()
        }
    }
}

impl ops::Not for OpNode {
    type Output = OpNode;

    // Negate the result, not the operands: this also works for subnodes
    // and for nodes whose polarity is already NOT.
    fn not(mut self) -> OpNode {
        if !self.is_unset() {
            self.oppolarity = !self.oppolarity;
        }
        self
    }
}

impl std::fmt::Debug for OpNode {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.optype)
         .field(&self.oppolarity)
         .field(&self.subnodes)
         .field(&self.subops)
         .finish()
    }
}

impl std::fmt::Display for OpNode {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.optype)
         .field(&self.oppolarity)
         .field(&self.subnodes)
         .field(&self.subops)
         .finish()
    }
}

impl OpNode {
    #[inline(always)]
    pub fn new_keep(
        optype:        OpType,
        subnode_count: usize,
        subop_count:   usize
    ) -> Self {
        Self {
            optype,
            oppolarity: OpPolarity::KEEP,
            subnodes:   Vec::with_capacity(subnode_count),
            subops:     Vec::with_capacity(subop_count)
        }
    }

    #[inline(always)]
    pub fn new(
        optype:        OpType,
        oppolarity:    OpPolarity,
        subnode_count: usize,
        subop_count:   usize
    ) -> Self {
        Self {
            optype,
            oppolarity,
            subnodes:   Vec::with_capacity(subnode_count),
            subops:     Vec::with_capacity(subop_count)
        }
    }

    #[inline(always)]
    pub fn new_zero() -> Self {
        Self {
            optype:     OpType::OR,
            oppolarity: OpPolarity::KEEP,
            subnodes:   Vec::new(),
            subops:     Vec::new()
        }
    }

    #[inline(always)]
    pub fn new_one() -> Self {
        Self {
            optype:     OpType::AND,
            oppolarity: OpPolarity::KEEP,
            subnodes:   Vec::new(),
            subops:     Vec::new()
        }
    }

    #[inline(always)]
    pub fn new_unset() -> Self {
        Self {
            optype:     OpType::UNSET,
            oppolarity: OpPolarity::KEEP,
            subnodes:   Vec::new(),
            subops:     Vec::new()
        }
    }

    #[inline(always)]
    pub fn has_ops(&self) -> bool {
        !self.subops.is_empty()
    }

    #[inline(always)]
    pub fn is_unset(&self) -> bool {
        self.optype == OpType::UNSET
    }

    /// Evaluated the node, that means used val as argument
    /// to calculate the logic expresstion that is represented
    /// of this node.
    /// important: only subops are calculated
    /// subnodes can not be calculated, since subnodes
    /// is a vector that contains node indexes. These are
    /// not known by OpNode. These are known by OpTree
    /// which is the container for all node.
    /// As a consequence OpTree evaluates opnodes of OpNode.
    #[inline(always)]
    fn eval_plain(&self, val: u32) -> bool {
        if OpType::AND == self.optype {
            // OpType::AND means that all
            // all contents of self.subops are
            // accumulated with AND
            // that means also that the parts itself should be
            // called via OR, since if that could be called via AND
            // the whole operation could be simplified to one AND
            for part in self.subops.iter() {
                if !part.eval_or(val) { return false; }
            }
            true
        }
        else if OpType::OR == self.optype { // OpType::OR od OpType::UNSET
            // when unset this function will never be called
            //
            // OpType::OR means that all
            // all contents of self.subops are
            // accumulated with OR
            // that means also that the parts itself should be
            // called via AND, since if that could be called via OR
            // the whole operation could be simplified to one OR
            for part in self.subops.iter() {
                if part.eval_and(val) { return true; }
            }
            false
        }
        else {
            false
        }
    }

    pub fn eval(&self, val:u32) -> bool {
        if OpPolarity::KEEP == self.oppolarity {
            self.eval_plain(val)
        }
        else {
            !self.eval_plain(val)
        }
    }

    #[inline(always)]
    pub fn push_op(&mut self, isset: u32, mask: u32) {
        self.subops.push(UnionOp::new(isset, mask));
    }

    #[inline(always)]
    pub fn push_node(&mut self, node: OpNodeRef) {
        self.subnodes.push(node.index);
    }
}

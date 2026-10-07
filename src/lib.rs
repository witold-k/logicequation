// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

pub mod andop;
pub mod baseop;
pub mod orop;
pub mod unionop;
pub mod opnode;
pub mod opnoderef;
pub mod opnoderefvec;
pub mod optree;
pub mod expression;

use std::cell::RefCell;
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::hash::Hash;
use std::rc::Rc;

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum OpType {
    AND       = 0,
    OR        = 1,
    UNSET     = 2
}

#[derive(Copy, Clone, PartialEq, Hash, Debug)]
pub enum OpPolarity {
    KEEP = 0,
    NOT  = 1
}

#[derive(Copy, Clone)]
pub struct BaseOp {
    pub isset: u32,
    pub mask:  u32
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct AndOp {
    pub data: BaseOp
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct OrOp {
    pub data: BaseOp
}

#[derive(Copy, Clone)]
pub struct UnionOp {
    pub data: BaseOp
}

#[derive(Clone, PartialEq, Hash)]
pub struct OpNode {
    pub optype:     OpType,
    pub oppolarity: OpPolarity,
    pub subnodes:   Vec<usize>,
    pub subops:     Vec<UnionOp>
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct ValueVec {
    pub valuevec: [u32; 32],
    pub size:     usize
}


#[derive(Copy, Clone, PartialEq, Hash)]
pub struct OpNodeRef {
    pub index: usize
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub struct OpNodeRefVec {
    nodevec: [OpNodeRef; 32],
    size:    usize
}

#[derive(Clone, PartialEq, Hash)]
pub struct OpTree {
    pub nodes: Vec<OpNode>
}

#[derive(Copy, Clone, PartialEq, Hash)]
pub enum ExpressionState {
    OK,
    TreeNotSame
}

pub type ExpressionResult = Result<Expression, ExpressionState>;

#[derive(Clone)]
pub struct Expression {
    /** hint for beginners:
     * Shared references in Rust disallow mutation by default,
     * and Arc is no exception:
     * you cannot generally obtain a mutable reference to something inside an Arc.
     * If you need to mutate through an Arc,
     * use Mutex, RwLock, or one of the Atomic types.
     */
    pub tree:  Rc<RefCell<OpTree>>,
    pub roots: OpNodeRefVec,
    pub zero:  OpNodeRef,
    pub one:   OpNodeRef,
    pub unset: OpNodeRef,
    pub state: ExpressionState
}


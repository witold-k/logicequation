// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use std::cmp::min;
use std::ops;
use std::vec::Vec;
use std::default::Default;
use crate::*;

impl Default for OpTree {
    #[inline(always)]
    fn default() -> Self {
        OpTree::new()
    }
}

impl ops::Index<usize> for OpTree {
    type Output = OpNode;

    #[inline(always)]
    fn index(&self, index: usize) -> &Self::Output {
        &self.nodes[index]
    }
}

impl ops::IndexMut<usize> for OpTree {
    #[inline(always)]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.nodes[index]
    }
}

impl ops::Not for OpTree {
    type Output = OpTree;

    fn not(self) -> OpTree {
        let mut cloned_nodes = self.nodes.clone();
        for op in cloned_nodes.iter_mut() {
            *op = !op.clone();
        }
        OpTree {
            nodes: cloned_nodes
        }
    }
}

impl std::fmt::Debug for OpTree {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.nodes)
         .finish()
    }
}

impl std::fmt::Display for OpTree {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.nodes)
         .finish()
    }
}

impl OpTree {
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            nodes: Vec::new()
        }
    }

    #[inline(always)]
    pub fn with_capacity(node_count: usize) -> Self {
        Self {
            nodes: Vec::with_capacity(node_count)
        }
    }

    pub fn create_node(
        &mut self, optype: OpType, oppolarity: OpPolarity,
        subnode_count: usize, subop_count: usize
    ) -> OpNodeRef {
        let opnode: OpNode = OpNode::new(optype, oppolarity, subnode_count, subop_count);
        self.nodes.push(opnode);
        OpNodeRef{ index: self.nodes.len() - 1 }
    }

    #[inline(always)]
    pub fn create_zero_node(&mut self) -> OpNodeRef {
        self.nodes.push(OpNode::new_zero());
        OpNodeRef{ index: self.nodes.len() - 1 }
    }

    #[inline(always)]
    pub fn create_one_node(&mut self) -> OpNodeRef {
        self.nodes.push(OpNode::new_one());
        OpNodeRef{ index: self.nodes.len() - 1 }
    }

    #[inline(always)]
    pub fn create_unset_node(&mut self) -> OpNodeRef {
        self.nodes.push(OpNode::new_unset());
        OpNodeRef{ index: self.nodes.len() - 1 }
    }

    #[inline(always)]
    pub fn is_node_unset(&self, node: OpNodeRef) -> bool {
        self[node.index].is_unset()
    }

    // -----------------------------------------------------------------------
    // -------- SINGLE NODE OPERATIONS ---------------------------------------
    // -----------------------------------------------------------------------

    /// Negates the logic of a single node within the tree.
    /// If the node is unset, it returns the node itself.
    /// Otherwise, it creates a new AND node with NOT polarity,
    /// linking the original node as a subnode.
    pub fn not_node(&mut self, node: OpNodeRef) -> OpNodeRef {
        if self.is_node_unset(node) {
            node
        }
        else {
            let noderef:OpNodeRef = self.create_node(
                OpType::AND, OpPolarity::NOT, 1, 0
            );
            let new_node = &mut self[noderef.index];
            new_node.subnodes.push(node.index);
            noderef
        }
    }

    /// Performs a logical AND operation between two nodes.
    /// If either node is unset, it returns the other node.
    /// Otherwise, it creates a new AND node, linking both nodes as subnodes.
    pub fn and_node(&mut self, lnode: OpNodeRef, rnode: OpNodeRef) -> OpNodeRef {
        let lunset = self.is_node_unset(lnode);
        let runset = self.is_node_unset(rnode);
        if lunset { return rnode; } else if runset { return lnode; }

        let noderef:OpNodeRef = self.create_node(
            OpType::AND, OpPolarity::KEEP, 2, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(lnode.index);
        node.subnodes.push(rnode.index);
        noderef
    }

    /// Performs a logical AND operation between three nodes.
    /// If any node is unset, it returns the logical AND of the remaining nodes.
    /// Otherwise, it creates a new AND node, linking all three nodes as subnodes.
    pub fn and3_node(&mut self, node1: OpNodeRef, node2: OpNodeRef, node3: OpNodeRef) -> OpNodeRef {
        let unset1 = self.is_node_unset(node1);
        let unset2 = self.is_node_unset(node2);
        let unset3 = self.is_node_unset(node3);
        if unset1 {
            if unset2 {
                return node3;
            }
            else {
                return self.and_node(node2, node3);
            }
        }
        else if unset2 {
                return self.and_node(node1, node3);
            }
            else if unset3 {
                    return self.and_node(node1, node2);
                }
                else {
                    // continue with normal calculations
                }

        let noderef:OpNodeRef = self.create_node(
            OpType::AND, OpPolarity::KEEP, 3, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(node1.index);
        node.subnodes.push(node2.index);
        node.subnodes.push(node3.index);
        noderef
    }

    /// Performs a logical NAND operation between two nodes.
    /// If either node is unset, it negates the other node.
    /// Otherwise, it creates a new AND node with NOT polarity,
    /// linking both nodes as subnodes.
    pub fn nand_node(&mut self, lnode: OpNodeRef, rnode: OpNodeRef) -> OpNodeRef {
        let lunset = self.is_node_unset(lnode);
        let runset = self.is_node_unset(rnode);
        if lunset { return self.not_node(rnode); } else if runset { return self.not_node(lnode); }

        let noderef:OpNodeRef = self.create_node(
            OpType::AND, OpPolarity::NOT, 2, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(lnode.index);
        node.subnodes.push(rnode.index);
        noderef
    }

    /// Performs a logical NAND operation between three nodes.
    /// If any node is unset, it negates the logical AND of the remaining nodes.
    /// Otherwise, it creates a new AND node with NOT polarity,
    /// linking all three nodes as subnodes.
    pub fn nand3_node(&mut self, node1: OpNodeRef, node2: OpNodeRef, node3: OpNodeRef) -> OpNodeRef {
        let unset1 = self.is_node_unset(node1);
        let unset2 = self.is_node_unset(node2);
        let unset3 = self.is_node_unset(node3);
        if unset1 {
            if unset2 {
                return self.not_node(node3);
            }
            else {
                return self.nand_node(node2, node3);
            }
        }
        else {
            if unset2 {
                return self.nand_node(node1, node3);
            }
            else {
                if unset3 {
                    return self.nand_node(node1, node2);
                }
                else {
                    // continue with normal calculations
                }
            }
        }

        let noderef:OpNodeRef = self.create_node(
            OpType::AND, OpPolarity::NOT, 3, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(node1.index);
        node.subnodes.push(node2.index);
        node.subnodes.push(node3.index);
        noderef
    }

    /// Performs a logical OR operation between two nodes.
    /// If either node is unset, it returns the other node.
    /// Otherwise, it creates a new OR node, linking both nodes as subnodes.
    pub fn or_node(&mut self, lnode: OpNodeRef, rnode: OpNodeRef) -> OpNodeRef {
        let lunset = self.is_node_unset(lnode);
        let runset = self.is_node_unset(rnode);
        if lunset { return rnode; } else if runset { return lnode; }

        let noderef:OpNodeRef = self.create_node(
            OpType::OR, OpPolarity::KEEP, 2, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(lnode.index);
        node.subnodes.push(rnode.index);
        noderef
    }

    /// Performs a logical OR operation between three nodes.
    /// If any node is unset, it returns the logical OR of the remaining nodes.
    /// Otherwise, it creates a new OR node, linking all three nodes as subnodes.
    pub fn or3_node(&mut self, node1: OpNodeRef, node2: OpNodeRef, node3: OpNodeRef) -> OpNodeRef {
        let unset1 = self.is_node_unset(node1);
        let unset2 = self.is_node_unset(node2);
        let unset3 = self.is_node_unset(node3);
        if unset1 {
            if unset2 {
                return node3;
            }
            else {
                return self.or_node(node2, node3);
            }
        }
        else {
            if unset2 {
                return self.or_node(node1, node3);
            }
            else {
                if unset3 {
                    return self.or_node(node1, node2);
                }
                else {
                    // continue with normal calculations
                }
            }
        }

        let noderef:OpNodeRef = self.create_node(
            OpType::OR, OpPolarity::KEEP, 3, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(node1.index);
        node.subnodes.push(node2.index);
        node.subnodes.push(node3.index);
        noderef
    }

    /// Performs a logical NOR operation between two nodes.
    /// If either node is unset, it negates the other node.
    /// Otherwise, it creates a new OR node with NOT polarity,
    /// linking both nodes as subnodes.
    pub fn nor_node(&mut self, lnode: OpNodeRef, rnode: OpNodeRef) -> OpNodeRef {
        let lunset = self.is_node_unset(lnode);
        let runset = self.is_node_unset(rnode);
        if lunset { return self.not_node(rnode); } else if runset { return self.not_node(lnode); }

        let noderef:OpNodeRef = self.create_node(
            OpType::OR, OpPolarity::NOT, 2, 0
        );
        let node = &mut self[noderef.index];
        if !lunset { node.subnodes.push(lnode.index); }
        if !runset { node.subnodes.push(rnode.index); }
        noderef
    }

    /// Performs a logical NOR operation between three nodes.
    /// If any node is unset, it negates the logical OR of the remaining nodes.
    /// Otherwise, it creates a new OR node with NOT polarity,
    /// linking all three nodes as subnodes.
    pub fn nor3_node(&mut self, node1: OpNodeRef, node2: OpNodeRef, node3: OpNodeRef) -> OpNodeRef {
        let unset1 = self.is_node_unset(node1);
        let unset2 = self.is_node_unset(node2);
        let unset3 = self.is_node_unset(node3);
        if unset1 {
            if unset2 {
                return self.not_node(node3);
            }
            else {
                return self.nor_node(node2, node3);
            }
        }
        else if unset2 {
                return self.nor_node(node1, node3);
            }
            else if unset3 {
                    return self.nor_node(node1, node2);
                }
                else {
                    // continue with normal calculations
                }

        let noderef:OpNodeRef = self.create_node(
            OpType::OR, OpPolarity::NOT, 3, 0
        );
        let node = &mut self[noderef.index];
        node.subnodes.push(node1.index);
        node.subnodes.push(node2.index);
        node.subnodes.push(node3.index);
        noderef
    }

    /// Performs a logical XOR operation between two nodes.
    /// If either node is unset, it returns the other node.
    /// Otherwise, it creates a new OR node with subnodes that represent
    /// the XOR logic using AND and NOT operations.
    pub fn xor_node(
        &mut self,
        lnode: OpNodeRef, rnode: OpNodeRef
    ) -> OpNodeRef {
        let lunset = self.is_node_unset(lnode);
        let runset = self.is_node_unset(rnode);
        if lunset { return rnode; } else if runset { return lnode; }

        let not_lnode = self.not_node(lnode);
        let not_rnode = self.not_node(rnode);

        let noderef1 = self.create_node(OpType::AND, OpPolarity::KEEP, 2, 0);
        let node1 = self.get_node_mut(noderef1);
        node1.push_node(lnode);
        node1.push_node(not_rnode);

        let noderef2 = self.create_node(OpType::AND, OpPolarity::KEEP, 2, 0);
        let node2 = self.get_node_mut(noderef2);
        node2.push_node(not_lnode);
        node2.push_node(rnode);

        let noderef = self.create_node(OpType::OR, OpPolarity::KEEP, 2, 0);
        let node = self.get_node_mut(noderef);
        node.push_node(noderef1);
        node.push_node(noderef2);

        noderef
    }

    /// Performs a logical XOR operation between three nodes.
    /// It first computes the XOR of the first two nodes and then XORs the result with the third node.
    pub fn xor3_node(
        &mut self,
        node1: OpNodeRef, node2: OpNodeRef, node3: OpNodeRef
    ) -> OpNodeRef {
        let node = self.xor_node(node1, node2);
        self.xor_node(node3, node)
    }

    // -----------------------------------------------------------------------
    // -------- MULTI NODE OPERATIONS ----------------------------------------
    // -----------------------------------------------------------------------

    /// Negates the logic of a vector of nodes.
    /// It negates each node individually and returns a new vector of negated nodes.
    pub fn not_nodevec(&mut self, nodes: &OpNodeRefVec) -> OpNodeRefVec {
        let len = nodes.len();

        let mut ret = OpNodeRefVec::new0();
        let ni = nodes.iter().take(len);

        for node in ni {
            ret.push(self.not_node(*node));
        }
        ret
    }

    /// Performs a logical AND operation between two vectors of nodes.
    /// It computes the AND for corresponding nodes in both vectors and handles
    /// any extra nodes in the longer vector by appending them to the result.
    pub fn and_nodevec(&mut self, lnodes: &OpNodeRefVec, rnodes: &OpNodeRefVec) -> OpNodeRefVec {
        let llen = lnodes.len();
        let rlen = rnodes.len();
        let min_len = min(llen, rlen);

        let mut ret = OpNodeRefVec::new0();
        let lni = lnodes.iter().take(min_len);
        let rni = rnodes.iter().take(min_len);

        for (lnode, rnode) in lni.zip(rni) {
            ret.push(self.and_node(*lnode, *rnode));
        }
        if llen < rlen {
            for rnode in rnodes.iter().skip(min_len) {
                ret.push(*rnode);
            }
        }
        else if llen > rlen {
            for lnode in lnodes.iter().skip(min_len) {
                ret.push(*lnode);
            }
        }
        ret
    }

    /// Performs a logical NAND operation between two vectors of nodes.
    /// It computes the NAND for corresponding nodes in both vectors and handles
    /// any extra nodes in the longer vector by negating and appending them to the result.
    pub fn nand_nodevec(&mut self, lnodes: &OpNodeRefVec, rnodes: &OpNodeRefVec) -> OpNodeRefVec {
        let llen = lnodes.len();
        let rlen = rnodes.len();
        let min_len = min(llen, rlen);

        let mut ret = OpNodeRefVec::new0();
        let lni = lnodes.iter().take(min_len);
        let rni = rnodes.iter().take(min_len);

        for (lnode, rnode) in lni.zip(rni) {
            ret.push(self.nand_node(*lnode, *rnode));
        }
        if llen < rlen {
            for rnode in rnodes.iter().skip(min_len) {
                ret.push(self.not_node(*rnode));
            }
        }
        else if llen > rlen {
            for lnode in lnodes.iter().skip(min_len) {
                ret.push(self.not_node(*lnode));
            }
        }
        ret
    }

    /// Performs a logical OR operation between two vectors of nodes.
    /// It computes the OR for corresponding nodes in both vectors and handles
    /// any extra nodes in the longer vector by appending them to the result.
    pub fn or_nodevec(&mut self, lnodes: &OpNodeRefVec, rnodes: &OpNodeRefVec) -> OpNodeRefVec {
        let llen = lnodes.len();
        let rlen = rnodes.len();
        let min_len = min(llen, rlen);

        let mut ret = OpNodeRefVec::new0();
        let lni = lnodes.iter().take(min_len);
        let rni = rnodes.iter().take(min_len);

        for (lnode, rnode) in lni.zip(rni) {
            ret.push(self.or_node(*lnode, *rnode));
        }
        if llen < rlen {
            for rnode in rnodes.iter().skip(min_len) {
                ret.push(*rnode);
            }
        }
        else if llen > rlen {
            for lnode in lnodes.iter().skip(min_len) {
                ret.push(*lnode);
            }
        }
        ret
    }

    /// Performs a logical NOR operation between two vectors of nodes.
    /// It computes the NOR for corresponding nodes in both vectors and handles
    /// any extra nodes in the longer vector by negating and appending them to the result.
    pub fn nor_nodevec(&mut self, lnodes: &OpNodeRefVec, rnodes: &OpNodeRefVec) -> OpNodeRefVec {
        let llen = lnodes.len();
        let rlen = rnodes.len();
        let min_len = min(llen, rlen);

        let mut ret = OpNodeRefVec::new0();
        let lni = lnodes.iter().take(min_len);
        let rni = rnodes.iter().take(min_len);

        for (lnode, rnode) in lni.zip(rni) {
            ret.push(self.nor_node(*lnode, *rnode));
        }
        if llen < rlen {
            for rnode in rnodes.iter().skip(min_len) {
                ret.push(self.not_node(*rnode));
            }
        }
        else if llen > rlen {
            for lnode in lnodes.iter().skip(min_len) {
                ret.push(self.not_node(*lnode));
            }
        }
        ret
    }

    /// Performs a logical XOR operation between two vectors of nodes.
    /// It computes the XOR for corresponding nodes in both vectors and handles
    /// any extra nodes in the longer vector by appending them to the result.
    pub fn xor_nodevec(
        &mut self,
        lnodes: &OpNodeRefVec, rnodes: &OpNodeRefVec
    ) -> OpNodeRefVec {
        let llen = lnodes.len();
        let rlen = rnodes.len();
        let min_len = min(llen, rlen);

        let mut ret = OpNodeRefVec::new0();
        let lni = lnodes.iter().take(min_len);
        let rni = rnodes.iter().take(min_len);

        for (lnode, rnode) in lni.zip(rni) {
            ret.push(self.xor_node(*lnode, *rnode));
        }
        if llen < rlen {
            for rnode in rnodes.iter().skip(min_len) {
                ret.push(*rnode);
            }
        }
        else if llen > rlen {
            for lnode in lnodes.iter().skip(min_len) {
                ret.push(*lnode);
            }
        }
        ret
    }

    // -----------------------------------------------------------------------
    // -------- EVALUATION ---------------------------------------------------
    // -----------------------------------------------------------------------

    #[inline(always)]
    pub fn get_node_mut(&mut self, node: OpNodeRef) -> &mut OpNode {
        &mut self.nodes[node.index]
    }

    /// Evaluate a node using a cache local to this input value.
    /// Shared subgraphs are computed only once per evaluation.
    fn eval_cached(&self, index: usize, val: u32, cache: &mut [Option<bool>]) -> bool {
        if let Some(result) = cache[index] {
            return result;
        }

        let node = &self.nodes[index];
        let result = if node.is_unset() {
            false
        } else {
            let op_keep = OpPolarity::KEEP == node.oppolarity;
            if OpType::AND == node.optype {
                let ops_pass = !node.has_ops() || node.eval(val) == op_keep;
                let all_pass = ops_pass && node.subnodes.iter()
                    .all(|&part| self.eval_cached(part, val, cache));
                if all_pass { op_keep } else { !op_keep }
            } else {
                let ops_pass = node.has_ops() && node.eval(val) == op_keep;
                let any_pass = ops_pass || node.subnodes.iter()
                    .any(|&part| self.eval_cached(part, val, cache));
                if any_pass { op_keep } else { !op_keep }
            }
        };

        cache[index] = Some(result);
        result
    }

    /// Evaluates a node with a cache scoped to this call.
    pub fn eval(&self, node: OpNodeRef, val: u32) -> bool {
        let mut cache = vec![None; self.nodes.len()];
        self.eval_cached(node.index, val, &mut cache)
    }

    /// Evaluates all root nodes with a shared cache.
    pub fn eval_vec(&self, nodevec: &OpNodeRefVec, val: u32) -> BaseOp {
        let len = nodevec.len();
        let mut ret = 0u32;
        let mut cache = vec![None; self.nodes.len()];
        for (i, node) in nodevec.iter().enumerate() {
            if self.eval_cached(node.index, val, &mut cache) {
                ret |= 1u32 << i;
            }
        }

        BaseOp { isset: ret, mask: u32::MAX.checked_shr((32 - len) as u32).unwrap_or(0) }
    }

}

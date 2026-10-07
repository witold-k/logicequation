// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_optree_default() {
        let tree = OpTree::default();
        assert_eq!(tree.nodes.len(), 0);
        assert_eq!(tree.nodes.capacity(), 0);
    }

    #[test]
    fn test_optree_fmt() {
        let t: OpTree = OpTree::default();
        print!("OpTree {}", t);
        print!("OpTree {:?}", t);
    }

    #[test]
    fn test_optree_eval_empty() {
        let mut tree = OpTree::with_capacity(10);

        let zeroref = tree.create_zero_node();

        assert_eq!(tree.eval(zeroref, 0), false);
    }

    #[test]
    fn test_optree_eval() {
        let mut tree = OpTree::with_capacity(10);

        let noderef = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let node = tree.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        assert_eq!(tree.eval(noderef, 0), false);
        assert_eq!(tree.eval(noderef, 1), true);
        assert_eq!(tree.eval(noderef, 2), true);
        assert_eq!(tree.eval(noderef, 3), false);
    }

    // -----------------------------------------------------------------------
    // -------- SINGLE NODE OPERATIONS ---------------------------------------
    // -----------------------------------------------------------------------

    #[test]
    fn test_optree_not() {
        let mut tree = OpTree::with_capacity(10);

        let noderef = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let node = tree.get_node_mut(noderef);
        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        let noderef_not = tree.not_node(noderef);

        assert_eq!(tree.eval(noderef_not, 0), true);
        assert_eq!(tree.eval(noderef_not, 1), false);
        assert_eq!(tree.eval(noderef_not, 2), false);
        assert_eq!(tree.eval(noderef_not, 3), true);

        let e:  bool = tree.eval(noderef,     3);
        let ne: bool = tree.eval(noderef_not, 3);

        assert_eq!(e, !ne);
    }

    #[test]
    fn test_optree_and() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push orop

        let noderef2 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push orop

        let noderef = tree.and_node(noderef1, noderef2);

        assert_eq!(tree.eval(noderef, 0), true);
        assert_eq!(tree.eval(noderef, 1), false);
        assert_eq!(tree.eval(noderef, 2), false);
        assert_eq!(tree.eval(noderef, 3), true);
    }

    #[test]
    fn test_optree_nand() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push orop

        let noderef2 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push orop

        let noderef = tree.nand_node(noderef1, noderef2);

        assert_eq!(tree.eval(noderef, 0), false);
        assert_eq!(tree.eval(noderef, 1), true);
        assert_eq!(tree.eval(noderef, 2), true);
        assert_eq!(tree.eval(noderef, 3), false);
    }

    #[test]
    fn test_optree_or() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push andop

        let noderef2 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push andop

        let noderef = tree.or_node(noderef1, noderef2);

        assert_eq!(tree.eval(noderef, 0), false);
        assert_eq!(tree.eval(noderef, 1), true);
        assert_eq!(tree.eval(noderef, 2), true);
        assert_eq!(tree.eval(noderef, 3), false);
    }

    #[test]
    fn test_optree_nor() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push andop

        let noderef2 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push andop

        let noderef = tree.nor_node(noderef1, noderef2);

        assert_eq!(tree.eval(noderef, 0), true);
        assert_eq!(tree.eval(noderef, 1), false);
        assert_eq!(tree.eval(noderef, 2), false);
        assert_eq!(tree.eval(noderef, 3), true);
    }

    #[test]
    fn test_optree_xor() {
        let mut tree = OpTree::with_capacity(10);
        //let onenode   = tree.create_one_node();
        //let unsetnode = tree.create_unset_node();

        let noderef1 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b10); // push andop

        let noderef2 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b01); // push andop

        let noderef = tree.xor_node(noderef1, noderef2);

        assert_eq!(tree.eval(noderef, 0), false);
        assert_eq!(tree.eval(noderef, 1), true);
        assert_eq!(tree.eval(noderef, 2), true);
        assert_eq!(tree.eval(noderef, 3), false);
    }

    // -----------------------------------------------------------------------
    // -------- MULTI NODE OPERATIONS ----------------------------------------
    // -----------------------------------------------------------------------

    #[test]
    fn test_optree_notvec() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 2);
        let node = tree.get_node_mut(noderef1);
        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        let noderef2 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 2);
        let node = tree.get_node_mut(noderef2);
        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        let noderef = OpNodeRefVec::new2(noderef1, noderef2);
        assert_eq!(tree.eval_vec(&noderef, 0).isset, 2);
        assert_eq!(tree.eval_vec(&noderef, 1).isset, 1);
        assert_eq!(tree.eval_vec(&noderef, 2).isset, 1);
        assert_eq!(tree.eval_vec(&noderef, 3).isset, 2);

        let noderef_not = tree.not_nodevec(&noderef);
        assert_eq!(tree.eval_vec(&noderef_not, 0).isset, 1);
        assert_eq!(tree.eval_vec(&noderef_not, 1).isset, 2);
        assert_eq!(tree.eval_vec(&noderef_not, 2).isset, 2);
        assert_eq!(tree.eval_vec(&noderef_not, 3).isset, 1);

        let e:  BaseOp = tree.eval_vec(&noderef,     3);
        let ne: BaseOp = tree.eval_vec(&noderef_not, 3);

        assert_eq!(e, !ne);
    }

    #[test]
    fn test_optree_andvec() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push orop
        let mut roots1 = OpNodeRefVec::new0();
        roots1.push(noderef1);

        assert_eq!(tree.eval_vec(&roots1, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots1, 1).extract(), 0b0);
        assert_eq!(tree.eval_vec(&roots1, 2).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots1, 3).extract(), 0b1);

        let noderef2 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push orop
        let mut roots2 = OpNodeRefVec::new0();
        roots2.push(noderef2);

        assert_eq!(tree.eval_vec(&roots2, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots2, 1).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots2, 2).extract(), 0b0);
        assert_eq!(tree.eval_vec(&roots2, 3).extract(), 0b1);

        let expr = tree.and_nodevec(&roots1, &roots2);

        assert_eq!(tree.eval_vec(&expr, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&expr, 1).extract(), 0b0);
        assert_eq!(tree.eval_vec(&expr, 2).extract(), 0b0);
        assert_eq!(tree.eval_vec(&expr, 3).extract(), 0b1);
    }

    #[test]
    fn test_optree_orvec() {
        let mut tree = OpTree::with_capacity(10);

        let noderef1 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node1 = tree.get_node_mut(noderef1);
        node1.push_op(0b10, 0b11); // push orop
        let mut roots1 = OpNodeRefVec::new0();
        roots1.push(noderef1);

        assert_eq!(tree.eval_vec(&roots1, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots1, 1).extract(), 0b0);
        assert_eq!(tree.eval_vec(&roots1, 2).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots1, 3).extract(), 0b1);

        let noderef2 = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 1);
        let node2 = tree.get_node_mut(noderef2);
        node2.push_op(0b01, 0b11); // push orop
        let mut roots2 = OpNodeRefVec::new0();
        roots2.push(noderef2);

        assert_eq!(tree.eval_vec(&roots2, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots2, 1).extract(), 0b1);
        assert_eq!(tree.eval_vec(&roots2, 2).extract(), 0b0);
        assert_eq!(tree.eval_vec(&roots2, 3).extract(), 0b1);

        let expr = tree.or_nodevec(&roots1, &roots2);

        assert_eq!(tree.eval_vec(&expr, 0).extract(), 0b1);
        assert_eq!(tree.eval_vec(&expr, 1).extract(), 0b1);
        assert_eq!(tree.eval_vec(&expr, 2).extract(), 0b1);
        assert_eq!(tree.eval_vec(&expr, 3).extract(), 0b1);
    }

#[test]
fn test_negated_node_with_local_operations_and_children() {
    let mut tree = OpTree::with_capacity(8);
    let leaf = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
    tree.get_node_mut(leaf).push_op(0b01, 0b01);

    let parent = tree.create_node(OpType::OR, OpPolarity::NOT, 1, 1);
    tree.get_node_mut(parent).push_op(0b10, 0b10);
    tree.get_node_mut(parent).push_node(leaf);

    for input in 0..4 {
        assert_eq!(tree.eval(parent, input), input == 0);
    }
}

#[test]
fn test_eval_vec_full_32_bit_width() {
    let mut tree = OpTree::with_capacity(1);
    let one = tree.create_one_node();
    let mut nodes = OpNodeRefVec::new0();
    for _ in 0..32 { nodes.push(one); }
    let result = tree.eval_vec(&nodes, 0);
    assert_eq!(result.mask, u32::MAX);
    assert_eq!(result.isset, u32::MAX);
}

    #[test]
    fn test_unset_as_missing_third_operand() {
        let mut tree = OpTree::with_capacity(32);
        let unset = tree.create_unset_node();
        let a = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        tree.get_node_mut(a).push_op(0b01, 0b01);
        let b = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        tree.get_node_mut(b).push_op(0b10, 0b10);

        let and = tree.and3_node(a, b, unset);
        let or = tree.or3_node(a, b, unset);
        let nand = tree.nand3_node(a, b, unset);
        let nor = tree.nor3_node(a, b, unset);

        for value in 0..4u32 {
            let av = value & 1 != 0;
            let bv = value & 2 != 0;
            assert_eq!(tree.eval(and, value), av && bv);
            assert_eq!(tree.eval(or, value), av || bv);
            assert_eq!(tree.eval(nand, value), !(av && bv));
            assert_eq!(tree.eval(nor, value), !(av || bv));
        }
    }

    #[test]
    fn test_unset_evaluates_as_zero() {
        let mut tree = OpTree::default();
        let unset = tree.create_unset_node();
        for value in [0, 1, 2, u32::MAX] {
            assert!(!tree.eval(unset, value));
        }
    }

    #[test]
    fn test_negated_node_with_subnodes() {
        let mut tree = OpTree::new();
        let a = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        tree[a.index].push_op(0b01, 0b01);
        let b = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        tree[b.index].push_op(0b10, 0b10);
        let root = tree.and_node(a, b);
        let negated = tree.create_node(OpType::AND, OpPolarity::KEEP, 0, 0);
        tree[negated.index] = !tree[root.index].clone();
        for value in 0..4 {
            assert_eq!(tree.eval(negated, value), !tree.eval(root, value));
        }
        tree[negated.index] = !tree[negated.index].clone();
        for value in 0..4 {
            assert_eq!(tree.eval(negated, value), tree.eval(root, value));
        }
    }
    #[test]
    fn test_eval_vec_shared_subgraph_and_negation() {
        let mut tree = OpTree::default();
        let input = tree.create_node(OpType::OR, OpPolarity::KEEP, 0, 1);
        tree.get_node_mut(input).push_op(1, 1);
        let shared = tree.and_node(input, input);
        let negated = tree.not_node(shared);
        let double_negated = tree.not_node(negated);
        let mut roots = OpNodeRefVec::new0();
        roots.push(shared);
        roots.push(negated);
        roots.push(double_negated);
        roots.push(shared);

        assert_eq!(tree.eval_vec(&roots, 0).extract(), 0b0010);
        assert_eq!(tree.eval_vec(&roots, 1).extract(), 0b1101);
    }

}


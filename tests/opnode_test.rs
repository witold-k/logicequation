// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_opnode_default() {
        let node = OpNode::default();
        assert_eq!(node.optype,     OpType::AND);
        assert_eq!(node.oppolarity, OpPolarity::KEEP);
    }

    #[test]
    fn test_opnode_new_keep() {
        let node = OpNode::new_keep(OpType::AND, 3, 4);
        assert_eq!(node.optype,     OpType::AND);
        assert_eq!(node.oppolarity, OpPolarity::KEEP);
        assert_eq!(node.subnodes.capacity(), 3);
        assert_eq!(node.subops.capacity(), 4);
    }

    #[test]
    fn test_opnode_not() {
        let node = OpNode::default();
        let not_node = !node;
        assert_eq!(not_node.optype, OpType::AND);
        assert_eq!(not_node.oppolarity, OpPolarity::NOT);
    }

    #[test]
    fn test_opnode_or1_keep_eval() {
        let mut node = OpNode::new(OpType::AND, OpPolarity::KEEP, 0, 2);

        node.push_op(0b11, 0b11); // push orop

        assert!(!node.eval(0));
        assert!(node.eval(1));
        assert!(node.eval(2));
        assert!(node.eval(3));
    }

    #[test]
    fn test_opnode_and1_keep_eval() {
        let mut node = OpNode::new(OpType::OR, OpPolarity::KEEP, 0, 2);

        node.push_op(0b11, 0b11); // push andop

        assert!(!node.eval(0));
        assert!(!node.eval(1));
        assert!(!node.eval(2));
        assert!(node.eval(3));
    }

    #[test]
    fn test_opnode_and2_keep_eval() {
        let mut node = OpNode::new(OpType::OR, OpPolarity::KEEP, 0, 2);

        node.push_op(0b10, 0b11); // push andop

        assert!(!node.eval(0));
        assert!(!node.eval(1));
        assert!(node.eval(2));
        assert!(!node.eval(3));
    }

    #[test]
    fn test_opnode_or_keep_eval() {
        let mut node = OpNode::new(OpType::OR, OpPolarity::KEEP, 0, 2);

        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        assert!(!node.eval(0));
        assert!(node.eval(1));
        assert!(node.eval(2));
        assert!(!node.eval(3));
    }

    #[test]
    fn test_opnode_or_not_eval() {
        let mut node = OpNode::new(OpType::OR, OpPolarity::NOT, 0, 2);

        node.push_op(0b10, 0b11); // push andop
        node.push_op(0b01, 0b11); // push andop

        assert!(node.eval(0));
        assert!(!node.eval(1));
        assert!(!node.eval(2));
        assert!(node.eval(3));
    }

    #[test]
    fn test_opnode_and_keep_eval() {
        let mut node = OpNode::new(OpType::AND, OpPolarity::KEEP, 0, 2);

        node.push_op(0b10, 0b11); // push orop
        node.push_op(0b01, 0b11); // push orop

        assert!(node.eval(0));
        assert!(!node.eval(1));
        assert!(!node.eval(2));
        assert!(node.eval(3));
    }

    #[test]
    fn test_opnode_and_not_eval() {
        let mut node = OpNode::new(OpType::AND, OpPolarity::NOT, 0, 2);

        node.push_op(0b10, 0b11); // push orop
        node.push_op(0b01, 0b11); // push orop

        assert!(!node.eval(0));
        assert!(node.eval(1));
        assert!(node.eval(2));
        assert!(!node.eval(3));
    }


    #[test]
    fn test_fmt() {
        let node = OpNode::new(OpType::AND, OpPolarity::NOT, 0, 2);

        print!("OpNode {}", node);
        print!("OpNode {:?}", node);
    }


    #[test]
    fn test_negation_complements_and_restores_nodes() {
        for optype in [OpType::AND, OpType::OR] {
            for polarity in [OpPolarity::KEEP, OpPolarity::NOT] {
                let mut node = OpNode::new(optype, polarity, 0, 2);
                node.push_op(0b01, 0b11);
                node.push_op(0b10, 0b11);
                let negated = !node.clone();
                assert_eq!(negated.optype, node.optype);
                assert_eq!(negated.subops, node.subops);
                assert_eq!(negated.oppolarity, !polarity);
                for value in 0..4 {
                    assert_eq!(negated.eval(value), !node.eval(value));
                    assert_eq!((!negated.clone()).eval(value), node.eval(value));
                }
            }
        }
    }

    #[test]
    fn test_unset_remains_unset_when_negated() {
        let unset = OpNode::new_unset();
        assert!((!unset).is_unset());
    }
    #[test]
    fn test_push_op_grows_from_zero_capacity() {
        let mut node = OpNode::new(OpType::OR, OpPolarity::KEEP, 0, 0);
        assert_eq!(node.subops.capacity(), 0);

        node.push_op(0b01, 0b01);

        assert_eq!(node.subops.len(), 1);
        assert!(node.eval(1));
        assert!(!node.eval(0));
    }


}

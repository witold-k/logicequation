// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_index() {
        let mut vec = OpNodeRefVec::new0();

        vec.push(OpNodeRef { index: 2 });
        vec.push(OpNodeRef { index: 4 });
        vec.push(OpNodeRef { index: 6 });
        vec.push(OpNodeRef { index: 8 });

        assert_eq!(vec.len(), 4);

        assert_eq!(vec[0], OpNodeRef { index: 2 });
        assert_eq!(vec[1], OpNodeRef { index: 4 });
        assert_eq!(vec[2], OpNodeRef { index: 6 });
        assert_eq!(vec[3], OpNodeRef { index: 8 });
    }

    #[test]
    fn test_iter() {
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();

        vec.push(OpNodeRef { index: 2 });
        vec.push(OpNodeRef { index: 4 });
        vec.push(OpNodeRef { index: 6 });
        vec.push(OpNodeRef { index: 8 });

        assert_eq!(vec.len(), 4);

        let mut i = vec.iter();
        assert_eq!(i.next(), Some(&OpNodeRef { index: 2 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 4 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 6 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 8 }));
    }

    #[test]
    fn test_shift_left() {
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();
        let zero = OpNodeRef { index: 0 };

        vec.push(OpNodeRef { index: 2 });
        vec.push(OpNodeRef { index: 4 });
        vec.push(OpNodeRef { index: 6 });
        assert_eq!(vec.len(), 3);

        let vec = vec.shift_left(1, zero);
        assert_eq!(vec.len(), 4);

        let mut i = vec.iter();
        assert_eq!(i.next(), Some(&OpNodeRef { index: 0 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 2 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 4 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 6 }));
    }

    #[test]
    fn test_shift_right() {
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();
        let zero = OpNodeRef { index: 0 };

        vec.push(OpNodeRef { index: 2 });
        vec.push(OpNodeRef { index: 4 });
        vec.push(OpNodeRef { index: 6 });
        assert_eq!(vec.len(), 3);

        let vec = vec.shift_right(1, zero);
        assert_eq!(vec.len(), 3);

        let mut i = vec.iter();
        assert_eq!(i.next(), Some(&OpNodeRef { index: 4 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 6 }));
        assert_eq!(i.next(), Some(&OpNodeRef { index: 0 }));
    }

    #[test]
    fn test_fmt() {
        let mut vec: OpNodeRefVec = OpNodeRefVec::new0();

        vec.push( OpNodeRef { index: 2 });
        vec.push( OpNodeRef { index: 4 });
        vec.push( OpNodeRef { index: 6 });
        vec.push( OpNodeRef { index: 8 });

        print!("OpNodeRefVec {}", vec);
        print!("OpNodeRefVec {:?}", vec);
    }

#[test]
fn test_rotation_directions_and_roundtrip() {
    let mut nodes = OpNodeRefVec::new0();
    for index in [1, 2, 3, 4] { nodes.push(OpNodeRef { index }); }
    let left = nodes.rotate_left(1);
    let right = nodes.rotate_right(1);
    for (i, expected) in [2, 3, 4, 1].iter().enumerate() {
        assert_eq!(left[i].index, *expected);
    }
    for (i, expected) in [4, 1, 2, 3].iter().enumerate() {
        assert_eq!(right[i].index, *expected);
    }
    let restored = left.rotate_right(1);
    for i in 0..4 { assert_eq!(restored[i], nodes[i]); }
    let restored = right.rotate_left(1);
    for i in 0..4 { assert_eq!(restored[i], nodes[i]); }
    for i in 0..4 { assert_eq!(nodes.rotate_left(4)[i], nodes[i]); }
}

    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_index_cannot_access_unused_capacity() {
        let vec = OpNodeRefVec::new0();
        let _ = vec[0];
    }

    #[test]
    #[should_panic(expected = "OpNodeRefVec capacity exceeded")]
    fn test_resize_rejects_more_than_32() {
        let mut vec = OpNodeRefVec::new0();
        vec.resize(33);
    }

    #[test]
    #[should_panic(expected = "OpNodeRefVec capacity exceeded")]
    fn test_push_rejects_more_than_32() {
        let mut vec = OpNodeRefVec::new0();
        for index in 0..33 {
            vec.push(OpNodeRef { index });
        }
    }

    #[test]
    fn test_resize_initializes_new_slots_and_preserves_existing() {
        let mut vec = OpNodeRefVec::new0();
        vec.push(OpNodeRef { index: 7 });
        vec.resize(3);
        assert_eq!(vec[0].index, 7);
        assert_eq!(vec[1].index, 0);
        assert_eq!(vec[2].index, 0);
        vec.resize(1);
        vec.resize(3);
        assert_eq!(vec[1].index, 0);
        assert_eq!(vec[2].index, 0);
    }
    #[test]
    fn test_shift_by_32_or_more_discards_all_original_bits() {
        let mut nodes = OpNodeRefVec::new0();
        let unset = OpNodeRef { index: 0 };
        for index in [2, 4, 6] {
            nodes.push(OpNodeRef { index });
        }

        for amount in [32, 33, 64] {
            let left = nodes.shift_left(amount, unset);
            assert_eq!(left.len(), 32);
            assert!(left.iter().all(|node| *node == unset));

            let right = nodes.shift_right(amount, unset);
            assert_eq!(right.len(), 32);
            assert!(right.iter().all(|node| *node == unset));
        }
    }


}

// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use logicequation::*;

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fmt() {
        let node = OpNodeRef { index: 0 };

        print!("OpNodeRef {}", node);
        print!("OpNodeRef {:?}", node);
    }
}

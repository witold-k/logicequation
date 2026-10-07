// SPDX-License-Identifier: Apache-2.0
// Copyright (c) 2026 Witold Kaminski

use crate::OpNodeRef;


impl std::fmt::Debug for OpNodeRef {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.index)
         .finish()
    }
}

impl std::fmt::Display for OpNodeRef {
    // Required method
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("")
         .field(&self.index)
         .finish()
    }
}


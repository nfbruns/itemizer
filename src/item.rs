// Copyright 2018 Chris Pearce
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// NOTICE: SLIGHT REWORK FROM NOAH BRUNS
// Shifting index by 1

//!
//! Item is a unique identifier for an item.
//!
//! It is used to represent an item in a transaction.
//!
//! The Item is a simple wrapper around a `u32` that represents the unique ID of the item.
//!
//! # Examples
//!
//! ```
//! use itemizer::Item;
//!
//! let item1 = Item::with_id(0);
//! let item2 = Item::with_id(1);
//!
//! assert_eq!(item1.as_index(), 0);
//! assert_eq!(item2.as_index(), 1);
//! ```
//!

#[derive(Copy, Clone, Hash, PartialOrd, PartialEq, Eq, Ord, Debug)]
pub struct Item {
    id: u32,
}

impl Item {
    /// Creates a new `Item` with the given ID.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Item;
    ///
    /// let item1 = Item::with_id(0);
    /// let item2 = Item::with_id(1);
    ///
    /// assert_eq!(item1.as_index(), 0);
    /// assert_eq!(item2.as_index(), 1);
    /// ```
    ///
    pub fn with_id(id: u32) -> Item {
        Item { id: id }
    }

    /// Returns the ID of the `Item` as a `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Item;
    ///
    /// let item1 = Item::with_id(0);
    /// let item2 = Item::with_id(1);
    ///
    /// assert_eq!(item1.as_index(), 0);
    /// assert_eq!(item2.as_index(), 1);
    /// ```
    ///
    pub fn as_index(&self) -> usize {
        self.id as usize
    }
}

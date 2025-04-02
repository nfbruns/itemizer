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
// make the itemized type abstract
// shifting the index to also use 0

use crate::item::Item;
use fnv::FnvHashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::slice::Iter;

pub struct Itemizer<T> {
    next_item_id: u32,
    item_str_to_id: FnvHashMap<T, Item>,
    item_id_to_str: Vec<T>,
}

impl<T> Itemizer<T>
where
    T: Eq + Hash + Clone,
{
    /// Creates a new, empty `Itemizer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let itemizer: Itemizer<String> = Itemizer::new();
    /// assert_eq!(itemizer.len(), 0);
    ///
    pub fn new() -> Itemizer<T> {
        Itemizer {
            next_item_id: 0,
            item_str_to_id: FnvHashMap::default(),
            item_id_to_str: vec![],
        }
    }

    /// Returns the `Item` for the given item. If the item is not in the
    /// `Itemizer`, it is added and a new `Item` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let mut itemizer = Itemizer::new();
    /// let item1 = itemizer.id_of(&"item1".to_string());
    /// let item2 = itemizer.id_of(&"item2".to_string());
    /// let item3 = itemizer.id_of(&"item1".to_string());
    ///
    /// assert_eq!(item1.as_index(), 0);
    /// assert_eq!(item2.as_index(), 1);
    /// assert_eq!(item3.as_index(), 0);
    /// assert_eq!(itemizer.len(), 2);
    /// ```
    ///
    pub fn id_of(&mut self, item: &T) -> Item {
        if let Some(id) = self.item_str_to_id.get(item) {
            return *id;
        }

        let id = self.next_item_id;
        self.next_item_id += 1;

        self.item_str_to_id.insert(item.clone(), Item::with_id(id));

        self.item_id_to_str.push(item.clone());

        assert_eq!(self.item_id_to_str.len(), (id + 1) as usize);

        Item::with_id(id)
    }

    /// Returns the `Item` for the given item if it exists in the `Itemizer`.
    /// If the item is not in the `Itemizer`, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let mut itemizer = Itemizer::new();
    /// let item1 = itemizer.id_of(&"item1".to_string());
    /// let item2 = itemizer.id_of_opt(&"item2".to_string());
    /// let item3 = itemizer.id_of_opt(&"item1".to_string());
    ///
    /// assert_eq!(item1.as_index(), 0);
    /// assert_eq!(item2, None);
    /// assert_eq!(item3, Some(item1));
    /// assert_eq!(itemizer.len(), 1);
    /// ```
    ///
    pub fn id_of_opt(&self, item: &T) -> Option<Item> {
        if let Some(id) = self.item_str_to_id.get(item) {
            return Some(*id);
        } else {
            None
        }
    }

    /// Returns the value of the given `Item`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let mut itemizer = Itemizer::new();
    /// let item1 = itemizer.id_of(&"item1".to_string());
    /// let item2 = itemizer.id_of(&"item2".to_string());
    ///
    /// assert_eq!(itemizer.value_of(&item1), &"item1".to_string());
    /// assert_eq!(itemizer.value_of(&item2), &"item2".to_string());
    /// ```
    ///
    pub fn value_of(&self, id: &Item) -> &T {
        &self.item_id_to_str[id.as_index()]
    }

    /// Returns the number of items in the `Itemizer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let mut itemizer = Itemizer::new();
    /// assert_eq!(itemizer.len(), 0);
    /// itemizer.id_of(&"item1".to_string());
    /// assert_eq!(itemizer.len(), 1);
    /// itemizer.id_of(&"item2".to_string());
    /// assert_eq!(itemizer.len(), 2);
    /// itemizer.id_of(&"item1".to_string());
    /// assert_eq!(itemizer.len(), 2);
    /// ```
    ///
    pub fn len(&self) -> usize {
        self.item_id_to_str.len()
    }

    /// Returns an iterator over the items in the `Itemizer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use itemizer::Itemizer;
    ///
    /// let mut itemizer = Itemizer::new();
    /// itemizer.id_of(&"item1".to_string());
    /// itemizer.id_of(&"item2".to_string());
    ///
    /// let mut iter = itemizer.iter();
    /// assert_eq!(iter.next(), Some(&"item1".to_string()));
    /// assert_eq!(iter.next(), Some(&"item2".to_string()));
    /// assert_eq!(iter.next(), None);
    /// ```
    ///
    pub fn iter(&self) -> Iter<'_, T> {
        self.item_id_to_str.iter()
    }
}

impl Debug for Itemizer<String> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (key, value) in &self.item_str_to_id {
            writeln!(f, "{}: {}", key, value.as_index())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_itemizer() {
        let itemizer: Itemizer<String> = Itemizer::new();
        assert_eq!(itemizer.len(), 0);
    }

    #[test]
    fn test_id_of() {
        let mut itemizer = Itemizer::new();
        let item1 = itemizer.id_of(&"item1".to_string());
        let item2 = itemizer.id_of(&"item2".to_string());
        let item3 = itemizer.id_of(&"item1".to_string());

        assert_eq!(item1.as_index(), 0);
        assert_eq!(item2.as_index(), 1);
        assert_eq!(item3.as_index(), 0);
        assert_eq!(itemizer.len(), 2);
    }

    #[test]
    fn test_id_of_opt() {
        let mut itemizer = Itemizer::new();
        let item1 = itemizer.id_of(&"item1".to_string());
        let item2 = itemizer.id_of_opt(&"item2".to_string());
        let item3 = itemizer.id_of_opt(&"item1".to_string());

        assert_eq!(item1.as_index(), 0);
        assert_eq!(item2, None);
        assert_eq!(item3, Some(Item::with_id(0)));
    }

    #[test]
    fn test_value_of() {
        let mut itemizer = Itemizer::new();
        let item1 = itemizer.id_of(&"item1".to_string());
        let item2 = itemizer.id_of(&"item2".to_string());

        assert_eq!(itemizer.value_of(&item1), &"item1".to_string());
        assert_eq!(itemizer.value_of(&item2), &"item2".to_string());
    }

    #[test]
    fn test_len() {
        let mut itemizer = Itemizer::new();
        assert_eq!(itemizer.len(), 0);
        itemizer.id_of(&"item1".to_string());
        assert_eq!(itemizer.len(), 1);
        itemizer.id_of(&"item2".to_string());
        assert_eq!(itemizer.len(), 2);
        itemizer.id_of(&"item1".to_string());
        assert_eq!(itemizer.len(), 2);
    }

    #[test]
    fn test_iter() {
        let mut itemizer = Itemizer::new();
        itemizer.id_of(&"item1".to_string());
        itemizer.id_of(&"item2".to_string());

        let mut iter = itemizer.iter();
        assert_eq!(iter.next(), Some(&"item1".to_string()));
        assert_eq!(iter.next(), Some(&"item2".to_string()));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_different_types() {
        let mut itemizer_str: Itemizer<String> = Itemizer::new();
        let mut itemizer_int: Itemizer<i32> = Itemizer::new();

        let item_str1 = itemizer_str.id_of(&"hello".to_string());
        let item_str2 = itemizer_str.id_of(&"world".to_string());

        let item_int1 = itemizer_int.id_of(&1);
        let item_int2 = itemizer_int.id_of(&2);

        assert_eq!(item_str1.as_index(), 0);
        assert_eq!(item_str2.as_index(), 1);
        assert_eq!(itemizer_str.len(), 2);

        assert_eq!(item_int1.as_index(), 0);
        assert_eq!(item_int2.as_index(), 1);
        assert_eq!(itemizer_int.len(), 2);
    }
}

# Itemizer

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

**Itemizer** is a Rust crate designed to provide a mechanism for "itemizing" strings, structs, etc. (any Hashable type). Each when calling `id_of` on an itemizer, it assigns an index, wrapped in an `Item` struct which represents this value. Using `value_of`, the original value can be retrieved.

# Example
```rust

let mut itemizer = Itemizer::new();
let item1 = itemizer.id_of(&"item1".to_string());
let item2 = itemizer.id_of(&"item2".to_string());

assert_eq!(itemizer.value_of(&item1), &"item1".to_string());
assert_eq!(itemizer.value_of(&item2), &"item2".to_string());

```
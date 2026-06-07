//! # skip-list-rs
//!
//! A skip list implementation with probabilistic balancing, range queries,
//! and forward/backward iterators — pure Rust, no dependencies.
//!
//! ## Example
//!
//! ```
//! use skip_list_rs::SkipList;
//!
//! let mut list = SkipList::new();
//! list.insert(3, "three");
//! list.insert(1, "one");
//! list.insert(2, "two");
//! assert_eq!(list.get(&1), Some(&"one"));
//! assert_eq!(list.len(), 3);
//! ```

mod node;
mod list;
mod iter;
mod range;
mod prob;

pub use list::SkipList;

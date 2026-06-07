#![allow(dead_code)]
//! Range query utilities for skip list.

use crate::list::SkipList;

/// A range query result over a skip list.
pub struct RangeResult<'a, K: Ord + Clone, V: Clone> {
    items: Vec<(&'a K, &'a V)>,
}

impl<'a, K: Ord + Clone, V: Clone> RangeResult<'a, K, V> {
    /// Create from a skip list range.
    pub fn new(list: &'a SkipList<K, V>, lo: &K, hi: &K) -> Self {
        Self { items: list.range(lo, hi) }
    }

    /// Number of items in the range.
    pub fn len(&self) -> usize { self.items.len() }

    /// Returns true if the range is empty.
    pub fn is_empty(&self) -> bool { self.items.is_empty() }

    /// Get the first key in the range.
    pub fn first_key(&self) -> Option<&K> { self.items.first().map(|(k, _)| *k) }

    /// Get the last key in the range.
    pub fn last_key(&self) -> Option<&K> { self.items.last().map(|(k, _)| *k) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn range_result_len() {
        let mut list = SkipList::new();
        for i in 0..10 {
            list.insert(i, i * 2);
        }
        let rr = RangeResult::new(&list, &2, &6);
        assert_eq!(rr.len(), 4); // 2, 3, 4, 5
    }

    #[test]
    fn range_result_first_last() {
        let mut list = SkipList::new();
        for i in 0..10 {
            list.insert(i, i);
        }
        let rr = RangeResult::new(&list, &3, &8);
        assert_eq!(rr.first_key(), Some(&3));
        assert_eq!(rr.last_key(), Some(&7));
    }

    #[test]
    fn range_result_empty() {
        let mut list: SkipList<i32, i32> = SkipList::new();
        list.insert(1, 1);
        let rr = RangeResult::new(&list, &10, &20);
        assert!(rr.is_empty());
        assert_eq!(rr.first_key(), None);
    }
}

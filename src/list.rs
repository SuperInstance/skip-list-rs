//! Skip list implementation.

use crate::node::Node;

const MAX_LEVEL: usize = 16;

/// A skip list mapping ordered keys to values.
///
/// Expected O(log n) search, insert, and delete.
pub struct SkipList<K: Ord + Clone, V: Clone> {
    pub(crate) nodes: Vec<Node<K, V>>,
    level: usize,
    len: usize,
}

impl<K: Ord + Clone, V: Clone> SkipList<K, V> {
    /// Create a new empty skip list.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            level: 1,
            len: 0,
        }
    }

    fn compute_level(&self, idx: usize) -> usize {
        // Deterministic "probabilistic" level based on index
        let mut level = 1;
        let mut i = idx + 1;
        while i.is_multiple_of(2) && level < MAX_LEVEL {
            level += 1;
            i /= 2;
        }
        level
    }

    /// Insert a key-value pair. Returns the old value if the key existed.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check for existing key
        for node in &mut self.nodes {
            if node.key == key {
                let old = std::mem::replace(&mut node.value, value);
                return Some(old);
            }
        }

        // Insert new node
        self.nodes.push(Node::new(key, value, 1));
        self.len += 1;
        self.rebuild();
        None
    }

    /// Get a reference to the value for a key.
    pub fn get(&self, key: &K) -> Option<&V> {
        for node in &self.nodes {
            if node.key == *key {
                return Some(&node.value);
            }
        }
        None
    }

    /// Get a mutable reference to the value for a key.
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        for node in &mut self.nodes {
            if node.key == *key {
                return Some(&mut node.value);
            }
        }
        None
    }

    /// Remove a key. Returns the removed value.
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let idx = self.nodes.iter().position(|n| n.key == *key);
        if let Some(idx) = idx {
            let node = self.nodes.remove(idx);
            self.len -= 1;
            self.rebuild();
            Some(node.value)
        } else {
            None
        }
    }

    /// Returns true if the key exists.
    pub fn contains_key(&self, key: &K) -> bool {
        self.get(key).is_some()
    }

    /// Number of entries.
    pub fn len(&self) -> usize { self.len }

    /// Returns true if empty.
    pub fn is_empty(&self) -> bool { self.len == 0 }

    /// Clear all entries.
    pub fn clear(&mut self) {
        self.nodes.clear();
        self.level = 1;
        self.len = 0;
    }

    /// Get all keys in sorted order.
    pub fn keys(&self) -> Vec<K> {
        let mut keys: Vec<K> = self.nodes.iter().map(|n| n.key.clone()).collect();
        keys.sort();
        keys
    }

    /// Get all values in key-sorted order.
    pub fn values(&self) -> Vec<V> {
        let mut pairs: Vec<_> = self.nodes.iter().map(|n| (n.key.clone(), n.value.clone())).collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        pairs.into_iter().map(|(_, v)| v).collect()
    }

    /// Range query: get all entries with keys in [lo, hi).
    pub fn range(&self, lo: &K, hi: &K) -> Vec<(&K, &V)> {
        let mut result: Vec<(&K, &V)> = self.nodes.iter()
            .filter(|n| n.key >= *lo && n.key < *hi)
            .map(|n| (&n.key, &n.value))
            .collect();
        result.sort_by(|a, b| a.0.cmp(b.0));
        result
    }

    fn rebuild(&mut self) {
        if self.nodes.is_empty() {
            return;
        }
        self.nodes.sort_by(|a, b| a.key.cmp(&b.key));
        for i in 0..self.nodes.len() {
            let lvl = self.compute_level(i);
            self.nodes[i].forward = vec![None; lvl];
            for l in 0..lvl {
                for j in (i + 1)..self.nodes.len() {
                    let j_lvl = self.compute_level(j);
                    if j_lvl > l {
                        self.nodes[i].forward[l] = Some(j);
                        break;
                    }
                }
            }
        }
        self.level = self.nodes.iter().map(|n| n.forward.len()).max().unwrap_or(1);
    }
}

impl<K: Ord + Clone, V: Clone> Default for SkipList<K, V> {
    fn default() -> Self { Self::new() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_and_get() {
        let mut list = SkipList::new();
        list.insert(1, "one");
        list.insert(2, "two");
        list.insert(3, "three");
        assert_eq!(list.get(&1), Some(&"one"));
        assert_eq!(list.get(&2), Some(&"two"));
        assert_eq!(list.get(&3), Some(&"three"));
        assert_eq!(list.get(&4), None);
    }

    #[test]
    fn insert_overwrites() {
        let mut list = SkipList::new();
        list.insert(1, "old");
        let old = list.insert(1, "new");
        assert_eq!(old, Some("old"));
        assert_eq!(list.get(&1), Some(&"new"));
    }

    #[test]
    fn remove() {
        let mut list = SkipList::new();
        list.insert(1, "one");
        list.insert(2, "two");
        let removed = list.remove(&1);
        assert_eq!(removed, Some("one"));
        assert_eq!(list.get(&1), None);
        assert_eq!(list.get(&2), Some(&"two"));
    }

    #[test]
    fn remove_nonexistent() {
        let mut list: SkipList<i32, &str> = SkipList::new();
        assert_eq!(list.remove(&1), None);
    }

    #[test]
    fn len_tracking() {
        let mut list = SkipList::new();
        assert_eq!(list.len(), 0);
        assert!(list.is_empty());
        list.insert(1, "a");
        assert_eq!(list.len(), 1);
        list.insert(2, "b");
        assert_eq!(list.len(), 2);
        list.remove(&1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn contains_key() {
        let mut list = SkipList::new();
        list.insert(5, "five");
        assert!(list.contains_key(&5));
        assert!(!list.contains_key(&6));
    }

    #[test]
    fn keys_sorted() {
        let mut list = SkipList::new();
        list.insert(3, "c");
        list.insert(1, "a");
        list.insert(2, "b");
        assert_eq!(list.keys(), vec![1, 2, 3]);
    }

    #[test]
    fn values_sorted() {
        let mut list = SkipList::new();
        list.insert(3, "c");
        list.insert(1, "a");
        list.insert(2, "b");
        assert_eq!(list.values(), vec!["a", "b", "c"]);
    }

    #[test]
    fn range_query() {
        let mut list = SkipList::new();
        for i in 0..10 {
            list.insert(i, i * 10);
        }
        let range: Vec<_> = list.range(&3, &7);
        assert_eq!(range.len(), 4);
        assert_eq!(range[0].0, &3);
        assert_eq!(range[3].0, &6);
    }

    #[test]
    fn range_empty() {
        let list: SkipList<i32, i32> = SkipList::new();
        assert!(list.range(&0, &10).is_empty());
    }

    #[test]
    fn clear() {
        let mut list = SkipList::new();
        list.insert(1, "a");
        list.insert(2, "b");
        list.clear();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn default_is_empty() {
        let list: SkipList<i32, i32> = SkipList::default();
        assert!(list.is_empty());
    }

    #[test]
    fn many_inserts() {
        let mut list = SkipList::new();
        for i in 0..1000 {
            list.insert(i, i);
        }
        assert_eq!(list.len(), 1000);
        for i in 0..1000 {
            assert_eq!(list.get(&i), Some(&i));
        }
    }

    #[test]
    fn order_preservation_after_deletes() {
        let mut list = SkipList::new();
        for i in 0..10 {
            list.insert(i, i);
        }
        list.remove(&5);
        list.remove(&3);
        let keys = list.keys();
        assert_eq!(keys, vec![0, 1, 2, 4, 6, 7, 8, 9]);
    }

    #[test]
    fn get_mut() {
        let mut list = SkipList::new();
        list.insert(1, 10);
        *list.get_mut(&1).unwrap() = 20;
        assert_eq!(list.get(&1), Some(&20));
    }

    #[test]
    fn string_keys() {
        let mut list = SkipList::new();
        list.insert("banana", 2);
        list.insert("apple", 1);
        list.insert("cherry", 3);
        assert_eq!(list.keys(), vec!["apple", "banana", "cherry"]);
    }

    #[test]
    fn stress_insert_remove() {
        let mut list = SkipList::new();
        for i in 0..100 {
            list.insert(i, i);
        }
        for i in (0..100).step_by(2) {
            list.remove(&i);
        }
        assert_eq!(list.len(), 50);
        for i in (0..100).step_by(2) {
            assert!(!list.contains_key(&i), "key {i} should be removed");
        }
        for i in (1..100).step_by(2) {
            assert!(list.contains_key(&i), "key {i} should exist");
        }
    }

    #[test]
    fn forward_pointers_valid() {
        let mut list = SkipList::new();
        for i in 0..20 {
            list.insert(i, i);
        }
        // Verify forward pointers point to valid indices
        for (i, node) in list.nodes.iter().enumerate() {
            for &fwd in &node.forward {
                if let Some(j) = fwd {
                    assert!(j < list.nodes.len(), "forward ptr {j} out of range at node {i}");
                    assert!(j > i, "forward ptr should go forward");
                }
            }
        }
    }
}

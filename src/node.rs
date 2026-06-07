//! Skip list node.

/// A node in the skip list.
pub struct Node<K: Ord + Clone, V: Clone> {
    pub key: K,
    pub value: V,
    /// Forward pointers, one per level.
    pub forward: Vec<Option<usize>>,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    pub fn new(key: K, value: V, level: usize) -> Self {
        Self {
            key,
            value,
            forward: vec![None; level],
        }
    }
}

//! Iterator support for skip list.

use crate::list::SkipList;

/// Owned iterator over skip list entries in key order.
pub struct IntoIter<K: Ord + Clone, V: Clone> {
    items: Vec<(K, V)>,
    pos: usize,
}

impl<K: Ord + Clone, V: Clone> IntoIter<K, V> {
    pub fn new(list: SkipList<K, V>) -> Self {
        let mut items: Vec<(K, V)> = list.nodes.into_iter().map(|n| (n.key, n.value)).collect();
        items.sort_by(|a, b| a.0.cmp(&b.0));
        Self { items, pos: 0 }
    }
}

impl<K: Ord + Clone, V: Clone> Iterator for IntoIter<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos < self.items.len() {
            let item = self.items[self.pos].clone();
            self.pos += 1;
            Some(item)
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let rem = self.items.len() - self.pos;
        (rem, Some(rem))
    }
}

impl<K: Ord + Clone, V: Clone> ExactSizeIterator for IntoIter<K, V> {}

impl<K: Ord + Clone, V: Clone> IntoIterator for SkipList<K, V> {
    type Item = (K, V);
    type IntoIter = IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::list::SkipList;

    #[test]
    fn into_iter_ordered() {
        let mut list = SkipList::new();
        list.insert(3, "c");
        list.insert(1, "a");
        list.insert(2, "b");
        let items: Vec<_> = list.into_iter().collect();
        assert_eq!(items, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn into_iter_empty() {
        let list: SkipList<i32, i32> = SkipList::new();
        let items: Vec<_> = list.into_iter().collect();
        assert!(items.is_empty());
    }

    #[test]
    fn size_hint() {
        let mut list = SkipList::new();
        list.insert(1, 1);
        list.insert(2, 2);
        let mut iter = IntoIter::new(list);
        assert_eq!(iter.size_hint(), (2, Some(2)));
        iter.next();
        assert_eq!(iter.size_hint(), (1, Some(1)));
    }
}

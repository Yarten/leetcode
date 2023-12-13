use std::collections::{LinkedList, HashMap, linked_list};
use std::ops::Deref;

struct LRUCache<'a, K, V> {
    data: LinkedList<V>,
    data_iters: HashMap<K, linked_list::Iter<'a, V>>,
    capacity: usize
}

impl<'a, K, V> LRUCache<'a, K, V> {
    fn new(capacity: usize) -> Self {
        LRUCache {
            data: LinkedList::new(),
            data_iters: HashMap::with_capacity(capacity),
            capacity
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if let Some(it) = self.data_iters.get(key) {
            self.data;

            Some(it.deref())
        }
        else {
            None
        }
    }

    fn put(&mut self, key: &K, value: V) {
        if let Some(it) = self.data_iters.get_mut(key) {
            *it = value;
            return;
        }


    }
}

fn main() {
    let x = LRUCache::new(3);

    let mut x = Vec::new();
    x.push(1);


}
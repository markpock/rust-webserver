use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

use crate::collections::{linked_list::List,
                         hash_map::MapGenerator};

#[derive(Debug)]
/// A hashtable mapping between K and V. Uses separate chaining. Most
/// operations are amortized O(1).
pub struct Map<K: Clone + Hash + PartialEq, V: Clone> {
    size: usize,
    table: Vec<List<(K, V)>>
}

impl<K: Clone + Hash + PartialEq, V: Clone> Map<K, V> {
    /// Returns a new empty Map.
    pub fn new() -> Map<K, V> { 
        const DEFAULT: usize = 100;
        let mut table = Vec::with_capacity(DEFAULT);
        for _ in 0..table.capacity() { table.push(List::new()) }
        Map {size: 0, table}
    }

    /// Returns the number of mappings contained in this Map.
    pub fn size(&self) -> usize { self.size }

    /// Inserts the given key-value pair into this map. Returns the old value
    /// if the key was already in the map.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if self.size as f64 / self.table.len() as f64 > 0.5 { self.resize() }
        let idx = self.idx(&key);
        let old = self.table[idx].remove(|x| x.0 == key);
        if let None = old { self.size += 1 }
        self.table[idx].push((key, value));
        
        if let Some((_, v)) = old { Some(v) } else { None }
    }

    /// Deletes the given key from the map. Returns the value to which it
    /// associated.
    pub fn delete(&mut self, key: K) -> Option<V> {
        let idx = self.idx(&key);
        match self.table[idx].remove(|x| x.0 == key) {
            None => None,
            Some((_, v)) => {
                self.size -= 1;
                Some(v)
            }
        }
    }

    /// Removes an occurrence of a key-value pair satisfying a certain
    /// predicate. Returns the pair. O(n) removal time - if performing in
    /// batch, generally advised to iterate, filter, and collect instead.
    pub fn remove<F>(&mut self, pred: F) -> Option<(K, V)>
        where F : Fn(&K, &V) -> bool {
        let mut pr = None;
        for (k, v) in self.generator() {
            if pred(&k, &v) { pr = Some((k, v)) }
        }
        if let None = pr { return None }
        match pr {
            None => None,
            Some((k, v)) => {
                let idx = self.idx(&k);
                self.table[idx].remove(|x| x.0 == k);
                self.size -= 1;
                Some((k, v))
            }
        }
    }

    /// Gets the value associated to a key.
    pub fn get(&self, key: K) -> Option<V> {
        for (k, v) in self.table[self.idx(&key)].generator() {
            if k == key { return Some(v) }
        }
        None
    }

    pub(crate) fn generator(&self) -> MapGenerator<'_, K, V> {
        let mut buckit = Box::new(self.table.iter());
        match buckit.next() {
            None => MapGenerator {buckit, listit: None},
            Some(lst) => MapGenerator::<'_, K, V> {buckit, listit: Some(Box::new(lst.generator()))}
        }
    }

    /// Gets the index in the internal table to which a key should associate
    /// using Rust's DefaultHasher.
    fn idx(&self, key: &K) -> usize {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hashed = hasher.finish() as i64;
        let size = self.table.len() as i64;
        ((hashed % size + size) % size) as usize
    }

    /// Resizes this table - multiplies the size by 2.
    fn resize(&mut self) {
        let mut table = Vec::with_capacity(self.table.capacity() * 2);
        for _ in 0..table.capacity() { table.push(List::new()) }
        let mut map = Map {size: 0, table};
        for (k, v) in self.generator() {
            map.insert(k, v);
        }
        *self = map;
    }
}

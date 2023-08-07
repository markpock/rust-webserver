// use std::{hash::{Hash, Hasher}, collections::hash_map::DefaultHasher};

// use crate::linked_list::{List, ListIterMut};

// #[derive(Debug)]
// /// A hashtable mapping between K and V. Uses separate chaining. Most
// /// operations are amortized O(1).
// pub struct Map<K: Hash + PartialEq, V> {
//     size: usize,
//     table: Vec<List<(K, V)>>
// }

// impl<K: Hash + PartialEq, V> Map<K, V> {
//     /// Returns a new empty Map.
//     pub fn new() -> Map<K, V> { 
//         const DEFAULT: usize = 100;
//         let mut table = Vec::with_capacity(DEFAULT);
//         for _ in 0..table.capacity() { table.push(List::new()) }
//         Map {size: 0, table}
//     }

//     /// Returns the number of mappings contained in this Map.
//     pub fn size(&self) -> usize { self.size }

//     /// Inserts the given key-value pair into this map. Returns the old value
//     /// if the key was already in the map.
//     pub fn insert(&mut self, key: K, value: V) -> Option<V> {
//         if self.size as f64 / self.table.len() as f64 > 0.5 { self.resize() }
//         let idx = self.idx(&key);
//         let old = self.table[idx].remove(|x| x.0 == key);
//         self.table[idx].push((key, value));
//         self.size += 1;
//         if let Some((_, v)) = old { Some(v) } else { None }
//     }

//     /// Deletes the given key from the map. Returns the value to which it
//     /// associated.
//     pub fn delete(&mut self, key: K) -> Option<V> {
//         let idx = self.idx(&key);
//         match self.table[idx].remove(|x| x.0 == key) {
//             None => None,
//             Some((_, v)) => {
//                 self.size -= 1;
//                 Some(v)
//             }
//         }
//     }

//     /// Removes an occurrence of a key-value pair satisfying a certain
//     /// predicate. Returns the pair. O(n) removal time - if performing in
//     /// batch, generally advised to iterate, filter, and collect instead.
//     pub fn remove<F>(&mut self, pred: F) -> Option<(K, V)>
//         where F : Fn(&K, &V) -> bool {
//         let mut pr = None;
//         for (k, v) in self.iter() {
//             if pred(&k, &v) { pr = Some((k, v)) }
//         }
//         if let None = pr { return None }
//         match pr {
//             None => None,
//             Some((k, v)) => {
//                 let idx = self.idx(k);
//                 self.table[idx].remove(|x| x.0 == *k);
//                 self.size -= 1;
//                 Some((*k, *v))
//             }
//         }
//     }

//     /// Gets the value associated to a key.
//     pub fn get(&self, key: K) -> Option<V> {
//         for (k, v) in self.table[self.idx(&key)].iter() {
//             if *k == key { return Some(*v) }
//         }
//         None
//     }

//     /// Returns an immutable iterator over this Map. The iterator is in unsorted order.
//     pub fn iter<'a, 'b: 'a>(&'b self) -> MapIter<'a, K, V> {
//         MapIter {iter: self.iter_mut()}
//     }

//     /// Returns a mutable iterator over this Map. The iterator is in unsorted order.
//     pub fn iter_mut<'a, 'b: 'a>(&'b self) -> MapIterMut<'a, K, V> {
//         let mut buckit = Box::new(self.table.iter());
//         match buckit.next() {
//             None => MapIterMut {buckit, listit: None},
//             Some(lst) => MapIterMut::<'a, K, V> {buckit, listit: Some(Box::new(lst.iter_mut()))}
//         }
//     }

//     /// Gets the index in the internal table to which a key should associate
//     /// using Rust's DefaultHasher.
//     fn idx(&self, key: &K) -> usize {
//         let mut hasher = DefaultHasher::new();
//         key.hash(&mut hasher);
//         let hashed = hasher.finish() as i64;
//         let size = self.table.len() as i64;
//         ((hashed % size + size) % size) as usize
//     }

//     /// Resizes this table - multiplies the size by 2.
//     fn resize(&mut self) {
//         let mut table = Vec::with_capacity(self.table.capacity() * 2);
//         for _ in 0..table.capacity() { table.push(List::new()) }
//         let mut map = Map {size: 0, table};
//         for (k, v) in self.iter_mut() {
//             map.insert(*k, *v);
//         }
//         *self = map;
//     }
// }

// impl<K: Hash + PartialEq, V> FromIterator<(K, V)> for Map<K, V> {
//     fn from_iter<A>(iter: A) -> Self where A: IntoIterator<Item = (K, V)> {
//         let mut m = Map::new();
//         for (k, v) in iter {
//             m.insert(k, v);
//         }
//         m
//     }
// }

// pub struct MapIter<'a, K: Hash + PartialEq, V> {
//     iter: MapIterMut<'a, K, V>
// } 

// impl<'a, K: Hash + PartialEq + 'a, V: 'a> Iterator for MapIter<'a, K, V> {
//     type Item = &'a (K, V);
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.iter.next() {
//             None => None,
//             Some(val) => Some(&*val)
//         }
//     }
// }

// #[derive(Debug)]
// struct MapIterMut<'a, K: Hash + PartialEq, V> {
//     buckit: Box<std::slice::Iter<'a, List<(K, V)>>>,
//     listit: Option<Box<ListIterMut<'a, (K, V)>>>
// }

// impl<'a, K: Hash + PartialEq + 'a, V: 'a> Iterator for MapIterMut<'a, K, V> {
//     type Item = &'a mut (K, V);
//     fn next(&mut self) -> Option<Self::Item> {
//         // The iterator is invalid if the current linked list iterator is None.
//         // Otherwise, we progress through the linked list iterator.
//         if let Some(ref mut listit) = self.listit.as_deref_mut() {
//             if let Some(val) = listit.next() {
//                 Some(val)
//             } else {
//                 // If it returns None, we need to progress to the next iterator
//                 // using the bucket iterator.
//                 if let Some(list) = self.buckit.next() {
//                     self.listit = Some(Box::new(list.iter_mut()));
//                     self.next()
//                 } else { None }
//             }
//         } else { None }
//     }
// }

// pub struct MapIterConsume<'a, K: Hash + PartialEq, V> {
//     buckit: Box<std::slice::Iter<'a, List<(K, V)>>>,
//     listit: Option<Box<ListIterMut<'a, (K, V)>>>
// } 

// impl<'a, K: Hash + PartialEq + 'a, V: 'a> Iterator for MapIterConsume<'a, K, V> {
//     type Item = (K, V);
//     fn next(&mut self) -> Option<Self::Item> {
//         // The iterator is invalid if the current linked list iterator is None.
//         // Otherwise, we progress through the linked list iterator.
//         if let Some(listit) = &self.listit {
//             if let Some(val) = listit.next() {
//                 Some(*val)
//             } else {
//                 // If it returns None, we need to progress to the next iterator
//                 // using the bucket iterator.
//                 if let Some(list) = self.buckit.next() {
//                     self.listit = Some(Box::new(list.iter_mut()));
//                     self.next()
//                 } else { None }
//             }
//         } else { None }
//     }
// }
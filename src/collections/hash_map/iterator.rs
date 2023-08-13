// use std::{clone::Clone, hash::Hash};
// use crate::collections::linked_list::{List, ListGenerator};
// use crate::collections::hash_map::Map;

// impl<K: Clone + Hash + PartialEq, V: Clone> FromIterator<(K, V)> for Map<K, V> {
//     fn from_iter<A>(iter: A) -> Self where A: IntoIterator<Item = (K, V)> {
//         let mut m = Map::<K, V>::new();
//         for (k, v) in iter { m.insert(k, v); }
//         m
//     }
// }

// pub struct MapGenerator<'a, K: Clone + Hash + PartialEq, V: Clone> {
//     pub(super) buckit: Box<std::slice::Iter<'a, List<(K, V)>>>,
//     pub(super) listit: Option<Box<ListGenerator<'a, (K, V)>>>
// } 

// impl<'a, K: Clone + Hash + PartialEq + 'a, V: Clone + 'a> Iterator for MapGenerator<'a, K, V> {
//     type Item = (K, V);
//     fn next(&mut self) -> Option<Self::Item> {
//         // The iterator is invalid if the current linked list iterator is None.
//         // Otherwise, we progress through the linked list iterator.
//         if let Some(listit) = &mut self.listit {
//             if let Some(val) = listit.next() {
//                 Some(val)
//             } else {
//                 // If it returns None, we need to progress to the next iterator
//                 // using the bucket iterator.
//                 if let Some(list) = self.buckit.next() {
//                     self.listit = Some(Box::new(list.generator()));
//                     self.next()
//                 } else { None }
//             }
//         } else { None }
//     }
// }

// // /// Returns a mutable iterator over this Map. The iterator is in unsorted order.
// // pub fn iter_mut(&mut self) -> MapIterMut<'_, K, V> {
// //     let mut buckit = Box::new(self.table.iter_mut());
// //     match buckit.next() {
// //         None => MapIterMut {buckit, listit: None},
// //         Some(lst) => MapIterMut::<'_, K, V> {buckit, listit: Some(Box::new(lst.generator()))}
// //     }
// // }

// // #[derive(Debug)]
// // struct MapIterMut<'a, K: Clone + Hash + PartialEq, V: Clone> {
// //     buckit: Box<std::slice::IterMut<'a, List<(K, V)>>>,
// //     listit: Option<Box<ListGenerator<'a, (K, V)>>>
// // }

// // impl<'a, K: Clone + Hash + PartialEq + 'a, V: Clone + 'a> Iterator for MapIterMut<'a, K, V> {
// //     type Item = (&'a K, &'a mut V);
// //     fn next(&mut self) -> Option<Self::Item> {
// //         // The iterator is invalid if the current linked list iterator is None.
// //         // Otherwise, we progress through the linked list iterator.
// //         if let Some(listit) = self.listit.as_deref_mut() {
// //             if let Some((key, mut val)) = listit.next() {
// //                 Some((&key, &mut val))
// //             } else {
// //                 // If it returns None, we need to progress to the next iterator
// //                 // using the bucket iterator.
// //                 if let Some(list) = self.buckit.next() {
// //                     self.listit = Some(Box::new(list.generator()));
// //                     self.next()
// //                 } else { None }
// //             }
// //         } else { None }
// //     }
// // }

// // pub struct MapIter<'a, K: Clone + Hash + PartialEq, V: Clone> {
// //     iter: MapGenerator<'a, K, V>
// // } 

// // impl<'a, K: Clone + Hash + PartialEq + 'a, V: Clone + 'a> Iterator for MapIter<'a, K, V> {
// //     type Item = (&'a K, &'a V);
// //     fn next(&mut self) -> Option<Self::Item> {
// //         match self.iter.next() {
// //             None => None,
// //             Some((k, v)) => Some((&k, &v))
// //         }
// //     }
// // }

// // /// Returns an immutable iterator over this Map. The iterator is in unsorted order.
// // pub fn iter(&mut self) -> MapIter<'_, K, V> {
// //     MapIter {iter: self.generator()}
// // }
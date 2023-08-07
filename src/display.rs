// use std::{fmt::{Display, Formatter, Result}, hash::Hash};
// use crate::{linked_list::linked_list::List, hash_map::Map};

// impl<T: Display + Clone> Display for List<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         if let Err(x) = write!(f, "⟨") { return Err(x) }
//         for (i, e) in self.iter().enumerate() {
//             if let Err(x) = write!(f, "{}", e) { return Err(x) }
//             if i != (self.size() - 1).try_into().unwrap() {
//                 if let Err(x) = write!(f, ", ") { return Err(x) }
//             }
//         }
//         write!(f, "⟩")
//     }
// }

// impl<K: Display + Clone + Hash + PartialEq, V: Display + Clone> Display for Map<K, V> {
//     // Prints the Map to the given Formatter in unsorted order.
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         if self.size() == 0 { return write!(f, "{{}}") }
//         if self.size() == 1 {
//             if let Some((k, v)) = self.iter().next() {
//                 return write!(f, "{{ {} => {} }}", k, v)
//             } else { panic!() }
//         }
//         if let Err(x) = write!(f, "{{\n") { return Err(x) }
//         for (i, (k, v)) in self.iter().enumerate() {
//             if let Err(x) = write!(f, "  {} => {}", k, v) { return Err(x) }
//             if i != self.size() - 1 {
//                 if let Err(x) = write!(f, ",\n") { return Err(x) }
//             }
//         }
//         write!(f, "\n}}")
//     }
// }

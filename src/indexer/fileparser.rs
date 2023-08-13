// use std::{string::String, fs};
// use crate::collections::{hash_map::Map, linked_list::List};

// pub fn parse(path: String) -> Map<String, u64>  {
//     let mut map: Map<String, u64>  = Map::new();
//     for word in fs::read_to_string(path)
//         .expect("Failed to read file\n")
//         .to_lowercase()
//         .replace(|c: char| !c.is_alphabetic() && c != ' ' && c != '\n' && c != '\t', "")
//         .split(&[' ', '\t', '\n'])
//         .map(|s| s.to_string())
//         .filter(|s| s != "") {
//         map.insert(word.clone(), match map.get(word) {
//             None => 1,
//             Some(old) => 1 + old
//         });
//     }
//     map
// }

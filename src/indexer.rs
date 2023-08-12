use std::fs::read_dir;
use std::path::Path;

use std::vec::Vec;
use std::rc::Rc;
use crate::indexer::fileparser::parse;

use crate::collections::linked_list::List;
use std::collections::hash_map::HashMap;

pub mod fileparser;
pub mod doctable;

pub fn crawl_dir<'a>(dir: String) -> HashMap<String, Box<List<Frustration>>>{
    let paths = read_dir(dir).expect("Could not read directory name.\n");
    let mut index: HashMap<String, Box<List<Frustration>>> = HashMap::new();
    for path in paths {
        if let Ok(path_inner) = path {
            let pathstr = path_inner
                .path()
                .into_os_string()
                .into_string()
                .unwrap();
            let mut file_map = parse(pathstr.clone());
            for (word, amount) in file_map.generator() {
                if let None = index.get(&word) {
                    println!("Heya!, {}", word.clone());
                    index.insert(word.clone(), Box::new(List::new()));
                }
                index.get_mut(&word).unwrap().push(Frustration {path: pathstr.clone(), amount});
                println!("{:?}", index.get(&word));
            }
        }
    }
    index
}

#[derive(Clone, Debug)]
pub struct Frustration {
    path: String,
    amount: u64
}

impl std::fmt::Display for Frustration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}: {})", self.path, self.amount)
    }
}
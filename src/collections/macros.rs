#[macro_export]
macro_rules! list {
    ($($x:expr),*) => {
        {
            use crate::collections::linked_list::List;
            let mut l = List::new();
            $(l.append($x);)*
            l
        }
    }
}
pub use list;

// macro_rules! map {
//     ($($k:expr => $v:expr),*) => {
//         {
//             use crate::collections::hash_map::Map;
//             let mut m = Map::new();
//             $(m.insert($k, $v);)*
//             m
//         }
//     }
// }
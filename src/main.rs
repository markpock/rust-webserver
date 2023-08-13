// use query_processor::indexer::{fileparser::parse, crawl_dir};
// use std::env::{args, current_dir};

// fn main() {
//     let args: Vec<String> = args().collect();
//     if args.len() != 2 {
//         eprintln!("Usage: ./main filename");
//         panic!()
//     }
//     let curr_dir: String = current_dir()
//         .unwrap()
//         .into_os_string()
//         .into_string()
//         .unwrap();
//     let read_dir: String = format!("{}/{}", curr_dir, args[1].clone());
//     println!("{:?}", crawl_dir(read_dir));
// }
fn main() {}
#![feature(arc_unwrap_or_clone)]

use collections::linked_list::*;
use collections::hash_map::*;

fn t_list() {
    let mut lst: List<i32> = list!{1, 2, 3, 4};
    println!("lst: {:?}", lst);
    println!("popped: {:?}", lst.pop());
}

fn main() {
    println!("Hello, world!");
    t_list()
    // let mut lst: List<i32> = list!{1, 2, 3, 4};
    // println!("lst: {}", lst);
    // lst = lst.iter().map(|x| x + 1).collect();
    // println!("lst: {}", lst);
    // println!("lst: {}", lst.iter().filter(|&x| *x > 2).collect::<List<_>>());
    // let mut map: Map<i32, i32> = map!{
    //     1 => 1,
    //     2 => 2,
    //     3 => 3
    // };
    // println!("map[1]: {:?}", map.get(1));
    // println!("map: {}", map);
    // println!("map: {}", Map::<i32, i32>::new());
    // println!("map: {}", map.iter().map(|(k, v)| (*k, v + 1)).collect::<Map<i32, i32>>());
    // println!("removed val: {:?}", map.delete(1));
    
    // println!("map: {}", map);
    // println!("removed val: {:?}", map.delete(2));
    // println!("map: {}", map);
    // println!("removed val: {:?}", map.delete(3));
    // println!("map: {}", map);
    // println!("len: {}", map.size());

    // map = map!{1 => 1, 2 => 3, 3 => 3, 4 => 5, 5 => 5};
    // println!("map: {}", map);
    // println!("removed: {:?}", map.remove(|&x, &y| x != y));
    // println!("map: {}", map);
}

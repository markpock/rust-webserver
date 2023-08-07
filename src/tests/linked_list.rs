use crate::collections::linked_list::*;

#[test]
fn macro_init() {
    let empty: List<i32> = list!();
    assert_eq!(empty.size(), 0);

    let elts = list!(1, 2, 3, 4, 5);
    assert_eq!(elts.size(), 5);
}

#[test]
fn adders_deleters() {
    let mut l = List::<i32>::new();
    assert_eq!(l.size(), 0);

    assert_eq!(l.pop(), None);
    assert_eq!(l.slice(), None);

    l.push(1);
    assert_eq!(l.size(), 1);
    assert_eq!(l.pop(), Some(1));

    l.push(1);
    l.push(2);
    assert_eq!(l.size(), 2);
    assert_eq!(l.pop(), Some(2));
    assert_eq!(l.pop(), Some(1));

    l.append(1);
    l.append(2);
    assert_eq!(l.pop(), Some(1));
    assert_eq!(l.pop(), Some(2));

    l.push(1);
    l.push(2);
    assert_eq!(l.slice(), Some(1));
    assert_eq!(l.slice(), Some(2));

    l.append(1);
    l.append(2);
    assert_eq!(l.slice(), Some(2));
    assert_eq!(l.slice(), Some(1));

    assert_eq!(l.pop(), None);
    assert_eq!(l.slice(), None);
}

#[test]
fn remove() {
    let mut l = list!(1, 2, 3, 4, 5);
    println!("{}", l);
    assert_eq!(l.remove(|&x| x > 3), Some(4));
}

#[test]
fn generator() {
    let l = list!(1, 2, 3, 4, 5);
    let g = l.generator();
}
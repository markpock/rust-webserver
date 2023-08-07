// A singly-linked list.

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
/// A doubly-linked list. Supports O(1) dequeue style operations and O(n)
/// remove.
pub struct List<T: Clone> {
    len: usize,
    refs: ListState<T>
}

#[derive(Debug, Clone)]
enum ListState<T> {
    Empty, 
    Dequeue {hd: Rc<RefCell<Node<T>>>,
             tl: Rc<RefCell<Node<T>>>}
}

use crate::linked_list::list::ListState::*;

#[derive(Debug)]
struct Node<T> {
    data: T,
    nxt: Option<Rc<RefCell<Node<T>>>>,
    prev: Weak<RefCell<Node<T>>> // Option not needed because all access through upgrade
}

impl<T: Clone> List<T> {
    pub fn new() -> Self { List {len: 0, refs: ListState::Empty} }

    pub fn size(&self) -> usize { self.len }

    pub fn push(&mut self, data: T) {
        match &mut self.refs {
            Empty => {
                let tmp = Rc::new(RefCell::new(Node{data, nxt: None, prev: Weak::new()}));
                *self = List {len: 1, refs: Dequeue{hd: tmp.clone(), tl: tmp.clone()}}
            }
            Dequeue {hd, tl: _} => {
                let tmp: Rc<RefCell<Node<T>>> = Rc::new(RefCell::new(Node{data, nxt: Some(hd.to_owned()), prev: Weak::new()}));
                hd.borrow_mut().prev = Rc::downgrade(&tmp);
                *hd = tmp;
                self.len += 1;
            }
        }
    }

    pub fn append(&mut self, data: T) {
        match &mut self.refs {
            Empty => self.push(data),
            Dequeue {hd: _, tl} => {
                let tmp = Rc::new(RefCell::new(Node{data, nxt: None, prev: Rc::downgrade(tl)}));
                tl.borrow_mut().nxt = Some(tmp.clone());
                *tl = tmp;
                self.len += 1;
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match &mut self.refs {
            Empty => None,
            Dequeue {hd, tl: _} => {
                let tmp = hd.borrow().data.clone();
                if self.len == 1 {
                    self.refs = Empty;
                } else {
                    let next = hd.borrow().nxt.clone().unwrap();
                    *hd = next;
                    hd.borrow_mut().prev = Weak::new();
                }
                self.len -= 1;
                Some(tmp)
            }
        }
    }

    pub fn slice(&mut self) -> Option<T> {
        match &mut self.refs {
            Empty => None,
            Dequeue {hd: _, tl} => {
                let tmp = tl.borrow().data.clone();
                if self.len == 1 {
                    self.refs = Empty;
                } else {
                    let prev = tl.borrow().prev.upgrade().unwrap();
                    *tl = prev;
                    tl.borrow_mut().nxt = None;
                }
                self.len -= 1;
                Some(tmp)
            }
        }
    }
    
    pub fn remove<F>(&mut self, pred: F) -> Option<T> where F: Fn(&T) -> bool {
        match self.refs.clone() {
            Empty => None,
            Dequeue {hd, tl} => {
                // First, check the dequeue end cases.
                if pred(&hd.borrow().data) { return self.pop() }
                if pred(&tl.borrow().data) { return self.slice() }
                // We guarantee that it is in the middle.
                let mut curr = Some(hd);
                while let Some(node) = curr {
                    if pred(&node.borrow().data) {
                        return Some(node.borrow().data.clone());
                    }
                    curr = node.borrow().nxt.clone();
                }
                None
            }
        }
    }
}

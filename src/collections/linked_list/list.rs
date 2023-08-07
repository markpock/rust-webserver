// A singly-linked list.

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
/// A doubly-linked list. Supports O(1) dequeue style operations and O(n)
/// remove. Elements must be cloneable.
pub struct List<T: Clone> {
    pub(super) len: usize,
    pub(super) refs: ListState<T>
}

#[derive(Debug, Clone)]
/// A small enum to capture that the list is either empty or has both a
/// head and tail we should keep track of.
pub(super) enum ListState<T> {
    Empty, 
    Dequeue {hd: Rc<RefCell<Node<T>>>,
             tl: Rc<RefCell<Node<T>>>}
}

use self::ListState::*;
use super::ListGenerator;

#[derive(Debug)]
/// Our node class.
pub(super) struct Node<T> {
    pub(super) data: T,
    pub(super) nxt: Option<Rc<RefCell<Node<T>>>>,
    pub(super) prev: Weak<RefCell<Node<T>>> // Option not needed because all access through upgrade
}

// Adder methods.
impl<T: Clone> List<T> {
    /// Creates a new, empty list.
    pub fn new() -> Self { List {len: 0, refs: ListState::Empty} }

    /// Returns the number of elements in this list.
    pub fn size(&self) -> usize { self.len }

    /// Adds an element to the head of this list.
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

    /// Adds an element to the tail of this list.
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
}

// Deleter methods.
impl<T: Clone> List<T> {
    /// Removes an element from the head of this list, returning the removed
    /// element - or None if the list is empty.
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

    /// Removes an element from the tail of this list, returning the removed
    /// element - or None if the list is empty.
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

    /// Removes the first element from the head of this list satisfying the
    /// given predicate, or None if there exists no such element. O(n) linear
    /// search - use with caution. Conversion to an iterator and filtering is
    /// preferred if removing in batch.
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

// Iterator methods.
impl<T: Clone> List<T> {
    pub(crate) fn generator(&self) -> ListGenerator<'_, T> {
        let node = match self.refs.clone() {
            Empty => None,
            Dequeue{hd, tl: _} => Some(hd)
        };
        ListGenerator {list: self, node}
    }
}

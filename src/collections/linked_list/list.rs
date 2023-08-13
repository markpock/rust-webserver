// A singly-linked list.

use std::{cell::RefCell, rc::{Rc, Weak}};

#[derive(Debug)]
/// A doubly-linked list. Supports O(1) dequeue style operations and O(n)
/// remove.
pub struct List<T> {
    pub(super) len: usize,
    pub(super) state: ListState<T>
}

#[derive(Debug)]
/// A small enum to capture that the list is either empty or has both a
/// head and tail we should keep track of.
pub(super) enum ListState<T> {
    Empty, 
    Dequeue {hd: Rc<RefCell<Node<T>>>,
             tl: Rc<RefCell<Node<T>>>}
}

use self::ListState::*;

#[derive(Debug)]
/// Our node class.
pub struct Node<T> {
    data: T,
    nxt: Option<Rc<RefCell<Node<T>>>>,
    prev: Weak<RefCell<Node<T>>> // Option not needed because all access through upgrade
}

impl<T> Node<T> {
    // Creates a new, lone node with no links.
    pub fn lone(data: T) -> Self { Node {data, nxt: None, prev: Weak::new() }}
    // Creates a node with a single link to a next node, as if it were the head of a list.
    pub fn head(data: T, next: &Rc<RefCell<Node<T>>>) -> Self { Node {data, nxt: Some(next.clone()), prev: Weak::new() }}
    // Creates a node with a single link to a previous noed, as if it were the tail of a list.
    pub fn tail(data: T, prev: &Rc<RefCell<Node<T>>>) -> Self { Node {data, nxt: None, prev: Rc::downgrade(prev)}}
    // Wraps a node in an Rc<RefCell<Node<T>>> so it can be used by the LinkedList API.
    pub fn wrap(node: Self) -> Rc<RefCell<Node<T>>> { Rc::new(RefCell::new(node)) }
    // Unwraps such a cell, assuming that there exists only one such cell. Panics if not.
    pub fn unwrap(refr: Rc<RefCell<Node<T>>>) -> Node<T> {
        match Rc::try_unwrap(refr) {
            Ok(val) => val.into_inner(),
            Err(_) => panic!()
        }
    }
}

// Adder methods.
impl<T: std::fmt::Debug> List<T> {
    /// Creates a new, empty list.
    pub fn new() -> Self { List {len: 0, state: ListState::Empty} }

    /// Returns the number of elements in this list.
    pub fn size(&self) -> usize { self.len }

    /// Adds an element to the head of this list.
    pub fn push(&mut self, data: T) {
        // Borrow self.state
        match &self.state {
            // Borrow should end inside each match case
            Empty => {
                let tmp = Node::wrap(Node::lone(data));
                // Now we can write to it
                self.state = Dequeue { hd: tmp.clone(), tl: tmp };
            }
            Dequeue { hd, tl } => {
                // We can't move hd, but we can clone it without cloning the data underneath.
                let stored_hd = hd.clone();
                let stored_tl = tl.clone();
                // Must do this to zero out reference count!! Yay!!
                let new_hd = Node::wrap(Node::head(data, hd));
                self.state = Empty;
                
                stored_hd.borrow_mut().prev = Rc::downgrade(&new_hd);
                new_hd.borrow_mut().nxt = Some(stored_hd);

                self.state = Dequeue { hd: new_hd, tl: stored_tl }
            }
        }
        self.len += 1
    }

    pub fn append(&mut self, data: T) {
        match &self.state {
            // Borrow should end inside each match case
            Empty => self.push(data),
            Dequeue { hd, tl } => {
                let stored_hd = hd.clone();
                let stored_tl = tl.clone();
                // Must do this to zero out reference count!! Yay!!
                let new_tl = Node::wrap(Node::tail(data, tl));
                self.state = Empty;
                
                stored_tl.borrow_mut().nxt = Some(new_tl.clone());
                new_tl.borrow_mut().prev = Rc::downgrade(&stored_tl);
                self.state = Dequeue { hd: stored_hd, tl: new_tl };

                self.len += 1
            }
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        match (&self.len, &self.state) {
            (_, Empty) => None,
            (1, Dequeue {hd, tl: _}) => {
                let stored = hd.clone();
                self.state = Empty;
                self.len -= 1;
                Some(Node::unwrap(stored).data)
            }
            (_, Dequeue {hd, tl}) => {
                let (stored_hd, stored_tl) = (hd.clone(), tl.clone());
                self.state = Empty;
                self.len -= 1;
                let old = Node::unwrap(stored_hd);
                if let Some(new_head) = old.nxt {
                    self.state = Dequeue { hd: new_head, tl: stored_tl }
                } else {
                    panic!();
                }
                Some(old.data)
            }
        }
    }

    pub fn slice(&mut self) -> Option<T> {
        match (&self.len, &self.state) {
            (_, Empty) => None,
            (1, Dequeue { hd: _, tl }) => {
                let stored = tl.clone();
                self.state = Empty;
                self.len -= 1;
                Some(Node::unwrap(stored).data)
            },
            (_, Dequeue {hd, tl}) => {
                let stored_hd = hd.clone();

                // prev is the 2nd to last
                let prev = tl.clone().borrow_mut().prev.upgrade().unwrap();

                // Drop hd, tl
                self.state = Empty;   
                
                // Now count of refs to tl is 2
                let ref_tl = prev.borrow().nxt.clone().unwrap();
                // Drop ref to tl from prev
                prev.borrow_mut().nxt = None;
                // tl should now have no references besides ref_tl!

                // Reset state and modify len
                self.state = Dequeue { hd: stored_hd, tl: prev };
                self.len -= 1;

                Some(Node::unwrap(ref_tl).data)
            }
        }
    }
}

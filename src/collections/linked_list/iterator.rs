use std::{cell::RefCell, rc::Rc};

use super::{List, Node};


impl<T: Clone> FromIterator<T> for List<T> {
    /// Creates a new list front-to-back from an iterator over raw items.
    fn from_iter<A>(iter: A) -> Self where A: IntoIterator<Item = T> {
        let mut l = List::new();
        for i in iter {
            l.append(i);
        }
        l
    }
}

#[derive(Debug)]
/// Clones items right out of a list.
pub struct ListGenerator<'a, T: Clone> {
    pub(super) list: &'a List<T>,
    pub(super) node: Option<Rc<RefCell<Node<T>>>>
}

impl<'a, T: Clone> Iterator for ListGenerator<'a, T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.node.clone() {
            None => None,
            Some(node) => {
                let result = node.borrow_mut().data.clone();
                self.node = node.borrow_mut().nxt.clone();
                Some(result)
            }
        }
    }


// #[derive(Debug)]
// pub struct ListIter<'a, T> {
//     iter: ListIterMut<'a, T>
// }


}

// #[derive(Debug)]
// pub struct ListConsumer<'a, T> {
//     node: Option<&'a Rc<RefCell<Node<T>>>>,
// }

// impl<'a, T: 'a> Iterator for ListIter<'a, T> {
//     type Item = &'a T;
//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//         //self.iter.next().as_deref()
//     }
// }

// impl<'a, T: 'a> Iterator for ListIterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.node {
//             None => None,
//             Some(node) => {
//                 todo!()
//             }
//         }
//     }
// }



// impl<'a, T> ListIterMut<'a, T> {
//     pub fn delete(&mut self) -> Option<T> {
//         if let Some(node) = self.node {
//             let result = node.borrow().data;
//             match (node.borrow().nxt, node.borrow().prev.upgrade()) {
//                 (Some(mut next), opt_prev) => {
//                     self.node = Some(&next);
//                     if let Some(mut prev) = opt_prev {
//                         prev.get_mut().nxt = Some(next);
//                         next.get_mut().prev = Rc::downgrade(&prev);
//                     } else {
//                         next.get_mut().prev = Weak::new();
//                     }
//                 },
//                 (None, Some(mut prev)) => {
//                     prev.get_mut().nxt = None;
//                     self.node = Some(&prev);
//                 }
//                 (None, None) => self.node = None
//             }
//             Some(result)
//         } else { None }
//     }
// }

// impl<'a, T: 'a> Iterator for ListIterMut<'a, T> {
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.node {
//             None => None,
//             Some(node) => {
//                 todo!()
//             }
//         }
//     }
// }




// impl<'a, T: 'a> Iterator for ListConsumer<'a, T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         match &mut self.node {
//             None => None,
//             Some(node) => {
//                 None
//             }
//         }
//     }
// }


// impl<'a, T: Clone> ListGenerator<'a, T> {
//     pub fn delete(&mut self) -> Option<T> {
//         let fin = match self.node.clone() {
//             None => None,
//             Some(node) => {
//                 let result = node.borrow_mut().data.clone();
//                 match (node.borrow().nxt.clone(), node.borrow().prev.upgrade()) {
//                     (Some(next), opt_prev) => {
//                         self.node = Some(next.clone());
//                         if let Some(prev) = opt_prev {
//                             prev.borrow_mut().nxt = Some(next.clone());
//                             next.borrow_mut().prev = Rc::downgrade(&prev);
//                         } else {
//                             next.borrow_mut().prev = Weak::new();
//                         }
//                     },
//                     (None, Some(prev)) => {
//                         prev.borrow_mut().nxt = None;
//                         self.node = Some(prev);
//                     }
//                     (None, None) => self.node = None
//                 }
//                 Some(result)
//             }
//         };
//         self.list.len -= 1;
//         if self.list.len == 0 {
//             self.list.refs = ListState::Empty;
//         }
//         fin
//     }
// }
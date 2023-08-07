use crate::linked_list::List;

impl<T: Clone> FromIterator<T> for List<T> {
    fn from_iter<A>(iter: A) -> Self where A: IntoIterator<Item = T> {
        let mut l = List::new();
        for i in iter {
            l.push(i);
        }
        l
    }
}

impl<T: Clone> List<T> {
    // pub fn iter<'a, 'b : 'a>(&self) -> ListIter<'a, T> {
    //     ListIter {iter: self.iter_mut()}
    // }

    // pub fn iter_mut<'a, 'b: 'a>(&'b self) -> ListIterMut<'a, T> {
    //     match &self.refs {
    //         Empty => ListIterMut { node: None },
    //         Dequeue {hd, tl: _} => ListIterMut { node: Some(&hd) }
    //     }
    // }
}



// #[derive(Debug)]
// pub struct ListIter<'a, T> {
//     iter: ListIterMut<'a, T>
// }

// #[derive(Debug)]
// pub struct ListIterMut<'a, T> {
//     node: Option<&'a Rc<RefCell<Node<T>>>>,
// }

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

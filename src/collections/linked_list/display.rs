// use std::{fmt::{Display, Formatter, Result}, clone::Clone};
// use super::List;

// // Display implementation.
// impl<T: Display + Clone + std::fmt::Debug> Display for List<T> {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         if let Err(x) = write!(f, "⟨") { return Err(x) }
//         for (i, e) in self.generator().enumerate() {
//             if let Err(x) = write!(f, "{}", e) { return Err(x) }
//             if i != (self.size() - 1).try_into().unwrap() {
//                 if let Err(x) = write!(f, ", ") { return Err(x) }
//             }
//         }
//         write!(f, "⟩");
//         write!(f, " My size is: {} ", self.size())
//     }
// }

pub mod naive_trie;
pub mod trie;

pub use trie::{Trie, TrieBuilder};

// use std::collections::HashMap;

// pub struct Trie {}

// impl Trie {
//     pub fn lookup(&self, word: &str) -> &str {
//         self.lookup_with_state(word, 0)
//     }
//     pub fn lookup_with_state(&self, word: &str, start_state: usize) -> &str {
//         let st = start_state;
//         ""
//     }
// }
// pub struct TrieBuilder {
//     trie: HashMap<usize, (HashMap<char, usize>, Vec<String>)>,
//     count: usize,
//     state: usize,
// }

// impl TrieBuilder {
//     pub fn new() -> Self {
//         let mut trie = HashMap::new();
//         trie.insert(0, (HashMap::new(), vec![]));
//         Self {
//             trie,
//             count: 0,
//             state: 0,
//         }
//     }

//     pub fn insert(&mut self, word: &str, decoration: &str) {
//         self.count += 1;
//         let mut st = 0;
//         // for i in 0..word.len() {
//         //     st = match self.trie.get(&st) {
//         //         Some(state) => match state.0.get(&word[i..i + 1]) {
//         //             Some(st) => *st,
//         //             None => {
//         //                 self.complete(st, &word[i..], decoration);
//         //                 return;
//         //             }
//         //         },
//         //         None => {
//         //             self.complete(st, &word[i..], decoration);
//         //             return;
//         //         }
//         //     }
//         // }
//     }

//     fn complete(&mut self, st: usize, word: &str, decoration: &str) {
//         for c in word.chars() {
//             self.state += 1;
//         }
//     }
//     pub fn build(self) -> Trie {
//         Trie {}
//     }

//     pub fn number_of_insertions(&self) -> usize {
//         self.count
//     }
// }
// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn can_build() {
//         let mut trie_builder = TrieBuilder::new();
//         trie_builder.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#);
//         assert_eq!(trie_builder.number_of_insertions(), 1);
//         let trie = trie_builder.build();

//         assert_eq!(trie.lookup("ösja"), r#"{"head":"ösja","pos":"vb"}"#);
//     }
// }

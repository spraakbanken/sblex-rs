use std::collections::HashMap;

use crate::{naive_trie::NaiveTrie, Trie};
use bytes::Bytes;

pub struct TrieBuilder {
    //<Label> {
    naive_trie: NaiveTrie, //<Label>,
    count: usize,
}

impl TrieBuilder {
    pub fn new() -> Self {
        Self {
            naive_trie: NaiveTrie::make_root(),
            count: 0,
        }
    }

    pub fn insert(&mut self, word: &str, decoration: Bytes) {
        self.count += 1;
        self.naive_trie.insert_bytes(word, decoration.clone())
    }

    pub fn number_of_insertions(&self) -> usize {
        self.count
    }

    pub fn build(&self) -> Trie {
        let trie_precomputed: HashMap<usize, (HashMap<char, usize>, Bytes)> = HashMap::new();
        let mut max_num_transitions = 0;
        for i in 0..self.naive_trie.num_children() {
            let state = self.naive_trie.children()[i];
            let mut tr = HashMap::new();
            for child in state.children() {
                tr.insert
            }
        }
        dbg!(&trie_precomputed);
        todo!("impl build")
    }
}

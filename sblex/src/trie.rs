use std::collections::HashMap;

use bytes::{BufMut, Bytes, BytesMut};

pub struct Trie {
    trie: HashMap<usize, (HashMap<char, usize>, Bytes)>,
}

impl Trie {
    fn new(trie: HashMap<usize, (HashMap<char, usize>, Bytes)>) -> Self {
        Self { trie }
    }
    pub fn builder() -> TrieBuilder {
        TrieBuilder::new()
    }

    pub fn lookup(&self, word: &str) -> Option<Bytes> {
        self.lookup_with_state(word, 0)
    }
    pub fn lookup_with_state(&self, word: &str, start_state: usize) -> Option<Bytes> {
        let mut st = start_state;
        for chr in word.chars() {
            st = match self.trie.get(&st) {
                Some(state) => match state.0.get(&chr) {
                    Some(st) => *st,
                    None => return None,
                },
                None => return None,
            };
        }
        Some(self.trie.get(&st).unwrap().1.clone())
    }
}
pub struct TrieBuilder {
    trie: HashMap<usize, (HashMap<char, usize>, Vec<Bytes>)>,
    count: usize,
    state: usize,
}

impl Default for TrieBuilder {
    fn default() -> Self {
        let mut trie = HashMap::new();
        trie.insert(0, (HashMap::new(), Vec::new()));
        Self {
            trie,
            count: 0,
            state: 0,
        }
    }
}

impl TrieBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert<D: Into<Bytes>>(&mut self, word: &str, decoration: D) {
        self.insert_bytes(word, decoration.into())
    }
    pub fn insert_bytes(&mut self, word: &str, decoration: Bytes) {
        self.count += 1;
        let mut st = 0;
        for (i, chr) in word.char_indices() {
            st = match self.trie.get(&st) {
                Some(state) => {
                    dbg!(state);
                    match state.0.get(&chr) {
                        Some(st) => *st,
                        None => {
                            self.complete(st, &word[i..], decoration);
                            return;
                        }
                    }
                    // todo!("state={:?}", state)
                }
                None => {
                    self.complete(st, &word[i..], decoration);
                    return;
                }
            }
        }
    }

    fn complete(&mut self, mut st: usize, word: &str, decoration: Bytes) {
        dbg!(&self.trie);
        for chr in word.chars() {
            self.state += 1;
            self.trie.entry(st).and_modify(|e| {
                e.0.insert(chr, self.state);
            });
            self.trie.insert(self.state, (HashMap::new(), Vec::new()));
            st = self.state;
        }
        self.trie.entry(st).and_modify(|e| {
            e.1.push(decoration.clone());
        });
    }

    pub fn number_of_insertions(&self) -> usize {
        self.count
    }

    pub fn build(self) -> Trie {
        let trie = self.precompute();
        Trie::new(trie)
    }

    fn precompute(mut self) -> HashMap<usize, (HashMap<char, usize>, Bytes)> {
        let mut trie_precomputed = HashMap::new();
        for i in 0..self.state + 1 {
            let (tr, dec) = self.trie.remove(&i).unwrap();
            // let ys = dec.join(b","[..]);
            let cont: String = tr.keys().collect();
            dbg!(&cont);
            let mut decoration = BytesMut::from(&b"{\"a\":["[..]);
            for (i, buf) in dec.iter().enumerate() {
                if i > 0 {
                    decoration.put(&b","[..]);
                }
                decoration.put(buf.as_ref());
            }
            // decoration.put(ys.as_bytes());
            decoration.put(&b"],\"c\":\""[..]);
            decoration.put(cont.as_bytes());
            decoration.put(&b"\"}"[..]);
            trie_precomputed.insert(i, (tr, decoration.freeze()));
        }
        trie_precomputed
        // todo!("trie_precomputed")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut trie_builder = Trie::builder();
        trie_builder.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#.as_bytes());
        dbg!(&trie_builder.trie);
        assert_eq!(trie_builder.number_of_insertions(), 1);

        let trie = trie_builder.build();
        dbg!(&trie.trie);
        let expected = r#"{"a":[{"head":"ösja","pos":"vb"}],"c":""}"#;
        assert_eq!(trie.lookup("ösja").unwrap(), expected);
    }
}

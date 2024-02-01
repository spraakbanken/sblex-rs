use hashbrown::HashMap;

use arcstr::ArcStr;
use unicode_segmentation::{Graphemes, UnicodeSegmentation};

type StringIntMap = HashMap<ArcStr, usize>;
// type StringIntMap = BTreeMap<ArcStr, usize>;
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Trie {
    trie: HashMap<usize, (HashMap<String, usize>, String)>,
    // trie: HashMap<usize, (HashMap<String, usize>, ArcStr)>,
}

impl Trie {
    pub fn builder() -> TrieBuilder {
        TrieBuilder::default()
    }

    pub fn lookup(&self, word: &str) -> Option<&str> {
        self.lookup_with_state(word, 0)
    }
    pub fn lookup_with_state(&self, word: &str, start_state: usize) -> Option<&str> {
        let mut st = start_state;
        for c in word.graphemes(true) {
            st = match self.trie.get(&st) {
                Some(tuple) => match tuple.0.get(c) {
                    Some(state) => *state,
                    None => return None,
                },
                None => return None,
            };
        }
        self.trie.get(&st).map(|x| x.1.as_str())
    }
}

#[derive(Debug)]
pub struct TrieBuilder {
    count: usize,
    state: usize,
    trie: HashMap<usize, (HashMap<String, usize>, Vec<String>)>,
}

impl Default for TrieBuilder {
    fn default() -> Self {
        let mut trie = HashMap::new();
        trie.insert(0, (HashMap::default(), Vec::default()));
        TrieBuilder {
            count: 0,
            state: 0,
            trie,
        }
    }
}
impl TrieBuilder {
    pub fn build(self) -> Trie {
        let mut trie = HashMap::new();
        for i in 0..=self.state {
            // println!("build: i={i}");
            let tr_dec = self.trie.get(&i).expect("state exist");
            let tr = tr_dec.0.clone();
            let cont = tr.keys().map(|s| &**s).collect::<Vec<_>>().join("");
            // println!("build: cont = {cont}");
            let ys = tr_dec.1.iter().map(|s| &**s).collect::<Vec<_>>().join(",");
            // println!("build: ys = {ys}");
            // let value = ArcStr::from(format!(r#"{{"a":[{ys}],"c":"{cont}"}}"#));
            let value = format!(r#"{{"a":[{ys}],"c":"{cont}"}}"#);
            trie.entry(i).insert((tr, value));
        }
        // println!("trie = {trie:?}");
        Trie { trie }
    }

    pub fn insert<S: Into<String>>(&mut self, word: &str, decoration: S) {
        // println!("insert: {word} in self.trie = {:?}", self.trie);
        self.count += 1;
        let mut st = 0;
        let mut iter = word.graphemes(true);
        loop {
            // for c in word.graphemes(true) {
            let curr_iter = iter.clone();
            let c = match iter.next() {
                Some(c) => c,
                None => break,
            };
            // println!("insert: c={c}");
            // println!("insert: st={st}");
            st = match self.trie.get(&st) {
                Some(tuple) => match tuple.0.get(c) {
                    Some(state) => *state,
                    None => {
                        return self.complete(st, curr_iter, decoration.into());
                    }
                },

                None => todo!(),
            };
        }
        self.trie
            .entry(st)
            .and_modify(|e| e.1.push(decoration.into()));
    }

    // create a new branch
    fn complete(&mut self, mut st: usize, word: Graphemes, decoration: String) {
        // println!("complete: st = {}, word = {}", st, word.as_str());
        for c in word {
            self.state += 1;
            self.trie
                .get_mut(&st)
                .expect("st exists")
                .0
                .entry(c.to_string())
                .insert(self.state);
            // {
            //     Some(place) => *place = self.state,
            //     None => unreachable!()
            // }
            self.trie
                .entry(self.state)
                .insert((HashMap::default(), Vec::default()));
            st = self.state;
        }
        self.trie.entry(st).and_modify(|e| e.1.push(decoration));
    }
    pub fn number_of_insertions(&self) -> usize {
        self.count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_create_trie() {
        let _trie = Trie::builder().build();
    }

    #[test]
    fn can_build() {
        let mut trie_builder = TrieBuilder::default();
        trie_builder.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#);
        dbg!(&trie_builder);
        assert_eq!(trie_builder.number_of_insertions(), 1);
        let trie = trie_builder.build();

        assert_eq!(trie.lookup("ösj"), Some(r#"{"a":[],"c":"a"}"#));
        assert_eq!(
            trie.lookup("ösja"),
            Some(r#"{"a":[{"head":"ösja","pos":"vb"}],"c":""}"#)
        );
    }
}

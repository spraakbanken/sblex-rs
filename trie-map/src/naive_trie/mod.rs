// pub enum NaiveTrie { //<Label> {
// Root(Box<NaiveTrieRoot<Label>>),
// IntermOrLeaf(Box<NaiveTrieInteromOrLeaf<Label>>),
// }

use bytes::Bytes;
use core::num;

// pub struct NaiveTrieRoot<Label> {
// children:
// }
#[derive(Debug)]
pub enum NaiveTrie {
    Root(Box<NaiveTrieRoot>),
    IntermOrLeaf(Box<NaiveTrieInteromOrLeaf>),
}

impl NaiveTrie {
    pub fn make_root() -> Self {
        NaiveTrie::Root(Box::new(NaiveTrieRoot { children: vec![] }))
    }

    pub fn make_interm_or_leaf(label: char, is_terminal: bool, decoration: Option<Bytes>) -> Self {
        NaiveTrie::IntermOrLeaf(Box::new(NaiveTrieInteromOrLeaf {
            label,
            children: vec![],
            is_terminal,
            decoration,
        }))
    }

    pub fn insert<D: Into<Bytes>>(&mut self, word: &str, decoration: D) {
        self.insert_bytes(word, decoration.into())
    }
    pub fn insert_bytes(&mut self, word: &str, decoration: Bytes) {
        let mut trie = self;
        let num_chars = word.chars().count();
        for (i, chr) in word.chars().enumerate() {
            let res = {
                trie.children()
                    .binary_search_by_key(&chr, |child| child.label())
            };
            match res {
                Ok(j) => todo!("ok"),
                Err(j) => {
                    let is_terminal = i == num_chars - 1;
                    let decoration_opt = if is_terminal {
                        Some(decoration.clone())
                    } else {
                        None
                    };
                    let child_trie =
                        Box::new(Self::make_interm_or_leaf(chr, is_terminal, decoration_opt));
                    trie = match trie {
                        NaiveTrie::Root(node) => {
                            node.children.insert(j, child_trie);
                            &mut node.children[j]
                        }
                        NaiveTrie::IntermOrLeaf(node) => {
                            node.children.insert(j, child_trie);
                            &mut node.children[j]
                        }
                        _ => panic!("Unexpected type"),
                    };
                }
            };
        }
    }

    pub fn children(&self) -> &[Box<Self>] {
        match self {
            Self::Root(node) => &node.children,
            Self::IntermOrLeaf(node) => &node.children,
            _ => panic!("unexpected type"),
        }
    }

    pub fn num_children(&self) -> usize {
        match self {
            Self::Root(node) => node.children.len(),
            Self::IntermOrLeaf(node) => node.children.len(),
            _ => panic!("unexpected type"),
        }
    }

    pub fn label(&self) -> char {
        match self {
            Self::IntermOrLeaf(node) => node.label.clone(),
            _ => panic!("unexpected type"),
        }
    }
}

#[derive(Debug)]
pub struct NaiveTrieRoot {
    children: Vec<Box<NaiveTrie>>,
}

#[derive(Debug)]
pub struct NaiveTrieInteromOrLeaf {
    label: char,
    children: Vec<Box<NaiveTrie>>,
    is_terminal: bool,
    decoration: Option<Bytes>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn building_naive_trie() {
        let mut root = NaiveTrie::make_root();
        dbg!(&root);
        root.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#);
        dbg!(&root);
        assert_eq!(root.num_children(), 0);
    }
}

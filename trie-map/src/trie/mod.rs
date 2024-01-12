#![allow(clippy::module_inception)]
mod trie;
mod trie_builder;

pub use self::trie::Trie;
pub use self::trie_builder::TrieBuilder;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_build() {
        let mut trie_builder = TrieBuilder::new();
        trie_builder.insert("ösja", r#"{"head":"ösja","pos":"vb"}"#.into());
        assert_eq!(trie_builder.number_of_insertions(), 1);
        // let trie = trie_builder.build();

        // assert_eq!(trie.lookup("ösja"), r#"{"head":"ösja","pos":"vb"}"#);
    }
}

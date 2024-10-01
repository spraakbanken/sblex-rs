pub mod error;
pub mod morphology;
pub mod trie;

pub use error::Error;
pub use morphology::Morphology;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

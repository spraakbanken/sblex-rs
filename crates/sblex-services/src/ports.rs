use crate::models::{lemma::Lemma, lexeme::Lexeme, lookup::LookupError};

pub trait SblexService: Clone + Send + Sync + 'static {
    fn lookup_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn lookup_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
    fn lookup_morph(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError>;
    fn lookup_morph_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError>;
}

pub trait LookupLid: Clone + Send + Sync + 'static {
    fn get_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn get_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
}

pub trait Morphology: Clone + Send + Sync + 'static {
    fn lookup(&self, fragment: &str) -> Result<Option<Vec<u8>>, LookupError>;
    fn lookup_with_cont(&self, fragment: &str) -> Result<Vec<u8>, LookupError>;
}

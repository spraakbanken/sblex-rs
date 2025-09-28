use crate::models::{lemma::Lemma, lexeme::Lexeme, lookup::LookupError};

pub trait SblexService: Clone + Send + Sync + 'static {
    fn lookup_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn lookup_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
}

pub trait LookupLid: Clone + Send + Sync + 'static {
    fn get_lemma(&self, lid: &str) -> Result<Option<Lemma>, LookupError>;
    fn get_lexeme(&self, lid: &str) -> Result<Option<Lexeme>, LookupError>;
}

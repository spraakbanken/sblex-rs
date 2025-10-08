use crate::errors::BoxDynError;

use super::{
    lemma::{is_lemma_id, LemmaId},
    lexeme::{is_saldo_id, SaldoId},
};

#[derive(Clone, Debug, serde::Serialize)]
pub enum SaldoOrLemmaId {
    SaldoId(SaldoId),
    LemmaId(LemmaId),
}

impl<'de> serde::Deserialize<'de> for SaldoOrLemmaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if is_lemma_id(s.as_str()) {
            return Ok(Self::LemmaId(LemmaId::new(s)));
        }
        if is_saldo_id(s.as_str()) {
            return Ok(Self::SaldoId(SaldoId::new(s)));
        }
        Err(serde::de::Error::custom(format!(
            "'{}' is not a lexeme nor a lemma",
            s
        )))
    }
}
#[derive(Debug, thiserror::Error, miette::Diagnostic)]
pub enum LookupError {
    #[error("Unknown error")]
    Unknown(#[source] BoxDynError),
}

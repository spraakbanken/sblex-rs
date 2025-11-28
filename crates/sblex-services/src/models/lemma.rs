use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Lemma {
    p: String,
    gf: String,
    l: Vec<String>,
}

impl Lemma {
    pub fn new(p: String, gf: String, l: Vec<String>) -> Self {
        Self { p, gf, l }
    }

    pub fn into_lexemes(self) -> Vec<String> {
        self.l
    }
}
#[derive(Clone, Debug, serde::Serialize)]
// #[serde(try_from="FromStr")]
// #[serde]
pub struct LemmaId(String);
pub fn is_lemma_id(s: &str) -> bool {
    match s.rfind('.') {
        None => false,
        Some(i) => s[(i - 1)..i] != *".",
    }
}
impl LemmaId {
    /// Precondition: is_lemma_id(s.as_str())
    pub(crate) fn new(s: String) -> Self {
        debug_assert!(is_lemma_id(s.as_str()));
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}
impl FromStr for LemmaId {
    type Err = BadLemmaId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match is_lemma_id(s) {
            true => Ok(Self(s.into())),
            false => Err(BadLemmaId(s.into())),
        }
    }
}

#[derive(Debug)]
pub struct BadLemmaId(String);

impl Display for BadLemmaId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("'{}' is not a lemma-id", self.0))
    }
}

impl std::error::Error for BadLemmaId {}

impl<'de> serde::Deserialize<'de> for LemmaId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match is_lemma_id(s.as_str()) {
            true => Ok(Self(s)),
            false => Err(serde::de::Error::custom(BadLemmaId(s))),
        }
    }
}

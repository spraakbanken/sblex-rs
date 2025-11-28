use std::{fmt::Display, str::FromStr};

#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize)]
pub struct Lexeme {
    pub lex: String,
    pub fm: String,
    pub fp: String,
    pub mf: Vec<String>,
    pub pf: Vec<String>,
    pub l: Vec<String>,
    pub path: Vec<String>,
    pub ppath: Vec<Vec<String>>,
}

impl Lexeme {
    pub fn new(
        lex: String,
        fm: String,
        fp: String,
        mf: Vec<String>,
        pf: Vec<String>,
        l: Vec<String>,
        path: Vec<String>,
        ppath: Vec<Vec<String>>,
    ) -> Self {
        Self {
            lex,
            fm,
            fp,
            mf,
            pf,
            l,
            path,
            ppath,
        }
    }
}

#[derive(Clone, Debug, serde::Serialize)]
// #[serde(try_from="FromStr")]
// #[serde]
pub struct SaldoId(String);
pub type LexemeId = SaldoId;

impl SaldoId {
    /// precondition: is_saldo_id(s.as_str())
    pub(crate) fn new(s: String) -> Self {
        debug_assert!(is_saldo_id(s.as_str()));
        Self(s)
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

pub fn is_saldo_id(s: &str) -> bool {
    match s.rfind('.') {
        None => false,
        Some(i) => s[(i - 1)..i] == *".",
    }
}

impl FromStr for SaldoId {
    type Err = BadSaldoId;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match is_saldo_id(s) {
            true => Ok(Self(s.into())),
            false => Err(BadSaldoId(s.into())),
        }
    }
}

#[derive(Debug)]
pub struct BadSaldoId(String);

impl Display for BadSaldoId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("'{}' is not a saldo-id", self.0))
    }
}

impl std::error::Error for BadSaldoId {}

impl<'de> serde::Deserialize<'de> for SaldoId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match is_saldo_id(s.as_str()) {
            true => Ok(Self(s)),
            false => Err(serde::de::Error::custom(BadSaldoId(s))),
        }
    }
}

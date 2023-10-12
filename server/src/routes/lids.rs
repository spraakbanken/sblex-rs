use std::str::FromStr;

use axum::body::{Bytes, Full};
use axum::http::{header, HeaderValue};
use axum::response::{Html, IntoResponse};
use axum::{extract::Path, response::Response};

use axum::http::StatusCode;
use serde::{de, Deserialize, Serialize};
use serde_json::json;

#[derive(Clone, Debug, Serialize)]
// #[serde(try_from="FromStr")]
// #[serde]
pub struct Lexeme(String);
pub fn is_lexeme(s: &str) -> bool {
    match s.rfind(".") {
        None => false,
        Some(i) => s[(i - 1)..i] == *".",
    }
}
impl FromStr for Lexeme {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match is_lexeme(s) {
            true => Ok(Self(s.into())),
            false => Err(format!("'{}' is not a lexeme", s)),
        }
    }
}
impl<'de> Deserialize<'de> for Lexeme {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match is_lexeme(s.as_str()) {
            true => Ok(Self(s)),
            false => Err(de::Error::custom(format!("'{}' is not a lexeme", s))),
        }
    }
}

#[derive(Clone, Debug, Serialize)]
// #[serde(try_from="FromStr")]
// #[serde]
pub struct Lemma(String);
pub fn is_lemma(s: &str) -> bool {
    match s.rfind(".") {
        None => false,
        Some(i) => s[(i - 1)..i] != *".",
    }
}
impl FromStr for Lemma {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match is_lemma(s) {
            true => Ok(Self(s.into())),
            false => Err(format!("'{}' is not a lemma", s)),
        }
    }
}
impl<'de> Deserialize<'de> for Lemma {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match is_lemma(s.as_str()) {
            true => Ok(Self(s)),
            false => Err(de::Error::custom(format!("'{}' is not a lemma", s))),
        }
    }
}

#[derive(Clone, Debug, Serialize)]

pub enum LexemeLemma {
    Lexeme(Lexeme),
    Lemma(Lemma),
}

impl<'de> Deserialize<'de> for LexemeLemma {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if is_lemma(s.as_str()) {
            return Ok(Self::Lemma(Lemma(s)));
        }
        if is_lexeme(s.as_str()) {
            return Ok(Self::Lexeme(Lexeme(s)));
        }
        Err(de::Error::custom(format!(
            "'{}' is not a lexeme nor a lemma",
            s
        )))
    }
}

pub async fn lookup_lid_json(Path(lid): Path<LexemeLemma>) -> impl IntoResponse {
    axum::Json(json!({"lid": lid}))
}

pub struct Xml<T>(pub T);

impl<T> IntoResponse for Xml<T>
where
    T: Into<Full<Bytes>>,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::XML.as_ref()),
            )],
            self.0.into(),
        )
            .into_response()
    }
}

pub async fn lookup_lid_xml(Path(lid): Path<LexemeLemma>) -> impl IntoResponse {
    Xml("<r></r>")
}

pub async fn lookup_lid_html(Path(lid): Path<LexemeLemma>) -> impl IntoResponse {
    Html("<r></r>")
}

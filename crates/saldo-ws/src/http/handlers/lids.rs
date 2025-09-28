use askama::Template;
use axum::extract::State;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::Response;
use axum::response::{Html, IntoResponse};

use sblex_services::models::lemma::Lemma;
use sblex_services::models::lexeme::Lexeme;
use sblex_services::models::lookup::SaldoOrLemmaId;
use sblex_services::ports::SblexService;

use crate::http::error::ApiError;
use crate::http::extractors::Path;
use crate::http::responses::ApiSuccessJson;
use crate::http::AppState;

pub async fn lookup_lid_json<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(lid): Path<SaldoOrLemmaId>,
) -> Result<ApiSuccessJson<LexemeOrLemma>, ApiError> {
    match lid {
        SaldoOrLemmaId::SaldoId(lid) => match state.sblex_service.lookup_lexeme(lid.as_str())? {
            Some(lexeme) => Ok(ApiSuccessJson::new(
                StatusCode::OK,
                LexemeOrLemma::Lexeme(lexeme),
            )),
            None => Err(ApiError::NotFound(format!(
                "There is no lexeme '{}'",
                lid.as_str()
            ))),
        },
        SaldoOrLemmaId::LemmaId(lid) => match state.sblex_service.lookup_lemma(lid.as_str())? {
            Some(lemma) => Ok(ApiSuccessJson::new(
                StatusCode::OK,
                LexemeOrLemma::Lemma(lemma),
            )),
            None => Err(ApiError::NotFound(format!(
                "There is no lemma '{}'",
                lid.as_str()
            ))),
        },
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize)]
#[serde(untagged)]
pub enum LexemeOrLemma {
    Lexeme(Lexeme),
    Lemma(Lemma),
}

pub struct Xml<T>(pub T);

impl<T> IntoResponse for Xml<T>
where
    T: IntoResponse,
{
    fn into_response(self) -> Response {
        (
            [(
                header::CONTENT_TYPE,
                HeaderValue::from_static(mime::XML.as_ref()),
            )],
            self.0.into_response(),
        )
            .into_response()
    }
}

pub async fn lookup_lid_xml<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(_lid): Path<SaldoOrLemmaId>,
) -> impl IntoResponse {
    Xml("<r></r>")
}

pub async fn lookup_lid_html<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(lid): Path<SaldoOrLemmaId>,
) -> Result<Html<String>, ApiError> {
    match lid {
        SaldoOrLemmaId::SaldoId(lid) => match state.sblex_service.lookup_lexeme(lid.as_str())? {
            Some(lexeme) => {
                // let lexeme_template = SaldoLidLexemeTemplate {};
                // Ok(Html(lexeme_template.render().unwrap()))
                todo!();
            }
            None => Err(ApiError::NotFound(format!(
                "There is no lexeme '{}'",
                lid.as_str()
            ))),
        },
        SaldoOrLemmaId::LemmaId(lid) => match state.sblex_service.lookup_lemma(lid.as_str())? {
            Some(lemma) => {
                // let lemma_template = SaldoTableTemplate {};
                // Ok(Html(lemma_template.render().unwrap()))
                todo!();
            }
            None => Err(ApiError::NotFound(format!(
                "There is no lemma '{}'",
                lid.as_str()
            ))),
        },
    }
}

//
// #[derive(askama::Template)]
// #[template(path = "saldo_table.html")]
// struct SaldoTableTemplate {}
//
// #[derive(askama::Template)]
// #[template(path = "saldo_lid_lexeme.html")]
// struct SaldoLidLexemeTemplate {}

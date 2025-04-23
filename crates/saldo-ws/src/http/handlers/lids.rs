use axum::extract::State;
use axum::http::{header, HeaderValue, StatusCode};
use axum::response::{Html, IntoResponse};
use axum::{extract::Path, response::Response};

use sblex_services::models::lemma::Lemma;
use sblex_services::models::lexeme::Lexeme;
use sblex_services::models::lookup::SaldoOrLemmaId;
use sblex_services::ports::SblexService;
use serde_json::json;

use crate::http::responses::{ApiError, ApiSuccess};
use crate::http::AppState;

pub async fn lookup_lid_json<S: SblexService>(
    State(state): State<AppState<S>>,
    Path(lid): Path<SaldoOrLemmaId>,
) -> Result<ApiSuccess<LexemeOrLemma>, ApiError> {
    match lid {
        SaldoOrLemmaId::SaldoId(lid) => match state.sblex_service.lookup_lexeme(lid.as_str())? {
            Some(lexeme) => Ok(ApiSuccess::json(
                StatusCode::OK,
                LexemeOrLemma::Lexeme(lexeme),
            )),
            None => Err(ApiError::NotFound(format!(
                "There is no lexeme '{}'",
                lid.as_str()
            ))),
        },
        SaldoOrLemmaId::LemmaId(lid) => match state.sblex_service.lookup_lemma(lid.as_str())? {
            Some(lemma) => Ok(ApiSuccess::json(
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

pub async fn lookup_lid_xml(Path(_lid): Path<SaldoOrLemmaId>) -> impl IntoResponse {
    Xml("<r></r>")
}

pub async fn lookup_lid_html(Path(_lid): Path<SaldoOrLemmaId>) -> impl IntoResponse {
    Html("<r></r>")
}

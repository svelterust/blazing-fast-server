use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

// Error type
#[derive(Debug)]
pub struct Error(pub anyhow::Error);

impl<E: Into<anyhow::Error>> From<E> for Error {
    fn from(error: E) -> Self {
        Error(error.into())
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let error = json!({
            "error": format!("{:?}", self.0)
        });
        (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
    }
}

// Exports
pub use anyhow::anyhow as error;
pub type Result<T> = anyhow::Result<T, Error>;

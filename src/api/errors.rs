use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub struct AppError(pub anyhow::Error, pub StatusCode);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.1, format!("Internal Server Error: {}", self.0)).into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into(), StatusCode::INTERNAL_SERVER_ERROR)
    }
}

use axum::{
    body::Body, http::{header, StatusCode}, response::{IntoResponse, Response}
};

#[derive(Debug)]
pub struct AppError(pub anyhow::Error, pub StatusCode);

// Tell axum how to convert `AppError` into a response.
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log the error.
        tracing::error!("{}", self.0);
        if self.1 == StatusCode::BAD_REQUEST {
            (self.1, format!("Bad Request: {}", self.0)).into_response()
        } else {
            (self.1, format!("Internal Server Error: {}", self.0)).into_response()
        }
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

#[derive(Debug)]
pub struct ValidationErrors(pub anyhow::Error);
impl<E> From<E> for ValidationErrors
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
impl IntoResponse for ValidationErrors {
    fn into_response(self) -> Response {
        // Log the error.
        tracing::error!("{}", self.0);

        Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(self.0.to_string()))
            .unwrap()
    }
}

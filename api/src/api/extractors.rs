use anyhow::Error;
use axum::{
    async_trait, extract::{rejection::JsonRejection, FromRequest, Request}, Json
};
use serde::de::DeserializeOwned;
use validator::Validate;


#[derive(Debug, Clone, Copy, Default)]
pub struct ValidateJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
    Json<T>: FromRequest<S, Rejection = JsonRejection>,
{
    type Rejection = crate::api::errors::ValidationErrors;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate().map_err(|err: validator::ValidationErrors| {
            crate::api::errors::ValidationErrors(Error::msg(err.to_string()))
        })?;
        Ok(ValidateJson(value))
    }
}




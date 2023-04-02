use std::fmt::Debug;

use axum::{
  async_trait,
  extract::FromRequest,
  response::{IntoResponse, Response},
  Json, RequestExt,
};
use hyper::{Request, StatusCode};
use validator::Validate;

pub struct ValidatedJson<J>(pub J);

#[async_trait]
impl<S, B, J> FromRequest<S, B> for ValidatedJson<J>
where
  B: Send + 'static,
  S: Send + Sync,
  J: Validate + 'static,
  Json<J>: FromRequest<(), B>,
{
  type Rejection = ValidationError;

  async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
    let Json(data) = req
      .extract::<Json<J>, _>()
      .await
      .map_err(|err| ValidationError::ParsingError(err.into_response()))?;
    data
      .validate()
      .map_err(|err| ValidationError::ValidationError(err))?;
    Ok(Self(data))
  }
}

#[derive(Debug)]
pub enum ValidationError {
  ValidationError(validator::ValidationErrors),
  ParsingError(Response),
}

impl IntoResponse for ValidationError {
  fn into_response(self) -> Response {
    match self {
      ValidationError::ValidationError(x) => {
        let errs = x
          .errors()
          .iter()
          .map(|(field, err)| match err {
            validator::ValidationErrorsKind::Field(x) => format!(
              "field:{} {}",
              field,
              x.iter()
                .map(|m| m.message.as_ref().unwrap_or(&m.code).to_string())
                .collect::<Vec<String>>()
                .join("\n ")
            ),
            _ => {
              format!("{:?}", x)
            }
          })
          .collect::<Vec<String>>()
          .join(", ");
        (StatusCode::BAD_REQUEST, errs).into_response()
      }
      ValidationError::ParsingError(err) => err,
    }
    .into_response()
  }
}

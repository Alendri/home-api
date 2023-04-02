use std::collections::HashMap;

use axum::{
  extract::{Path, Query},
  Json,
};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};

use crate::deserializers::{empty_and_zero_length_string_as_none, empty_string_as_none};

#[derive(Debug, Deserialize)]
#[allow(dead_code)]

pub struct QueryParsingParams {
  foo: Option<i32>,
  bar: Option<String>,
}
pub async fn query_parsing(Query(params): Query<QueryParsingParams>) -> String {
  format!("{:?}", params)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QueryParsingWithNoneParams {
  foo: Option<i32>,
  #[serde(default, deserialize_with = "empty_string_as_none")]
  bar: Option<String>,
}
pub async fn query_parsing_with_none(Query(params): Query<QueryParsingWithNoneParams>) -> String {
  format!("{:?}", params)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct QueryParsingWithNoneAndZeroParams {
  foo: Option<i32>,
  #[serde(default, deserialize_with = "empty_and_zero_length_string_as_none")]
  bar: Option<String>,
}
pub async fn query_parsing_with_none_and_zero(
  Query(params): Query<QueryParsingWithNoneAndZeroParams>,
) -> String {
  format!("{:?}", params)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct ParseBodyReq {
  #[serde(default)]
  foo: i32,
  #[serde(default, deserialize_with = "empty_and_zero_length_string_as_none")]
  bar: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct ParseBodyRes {
  foo: i32,
  bar: Option<String>,
  path_id: String,
  path_dir: String,
}
#[derive(Debug, Serialize)]
pub struct ParseBodyWrapper {
  res: ParseBodyRes,
  as_rust: String,
}
pub async fn parse_body_and_path(
  Path(path): Path<HashMap<String, String>>,
  Json(payload): Json<ParseBodyReq>,
) -> (StatusCode, Json<ParseBodyWrapper>) {
  let res = ParseBodyRes {
    bar: payload.bar,
    foo: payload.foo,
    path_dir: path.get("dir").map(ToOwned::to_owned).unwrap(),
    path_id: path
      .get("id")
      .map(ToOwned::to_owned)
      .unwrap_or("No :id was provided in path.".to_owned()),
  };

  //This will be converted into a JSON response with a status code of `201 Created`.
  (
    StatusCode::CREATED,
    Json(ParseBodyWrapper {
      as_rust: format!("{:?}", res),
      res,
    }),
  )
}

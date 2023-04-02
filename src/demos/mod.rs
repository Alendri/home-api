mod db;
mod parsing;

use self::{
  db::{list_numbers, list_sums, upsert_db_and_sum},
  parsing::{
    parse_body_and_path, query_parsing, query_parsing_with_none, query_parsing_with_none_and_zero,
  },
};
use axum::{
  routing::{get, post, put},
  Router,
};
use sqlx::SqlitePool;

pub fn get_demo_router() -> Router<SqlitePool> {
  Router::new()
    .route("/upsert_db_and_sum", put(upsert_db_and_sum))
    .route("/list_numbers", get(list_numbers))
    .route("/list_sums", get(list_sums))
    .route("/query_parsing", get(query_parsing))
    .route("/query_parsing_with_none", get(query_parsing_with_none))
    .route(
      "/query_parsing_with_none_and_zero",
      get(query_parsing_with_none_and_zero),
    )
    .route("/parse_body_and_path/:dir", post(parse_body_and_path))
    .route("/parse_body_and_path/:dir/:id", post(parse_body_and_path))
}

use axum::{extract::State, Json};
use hyper::StatusCode;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use validator::Validate;

use crate::{deserializers::empty_and_zero_length_string_as_none, validation::ValidatedJson};

#[derive(Debug, Deserialize, Validate)]
pub struct UpsertDbAndSumReq {
  #[validate(length(min = 3, max = 12, message = "Must be between 3 and 12 characters."))]
  id: String,
  int: i32,
  float: f64,
  #[serde(default, deserialize_with = "empty_and_zero_length_string_as_none")]
  collection: Option<String>,
}
#[derive(Debug, Serialize)]
pub struct NumberSumRes {
  int_sum: i32,
  float_sum: f64,
}
impl From<NumberSumResOptional> for NumberSumRes {
  fn from(o: NumberSumResOptional) -> Self {
    NumberSumRes {
      int_sum: o.int_sum.unwrap_or_default(),
      float_sum: o.float_sum.unwrap_or_default(),
    }
  }
}

#[derive(Debug, Serialize)]
struct NumberSumResOptional {
  int_sum: Option<i32>,
  float_sum: Option<f64>,
}

///Example of validating JSON body request,
///upserting the database and returning result
///of another query.
pub async fn upsert_db_and_sum(
  State(db): State<SqlitePool>,
  ValidatedJson(payload): ValidatedJson<UpsertDbAndSumReq>,
) -> Result<(StatusCode, Json<NumberSumRes>), StatusCode> {
  let UpsertDbAndSumReq {
    id,
    int,
    float,
    collection,
  } = payload;

  if sqlx::query!(
    "INSERT INTO demo_numbers (
      id, int8, f64, collection
    ) VALUES (
      ?, ?, ?, ?
    ) ON CONFLICT(id) DO UPDATE SET
      int8=excluded.int8,
      f64=excluded.f64,
      collection=excluded.collection
    ",
    id,
    int,
    float,
    collection
  )
  .execute(&db)
  .await
  .is_err()
  {
    return Err(StatusCode::INTERNAL_SERVER_ERROR);
  };

  let sums = match sqlx::query_as!(
    NumberSumResOptional,
    "SELECT SUM(int8) as int_sum, TOTAL(f64) as float_sum FROM demo_numbers"
  )
  .fetch_one(&db)
  .await
  {
    Ok(res) => res,
    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
  };

  Ok((StatusCode::OK, Json(sums.into())))
}

#[derive(Serialize, Debug)]
pub struct ListNumbersRes {
  id: String,
  int: i64,
  float: f64,
  collection: Option<String>,
}
pub async fn list_numbers(
  State(db): State<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<ListNumbersRes>>), StatusCode> {
  let entries = match sqlx::query_as!(
    ListNumbersRes,
    "SELECT id, int8 as int, f64 as float, collection FROM demo_numbers"
  )
  .fetch_all(&db)
  .await
  {
    Ok(res) => res,
    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
  };
  Ok((StatusCode::OK, Json(entries)))
}

#[derive(Debug, Serialize)]
pub struct ListSumsRes {
  collection: Option<String>,
  int_sum: i32,
  float_sum: f64,
}
impl From<ListSumsResOptional> for ListSumsRes {
  fn from(o: ListSumsResOptional) -> Self {
    ListSumsRes {
      collection: o.collection,
      int_sum: o.int_sum.unwrap_or_default(),
      float_sum: o.float_sum.unwrap_or_default(),
    }
  }
}

#[derive(Debug, Serialize)]
struct ListSumsResOptional {
  collection: Option<String>,
  int_sum: Option<i32>,
  float_sum: Option<f64>,
}
pub async fn list_sums(
  State(db): State<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<ListSumsRes>>), StatusCode> {
  let sums: Vec<ListSumsRes> = match sqlx::query_as!(
    ListSumsResOptional,
    "SELECT collection, sum(int8) as int_sum, total(f64) as float_sum from demo_numbers GROUP BY collection"
  )
  .fetch_all(&db)
  .await
  {
    Ok(res) => res.into_iter().map(|r| Into::<ListSumsRes>::into(r)).collect(),
    Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
  };

  Ok((StatusCode::OK, Json(sums)))
}

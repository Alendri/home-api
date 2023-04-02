use anyhow::anyhow;
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;

// #[derive(FromRow, Debug)]
// struct Count {
//   count: i32,
// }

pub async fn init(clear: bool, create: bool) -> Result<(), anyhow::Error> {
  let db_url = &env::var("DATABASE_URL")?;

  if clear {
    _ = Sqlite::drop_database(&db_url).await;
    println!("Database dropped.");
  }

  if create {
    if Sqlite::database_exists(db_url).await.unwrap_or(false) {
      println!("Database found when trying to create, aborting.");
      return Err(anyhow!("Cannot create database. Database already exists."));
    }
    match Sqlite::create_database(db_url).await {
      Ok(_) => println!("Database created successfully."),
      Err(error) => return Err(anyhow!(error)),
    }
    let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;
    sqlx::query!(
      r#"
        CREATE TABLE IF NOT EXISTS people (
          name TEXT NOT NULL,
          last_seen TEXT NOT NULL
        ) STRICT;
        CREATE TABLE IF NOT EXISTS demo_numbers (
          id TEXT NOT NULL,
          int8 INTEGER NOT NULL DEFAULT 0,
          f64 REAL NOT NULL DEFAULT +0,
          collection TEXT
        ) STRICT;
        "#
    )
    .execute(&pool)
    .await?;
    println!("Created people table.");
    pool.close().await;
  }

  //TODO: Implement migration.

  Ok(())
}

pub async fn get_sqlite() -> Result<SqlitePool, anyhow::Error> {
  let pool = SqlitePool::connect(&env::var("DATABASE_URL")?).await?;

  // let b = sqlx::query_as!(
  //   Count,
  //   r#"SELECT count(*) as count FROM sqlite_schema WHERE type='table' AND name='people'"#
  // )
  // .fetch_one(&pool)
  // .await?;

  // println!("{:?}", b.count);

  Ok(pool)
}

// pub async fn insert_person(pool: &SqlitePool, name: String) -> Result<i64> {
//   let mut conn = pool.acquire().await?;

//   //Insert a person, then obtain the ID of this row
//   let id = sqlx::query!(
//     r#"
// INSERT INTO todos ( description )
// VALUES ( ?1 )
//         "#,
//     description
//   )
//   .execute(&mut conn)
//   .await?
//   .last_insert_rowid();

//   Ok(id)
// }

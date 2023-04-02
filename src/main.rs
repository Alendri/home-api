mod db_sqlite;
mod demos;
mod deserializers;
mod validation;

use axum::{routing::get, Router};
use clap::Parser;
use std::{fs::File, net::SocketAddr};

use crate::demos::get_demo_router;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
  /// Clears any existing database.
  ///
  /// Use with --create to reset and setup new database.
  #[arg(long, action)]
  clear: Option<bool>,

  /// Creates a new database.
  #[arg(long, action)]
  create: Option<bool>,
  //
  // TODO: Implement migrations.
  // Will run migrations found in ./migrations folder.
  // #[arg(long)]
  // migrate: Option<bool>,
}

#[tokio::main]
async fn main() {
  if load_env().is_err() {
    println!("No valid .env file found.");
  }
  let args = Cli::parse();
  println!("args:{:?}", args);
  let clear = args.clear.unwrap_or(false);
  let create = args.create.unwrap_or(false);

  if clear || create {
    match db_sqlite::init(clear, create).await {
      Err(e) => panic!("{:?}", e),
      _ => println!("Completed database actions. Exiting."),
    };
    return ();
  }

  let pool = match db_sqlite::get_sqlite().await {
    Ok(sqlite_pool) => sqlite_pool,
    Err(e) => panic!("Error: {}", e),
  };

  let writer = File::create("api_log.json").unwrap();
  // initialize tracing
  let subscriber = tracing_subscriber::fmt()
    // Use a more compact, abbreviated log format
    .compact()
    // Display source code file paths
    .with_file(true)
    // Display source code line numbers
    .with_line_number(true)
    // Display the thread ID an event was recorded on
    .with_thread_ids(true)
    // Don't display the event's target (module path)
    .with_target(false)
    .json()
    .with_writer(writer)
    // Build the subscriber
    .finish();
  tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

  // build our application with a route
  let app = Router::new()
    // `GET /` goes to `root`
    .route("/", get(root))
    .nest("/demo", get_demo_router())
    .with_state(pool);

  // run our app with hyper
  // `axum::Server` is a re-export of `hyper::Server`
  let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
  tracing::info!("listening on {:?}", addr);
  axum::Server::bind(&addr)
    .serve(app.into_make_service())
    .await
    .unwrap();
}

async fn root() -> &'static str {
  "Hello, World!"
}

fn load_env() -> Result<(), anyhow::Error> {
  dotenvy::dotenv()?;

  Ok(())
}

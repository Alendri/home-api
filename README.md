# home-api
Try at making a REST api in Rust.

## Starting the api
### 1. Prepare database
- Run `cargo sqlx database setup`  
Which creates a local database file and runs migrations in `./migrations`.

OR 

- Run `cargo sqlx database create` to create the database and
- `cargo sqlx migrate run` to run migrations.

### 2. Launch the server
- Run `cargo run`


## Reset the database
When the migrations change during development use `cargo sqlx database reset` to re-create the database and run migrations.
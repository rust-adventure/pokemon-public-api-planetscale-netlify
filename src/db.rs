use std::env;

use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

pub async fn connect() -> Result<MySqlPool, sqlx::Error> {
    let database_url =
        env::var("DATABASE_URL").expect("a db url");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    Ok(pool)
}

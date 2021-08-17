use std::env;

mod models;
use ksuid::Ksuid;
use models::*;

use lamedh_http::{
    lambda::{lambda, Context, Error},
    IntoResponse, Request,
};
use sqlx::mysql::MySqlPoolOptions;

#[lambda(http)]
#[tokio::main]
async fn main(
    _: Request,
    _: Context,
) -> Result<impl IntoResponse, Error> {
    dbg!("in main");
    let database_url =
        env::var("DATABASE_URL").expect("a db url");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let row: Result<(Vec<u8>, String, u16, String), sqlx::Error> = sqlx::query_as("SELECT id, name, hp, GROUP_CONCAT(DISTINCT ability SEPARATOR ',') from pokemon_table inner join abilities_table on pokemon_table.id = abilities_table.pokemon_id GROUP BY name")
        .fetch_one(&pool).await;

    let things = row
        .iter()
        .map(|(id, name, hp, abilities)| {
            dbg!(id, name, hp, abilities);
            let id_string = std::str::from_utf8(id);
            dbg!(id_string);
        })
        .collect::<()>();

    // dbg!(results);
    Ok("boop")
}

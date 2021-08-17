use std::env;

mod models;
use ksuid::Ksuid;
use models::*;

use lamedh_http::{
    lambda::{lambda, Context, Error},
    IntoResponse, Request,
};
use serde::{Deserialize, Serialize};
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

    let rows = sqlx::query_as!(PokemonRow,"
    SELECT id, P.name, P.hp, abilities from pokemon_table P
    LEFT JOIN (
        SELECT name, GROUP_CONCAT(DISTINCT ability SEPARATOR ',') as abilities
        FROM pokemon_table
        INNER JOIN abilities_table
        ON pokemon_table.id = abilities_table.pokemon_id
        GROUP BY name
    ) A
    ON P.name = A.name
    ")
        .fetch_all(&pool).await?;

    // dbg!(results);
    Ok(serde_json::to_value(rows)?)
}

#[derive(Serialize, sqlx::FromRow)]
struct PokemonRow {
    id: Vec<u8>,
    name: String,
    hp: u16,
    abilities: Option<String>,
}

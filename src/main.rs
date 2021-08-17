use std::env;

mod models;
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

    // let results = pokemon_table
    //     .limit(5)
    //     .load::<PokemonDB>(&connection)
    //     .expect("Error loading pokemon");

    // dbg!(results);
    Ok("boop")
}

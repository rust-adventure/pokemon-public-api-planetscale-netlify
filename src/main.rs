#[macro_use]
extern crate diesel;

use std::env;

use diesel::prelude::*;

mod models;
mod schema;
use models::*;
use schema::*;

use lamedh_http::{
    lambda::{lambda, Context, Error},
    IntoResponse, Request,
};
use schema::pokemon_table::dsl::*;

#[lambda(http)]
#[tokio::main]
async fn main(
    _: Request,
    _: Context,
) -> Result<impl IntoResponse, Error> {
    dbg!("in main");
    let database_url =
        env::var("DATABASE_URL").expect("a db url");
    let connection = MysqlConnection::establish(
        &database_url,
    )
    .expect(&format!(
        "Error connecting to {}",
        database_url
    ));

    let results = pokemon_table
        .limit(5)
        .load::<PokemonDB>(&connection)
        .expect("Error loading pokemon");

    dbg!(results);
    Ok("boop")
}

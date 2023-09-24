use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, Response,
};
use serde::Serialize;
use serde_json::json;
use sqlx::{mysql::MySqlPoolOptions, MySql, Pool};
use std::env;
use std::sync::OnceLock;
use tracing::{error, info, instrument};
use upload_pokemon_data::PokemonId;

static POOL: OnceLock<Pool<MySql>> = OnceLock::new();

#[derive(Debug, sqlx::FromRow, Serialize)]
struct PokemonHp {
    id: PokemonId,
    name: String,
    hp: u16,
    legendary_or_mythical: bool,
}

#[instrument]
async fn function_handler(
    event: Request,
) -> Result<Response<Body>, Error> {
    let path = event.uri().path();
    let requested_pokemon = path.split("/").last();
    info!(requested_pokemon, "requested a pokemon");

    match requested_pokemon {
        None => todo!("this is a hard error, return 500"),
        Some("") => {
            error!("searched for empty pokemon");
            let error_message =
                serde_json::to_string(&json!({
                    "error": "searched for empty pokemon"
                }))?;
            let resp = Response::builder()
                .status(400)
                .header(CONTENT_TYPE, "application/json")
                .body(Body::Text(error_message))?;
            Ok(resp)
        }
        Some(pokemon_name) => {
            let result = sqlx::query_as!(
                PokemonHp,
                r#"
SELECT
    id as "id!: PokemonId",
    name,
    hp,
    legendary_or_mythical as "legendary_or_mythical!: bool"
FROM
    pokemon
WHERE
    slug = ?
"#,
                pokemon_name
            )
            .fetch_one(POOL.get().unwrap())
            .await?;

            let pokemon = serde_json::to_string(&result)?;
            let resp = Response::builder()
                .status(200)
                .header(CONTENT_TYPE, "application/json")
                .body(Body::Text(pokemon))?;
            Ok(resp)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    POOL.get_or_init(|| pool);

    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn accepts_apigw_request() {
        let database_url =
            env::var("DATABASE_URL").unwrap();

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        POOL.get_or_init(|| pool);

        let input = include_str!("apigw-request.json");

        let request = lambda_http::request::from_str(input)
            .expect("failed to create request");

        let response = function_handler(request)
            .await
            .expect("failed to handle request");
        assert_eq!(
            response.body(),
            &Body::Text(
                "{\"id\":\"2Vbj42oybesL4UeqponMuj2IPxt\",\"name\":\"Bulbasaur\",\"hp\":45,\"legendary_or_mythical\":false}"
                    .to_string()
            )
        );
    }

    #[tokio::test]
    async fn handles_empty_pokemon() {
        let database_url =
            env::var("DATABASE_URL").unwrap();

        let pool = MySqlPoolOptions::new()
            .max_connections(5)
            .connect(&database_url)
            .await
            .unwrap();

        POOL.get_or_init(|| pool);

        let input =
            include_str!("empty-pokemon-request.json");

        let request = lambda_http::request::from_str(input)
            .expect("failed to create request");

        let response = function_handler(request)
            .await
            .expect("failed to handle request");

        assert_eq!(
            response.body(),
            &Body::Text(
                "{\"error\":\"searched for empty pokemon\"}"
                    .to_string()
            )
        );
    }
}

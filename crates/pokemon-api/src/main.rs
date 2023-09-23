use lambda_http::{
    http::header::CONTENT_TYPE, run, service_fn, Body,
    Error, Request, Response,
};
use serde::Serialize;
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[derive(Debug, sqlx::FromRow, Serialize)]
struct PokemonHp {
    name: String,
    hp: u16,
}

async fn function_handler(
    _event: Request,
) -> Result<Response<Body>, Error> {
    let database_url = env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let result = sqlx::query_as!(
        PokemonHp,
        r#"SELECT name, hp from pokemon where slug = ?"#,
        "charmander"
    )
    .fetch_one(&pool)
    .await?;

    let pokemon = serde_json::to_string(&result)?;
    let resp = Response::builder()
        .status(200)
        .header(CONTENT_TYPE, "application/json")
        .body(Body::Text(pokemon))?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(function_handler)).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn accepts_apigw_request() {
        let input = include_str!("apigw-request.json");

        let request = lambda_http::request::from_str(input)
            .expect("failed to create request");

        let response = function_handler(request)
            .await
            .expect("failed to handle request");

        assert_eq!(
            response.body(),
            &Body::Text(
                "{\"name\":\"Charmander\",\"hp\":39}"
                    .to_string()
            )
        );
    }
}

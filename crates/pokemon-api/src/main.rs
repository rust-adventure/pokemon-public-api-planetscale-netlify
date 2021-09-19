use lambda_runtime::{handler_fn, Context, Error};
use serde::Serialize;
use serde_json::{json, Value};
use sqlx::mysql::MySqlPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("cold start");
    let processor = handler_fn(handler);
    lambda_runtime::run(processor).await?;
    Ok(())
}

#[derive(Debug, sqlx::FromRow, Serialize)]
struct PokemonHp {
    name: String,
    hp: u16,
}
async fn handler(
    _: Value,
    _: Context,
) -> Result<Value, Error> {
    println!("handler");
    let database_url = env::var("DATABASE_URL")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let result = sqlx::query_as!(
        PokemonHp,
        r#"SELECT name, hp FROM pokemon WHERE slug = ?"#,
        "charmander"
    )
    .fetch_one(&pool)
    .await?;

    Ok(json!({
        "body": serde_json::to_string(&result)?
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({});

        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .unwrap(),
            json!({
                "body": serde_json::to_string(
                    &PokemonHp{
                        name: String::from("Charmander"),
                        hp: 49
                    },
                ).unwrap()
            })
        )
    }
}

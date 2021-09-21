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
    event: Value,
    _: Context,
) -> Result<Value, Error> {
    println!("handler");
    let database_url = env::var("DATABASE_URL")?;

    let requested_pokemon = event["path"]
        .as_str()
        .and_then(|path| path.split("/").last());

    match requested_pokemon {
        Some("") => todo!("400"),
        None => todo!("500"),
        Some(pokemon_name) => {
            let pool = MySqlPoolOptions::new()
                .max_connections(5)
                .connect(&database_url)
                .await?;
            let result = sqlx::query_as!(
                PokemonHp,
                r#"SELECT name, hp FROM pokemon WHERE slug = ?"#,
                pokemon_name
            )
            .fetch_one(&pool)
            .await?;

            Ok(json!({
                "body": serde_json::to_string(&result)?
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({
            "path": "/api/pokemon/bulbasaur"
        });

        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .unwrap(),
            json!({
                "body": serde_json::to_string(
                    &PokemonHp{
                        name: String::from("Bulbasaur"),
                        hp: 45
                    },
                ).unwrap()
            })
        )
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: 400")]
    async fn handler_handles_empty_pokemon() {
        let event = json!({
            "path": "/api/pokemon//"
        });
        handler(event.clone(), Context::default())
            .await
            .unwrap();
    }

    #[tokio::test]
    #[should_panic(expected = "not yet implemented: 500")]
    async fn handler_panics_on_no_path() {
        let event = json!({});
        handler(event.clone(), Context::default())
            .await
            .unwrap();
    }
}

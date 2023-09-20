mod pokemon_csv;
use futures::{stream::FuturesUnordered, StreamExt};
use miette::{miette, IntoDiagnostic, WrapErr};
use pokemon_csv::*;
use sqlx::mysql::MySqlPoolOptions;
use std::{collections::HashMap, env, time::Duration};
mod db;
use db::*;
use indicatif::{ProgressBar, ProgressIterator};

#[tokio::main]
async fn main() -> miette::Result<()> {
    let database_url = env::var("DATABASE_URL").map_err(|e| {
        miette!(
            help="Run `pscale connect <database> <branch>` to get a connection",
            "{e}"
        )
    })
    .wrap_err("Must have a DATABASE_URL set")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(50)
        .acquire_timeout(Duration::from_secs(60 * 5))
        .connect(&database_url)
        .await
        .map_err(|e| {
            miette!(
                help="database urls must be in the form `mysql://username:password@host:port/database`",
                "{e}"
            )
        })?;

    let mut rdr = csv::Reader::from_path(
        "./crates/upload-pokemon-data/pokemon.csv",
    )
    .into_diagnostic()?;

    let pokemon = rdr
        .deserialize()
        .collect::<Result<Vec<PokemonCsv>, csv::Error>>()
        .into_diagnostic()?;

    let mut pokemon_map: HashMap<String, PokemonId> =
        HashMap::new();

    let mut tasks = FuturesUnordered::new();

    for record in pokemon.clone().into_iter().progress() {
        let pokemon_row: PokemonTableRow =
            record.clone().into();
        tasks.push(tokio::spawn(insert_pokemon(
            pool.clone(),
            pokemon_row.clone(),
        )));
        for ability in record.abilities.iter() {
            let pool = pool.clone();
            let pokemon_id = pokemon_row.id.clone();
            let ability = ability.clone();
            tasks.push(tokio::spawn(async move {
                sqlx::query!(
                    r#"
            INSERT INTO abilities (
                id, pokemon_id, ability
            ) VALUES (?, ?, ?)"#,
                    PokemonId::new(),
                    pokemon_id,
                    ability,
                )
                .execute(&pool)
                .await
            }));
        }
        for egg_group in record.egg_groups.iter() {
            let pool = pool.clone();
            let pokemon_id = pokemon_row.id.clone();
            let egg_group = egg_group.clone();
            tasks.push(tokio::spawn(async move {
                sqlx::query!(
                    r#"
            INSERT INTO egg_groups (
                id, pokemon_id, egg_group
            ) VALUES (?, ?, ?)"#,
                    PokemonId::new(),
                    pokemon_id,
                    egg_group,
                )
                .execute(&pool)
                .await
            }))
        }
        for typing in record.typing.iter() {
            let pool = pool.clone();
            let pokemon_id = pokemon_row.id.clone();
            let typing = typing.clone();
            tasks.push(tokio::spawn(async move {
                sqlx::query!(
                    r#"
            INSERT INTO typing (
                id, pokemon_id, typing
            ) VALUES (?, ?, ?)"#,
                    PokemonId::new(),
                    pokemon_id,
                    typing,
                )
                .execute(&pool)
                .await
            }))
        }
        pokemon_map.insert(record.name, pokemon_row.id);
    }

    for pokemon in pokemon
        .into_iter()
        .progress()
        .filter(|pokemon| pokemon.evolves_from.is_some())
    {
        let name = pokemon.evolves_from.expect(
            "Expected a value here since we just checked",
        );
        let pokemon_id =
            pokemon_map.get(&pokemon.name).unwrap().clone();
        let evolves_from_id =
            pokemon_map.get(&name).unwrap().clone();

        let pool = pool.clone();

        tasks.push(tokio::spawn(async move {
            sqlx::query!(
                r#"
        INSERT INTO evolutions (
            id, pokemon_id, evolves_from
        ) VALUES (?, ?, ?)"#,
                PokemonId::new(),
                pokemon_id,
                evolves_from_id,
            )
            .execute(&pool)
            .await
        }))
    }

    let pb = ProgressBar::new(tasks.len() as u64);
    while let Some(item) = tasks.next().await {
        item.into_diagnostic()?.into_diagnostic()?;
        pb.inc(1);
    }
    pb.finish();

    Ok(())
}

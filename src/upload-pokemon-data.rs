#[macro_use]
extern crate diesel;

use diesel::prelude::*;
use eyre::{Result, WrapErr};
use ksuid::Ksuid;
use std::{convert::TryFrom, env};

mod csv_utils;
mod models;
mod schema;
use csv_utils::*;
use models::*;
use schema::*;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let database_url =
        env::var("DATABASE_URL").expect("a db url");
    let conn = MysqlConnection::establish(&database_url)
        .expect(&format!(
            "Error connecting to {}",
            database_url
        ));

    let new_pokemon =
        csv::Reader::from_path("./pokemon.csv")?
            .deserialize::<PokemonCsv>()
            .map(|result| {
                result.map(|pokemon| {
                    let pokemon_db: PokemonDB =
                        pokemon.clone().into();
                    (pokemon, pokemon_db)
                })
            })
            .collect::<Result<
                Vec<(PokemonCsv, PokemonDB)>,
                csv::Error,
            >>()?;

    let abilities = new_pokemon
        .iter()
        .map(|(pokemon, pokemon_db)| {
            pokemon
                .abilities
                .iter()
                .map(|ability| Ability {
                    id: Ksuid::generate()
                        .to_base62()
                        .as_bytes()
                        .into(),
                    pokemon_id: pokemon_db.id.clone(),
                    ability: ability.clone(),
                })
                .collect::<Vec<Ability>>()
        })
        .flatten()
        .collect::<Vec<Ability>>();
    let egg_groups = new_pokemon
        .iter()
        .map(|(pokemon, pokemon_db)| {
            pokemon
                .egg_groups
                .iter()
                .map(|egg_group| EggGroup {
                    id: Ksuid::generate()
                        .to_base62()
                        .as_bytes()
                        .into(),
                    pokemon_id: pokemon_db.id.clone(),
                    egg_group: egg_group.clone(),
                })
                .collect::<Vec<EggGroup>>()
        })
        .flatten()
        .collect::<Vec<EggGroup>>();

    let evolutions = new_pokemon
        .iter()
        .filter_map(|(pokemon, pokemon_db)| {
            pokemon.evolves_from.clone().map(
                |evolves_from| {
                    let p = new_pokemon.iter().find(
                        |(pkm, pkm_db)| {
                            // dbg!(&evolves_from);
                            pkm_db.name == evolves_from
                        },
                    );
                    // dbg!(&p, evolves_from);
                    EvolvesFrom {
                        id: Ksuid::generate()
                            .to_base62()
                            .as_bytes()
                            .into(),
                        pokemon_id: pokemon_db.id.clone(),
                        evolves_from: p
                            .unwrap()
                            .1
                            .id
                            .clone(),
                    }
                },
            )
        })
        .collect::<Vec<EvolvesFrom>>();

    let typings = new_pokemon
        .iter()
        .map(|(pokemon, pokemon_db)| {
            pokemon
                .typing
                .iter()
                .map(|typing| Typing {
                    id: Ksuid::generate()
                        .to_base62()
                        .as_bytes()
                        .into(),
                    pokemon_id: pokemon_db.id.clone(),
                    typing: typing.clone(),
                })
                .collect::<Vec<Typing>>()
        })
        .flatten()
        .collect::<Vec<Typing>>();

    let result = diesel::insert_into(pokemon_table::table)
        .values(
            new_pokemon
                .into_iter()
                .map(|v| v.1)
                .collect::<Vec<PokemonDB>>(),
        )
        .execute(&conn);

    diesel::insert_into(abilities_table::table)
        .values(abilities)
        .execute(&conn)?;

    diesel::insert_into(egg_groups_table::table)
        .values(egg_groups)
        .execute(&conn)?;

    diesel::insert_into(evolutions_table::table)
        .values(evolutions)
        .execute(&conn)?;
    diesel::insert_into(typing_table::table)
        .values(typings)
        .execute(&conn)?;

    dbg!(result);

    Ok(())
}

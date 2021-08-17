use eyre::{Result, WrapErr};
use ksuid::Ksuid;
use sqlx::{
    mysql::{MySqlArguments, MySqlPoolOptions},
    MySql, MySqlPool,
};
use std::{collections::HashMap, convert::TryFrom, env, fs};
use indicatif::ProgressIterator;

mod csv_utils;
mod models;

use csv_utils::*;
use models::*;

async fn insert_pokemon(
    pool: &MySqlPool,
    PokemonDB {
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    }: &PokemonDB,
) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
    INSERT INTO pokemon_table (
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness
     )
    VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)
        "#,
        id,
        slug,
        name,
        pokedex_id,
        hp,
        attack,
        defense,
        special_attack,
        special_defense,
        speed,
        height,
        weight,
        generation,
        female_rate,
        genderless,
        legendary_or_mythical,
        is_default,
        forms_switchable,
        base_experience,
        capture_rate,
        base_happiness,
        primary_color,
        number_pokemon_with_typing,
        normal_attack_effectiveness,
        fire_attack_effectiveness,
        water_attack_effectiveness,
        electric_attack_effectiveness,
        grass_attack_effectiveness,
        ice_attack_effectiveness,
        fighting_attack_effectiveness,
        poison_attack_effectiveness,
        ground_attack_effectiveness,
        fly_attack_effectiveness,
        psychic_attack_effectiveness,
        bug_attack_effectiveness,
        rock_attack_effectiveness,
        ghost_attack_effectiveness,
        dragon_attack_effectiveness,
        dark_attack_effectiveness,
        steel_attack_effectiveness,
        fairy_attack_effectiveness,
    ).execute(pool).await
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let database_url =
        env::var("DATABASE_URL").expect("a db url");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let mut pokemon_map: HashMap<
        String,
        (PokemonCsv, PokemonDB),
    > = HashMap::new();

    let pokemon_csv = fs::read_to_string("./pokemon.csv")?;
    let csv_size = pokemon_csv.lines().count();

    let mut csv_reader = csv::Reader::from_reader(pokemon_csv.as_bytes());
    let it = csv_reader.deserialize::<PokemonCsv>();

    for row in it.progress_count(u64::try_from(csv_size)?)
    {
        let pokemon = row?;
        let pokemon_db: PokemonDB = pokemon.clone().into();

        insert_pokemon(&pool, &pokemon_db).await?;

        for ability in pokemon.abilities.iter() {
            sqlx::query!(
                r#"
            INSERT INTO abilities_table (id, pokemon_id, ability) VALUES (?,?, ?)"#,
                Ksuid::generate().to_base62().into_bytes(),
                pokemon_db.id.clone(),
                ability.clone(),
            ).execute(&pool).await?;
        }
        for egg_group in pokemon.egg_groups.iter() {
            sqlx::query!(
                r#"
            INSERT INTO egg_groups_table (id, pokemon_id, egg_group) VALUES (?,?, ?)"#,
                Ksuid::generate().to_base62().into_bytes(),
                pokemon_db.id.clone(),
                egg_group.clone(),
            ).execute(&pool).await?;
        }
        for typing in pokemon.typing.iter() {
            sqlx::query!(
                r#"
            INSERT INTO typing_table (id, pokemon_id, typing) VALUES (?,?, ?)"#,
                Ksuid::generate().to_base62().into_bytes(),
                pokemon_db.id.clone(),
                typing.clone(),
            ).execute(&pool).await?;
        }
        pokemon_map.insert(
            pokemon.name.clone(),
            (pokemon, pokemon_db),
        );
    }

    for (key, (pokemon, pokemon_db)) in pokemon_map.iter() {
        if let Some(evolves_from) = pokemon.evolves_from.clone() {
            if let Some((
                _key,
                (_evolves_from_pokemon, evolves_from_db),
            )) = pokemon_map.iter().find(
                |(key, (pkm, pkm_db))| {
                    // dbg!(&evolves_from);
                    pkm_db.name == evolves_from
                },
            ) {
                sqlx::query!(
                    r#"
                INSERT INTO evolutions_table (id, pokemon_id, evolves_from) VALUES (?,?, ?)"#,
                
                    Ksuid::generate()
                        .to_base62()
                        .into_bytes(),
                    pokemon_db.id.clone(),
                    evolves_from_db.id.clone(),
                
                ).execute(&pool).await?;
            };
        };
    }

    // let result = diesel::insert_into(pokemon_table::table)
    //     .values(
    //         new_pokemon
    //             .into_iter()
    //             .map(|v| v.1)
    //             .collect::<Vec<PokemonDB>>(),
    //     )
    //     .execute(&conn);

    // diesel::insert_into(abilities_table::table)
    //     .values(abilities)
    //     .execute(&conn)?;

    // diesel::insert_into(egg_groups_table::table)
    //     .values(egg_groups)
    //     .execute(&conn)?;

    // diesel::insert_into(evolutions_table::table)
    //     .values(evolutions)
    //     .execute(&conn)?;
    // diesel::insert_into(typing_table::table)
    //     .values(typings)
    //     .execute(&conn)?;

    // dbg!(result);

    Ok(())
}

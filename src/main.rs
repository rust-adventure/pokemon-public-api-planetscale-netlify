mod models;
use models::*;

use lamedh_http::{
    http::Uri,
    lambda::{lambda, Context, Error},
    IntoResponse, Request, RequestExt,
};
use serde::{Serialize, Serializer};
use serde_json::json;

mod db;

#[lambda(http)]
#[tokio::main]
async fn main(
    request: Request,
    context: Context,
) -> Result<impl IntoResponse, Error> {
    let pool = db::connect().await?;

    let pokemon_requested = request
        .uri()
        .path()
        .trim_start_matches("/api/pokemon/")
        .trim_end_matches("/");

    let rows = sqlx::query_as!(PokemonProfile,r#"
            SELECT    P.id,
            P.name,
            P.slug,
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
            genderless as "genderless!: bool",
            legendary_or_mythical as "legendary_or_mythical!: bool",
            is_default as "is_default!: bool",
            forms_switchable as "forms_switchable!: bool",
            abilities,
            typings,
            egg_groups,
            Ev.evolves_from
        FROM      pokemon_table P
        LEFT JOIN
            (
                       SELECT     name,
                                  Group_concat(DISTINCT ability SEPARATOR ',') AS abilities
                       FROM       pokemon_table
                       INNER JOIN abilities_table
                       ON         pokemon_table.id = abilities_table.pokemon_id
                       GROUP BY   name ) A
        ON        P.name = A.name
        LEFT JOIN
            (
                       SELECT     name,
                                  Group_concat(DISTINCT typing SEPARATOR ',') AS typings
                       FROM       pokemon_table
                       INNER JOIN typing_table
                       ON         pokemon_table.id = typing_table.pokemon_id
                       GROUP BY   name ) T
        ON        P.name = T.name
        LEFT JOIN
            (
                       SELECT     name,
                                  Group_concat(DISTINCT egg_group SEPARATOR ',') AS egg_groups
                       FROM       pokemon_table
                       INNER JOIN egg_groups_table
                       ON         pokemon_table.id = egg_groups_table.pokemon_id
                       GROUP BY   name ) E
        ON        P.name = E.name
        LEFT JOIN evolutions_table Ev
        ON        P.id = Ev.pokemon_id
        WHERE     slug = ?
        "#, pokemon_requested)
        .fetch_one(&pool).await?;

    Ok(serde_json::to_value(rows)?)
}

#[derive(Serialize, sqlx::FromRow)]
struct PokemonRow {
    #[serde(serialize_with = "to_ksuid_string")]
    id: Vec<u8>,
    name: String,
    hp: u16,
    abilities: Option<String>,
}

fn to_ksuid_string<S>(
    id: &Vec<u8>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let id = std::str::from_utf8(id).unwrap();
    serializer.serialize_str(id)
}

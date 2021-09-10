use std::env;

use color_eyre::{eyre::WrapErr, Section};
use lamedh_http::{
    lambda::{lambda, Context, Error},
    IntoResponse, Request,
};
mod db;
use db::*;
use sqlx::mysql::MySqlPoolOptions;

#[lambda(http)]
#[tokio::main]
async fn main(
    request: Request,
    context: Context,
) -> Result<impl IntoResponse, Error> {
    color_eyre::install()?;

    let database_url = env::var("DATABASE_URL")
        .wrap_err("Must have a DATABASE_URL set")
        .suggestion("Run `pscale connect <database> <branch>` to get a connection")?;

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .suggestion("database urls must be in the form `mysql://username:password@host:port/database`")?;

    let pokemon_requested = request
        .uri()
        .path()
        .trim_start_matches("/api/pokemon/")
        .trim_end_matches("/");

    let rows = sqlx::query_as!(PokemonProfile,r#"
            SELECT    P.id as "id!: PokemonId",
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
            base_experience,
            capture_rate,
            base_happiness,
            primary_color,
            legendary_or_mythical as "legendary_or_mythical!: bool",
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

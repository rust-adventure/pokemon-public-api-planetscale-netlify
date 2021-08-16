use diesel::prelude::*;
use ksuid::Ksuid;

struct PokemonId(Ksuid);

use crate::schema::*;
use diesel::{
    expression::AsExpression, prelude::*, sql_types::Binary,
};

#[derive(Debug, Queryable, Insertable)]
#[table_name = "pokemon_table"]
pub struct PokemonDB {
    pub id: Vec<u8>,
    pub name: String,
    pub slug: String,
    pub pokedex_id: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    // abilities: Vec<String>,
    // #[serde(deserialize_with = "from_comma_separated")]
    // typing: Vec<String>,
    pub hp: u16,
    pub attack: u16,
    pub defense: u16,
    pub special_attack: u16,
    pub special_defense: u16,
    pub speed: u16,
    pub height: u16,
    pub weight: u16,
    pub generation: u16,
    pub female_rate: Option<f32>,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub genderless: bool,
    // #[serde(
    //     rename(deserialize = "legendary/mythical"),
    //     deserialize_with = "from_capital_bool"
    // )]
    pub legendary_or_mythical: bool,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub is_default: bool,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    // egg_groups: Vec<String>,
    pub base_happiness: u16,
    // evolves_from: Option<String>,
    pub primary_color: String,
    pub number_pokemon_with_typing: f32,
    pub normal_attack_effectiveness: f32,
    pub fire_attack_effectiveness: f32,
    pub water_attack_effectiveness: f32,
    pub electric_attack_effectiveness: f32,
    pub grass_attack_effectiveness: f32,
    pub ice_attack_effectiveness: f32,
    pub fighting_attack_effectiveness: f32,
    pub poison_attack_effectiveness: f32,
    pub ground_attack_effectiveness: f32,
    pub fly_attack_effectiveness: f32,
    pub psychic_attack_effectiveness: f32,
    pub bug_attack_effectiveness: f32,
    pub rock_attack_effectiveness: f32,
    pub ghost_attack_effectiveness: f32,
    pub dragon_attack_effectiveness: f32,
    pub dark_attack_effectiveness: f32,
    pub steel_attack_effectiveness: f32,
    pub fairy_attack_effectiveness: f32,
}

pub struct Pokemon {
    pub id: Vec<u8>,
    pub name: String,
    pub slug: String,
    pub pokedex_id: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    // abilities: Vec<String>,
    // #[serde(deserialize_with = "from_comma_separated")]
    // typing: Vec<String>,
    pub hp: u8,
    pub attack: u8,
    pub defense: u8,
    pub special_attack: u8,
    pub special_defense: u8,
    pub speed: u8,
    pub height: u16,
    pub weight: u16,
    pub generation: u8,
    pub female_rate: Option<f32>,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub genderless: bool,
    // #[serde(
    //     rename(deserialize = "legendary/mythical"),
    //     deserialize_with = "from_capital_bool"
    // )]
    pub legendary_or_mythical: bool,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub is_default: bool,
    // #[serde(deserialize_with = "from_capital_bool")]
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u8,
    // #[serde(deserialize_with = "from_comma_separated")]
    // egg_groups: Vec<String>,
    pub base_happiness: u8,
    // evolves_from: Option<String>,
    pub primary_color: String,
    pub number_pokemon_with_typing: f32,
    pub normal_attack_effectiveness: f32,
    pub fire_attack_effectiveness: f32,
    pub water_attack_effectiveness: f32,
    pub electric_attack_effectiveness: f32,
    pub grass_attack_effectiveness: f32,
    pub ice_attack_effectiveness: f32,
    pub fighting_attack_effectiveness: f32,
    pub poison_attack_effectiveness: f32,
    pub ground_attack_effectiveness: f32,
    pub fly_attack_effectiveness: f32,
    pub psychic_attack_effectiveness: f32,
    pub bug_attack_effectiveness: f32,
    pub rock_attack_effectiveness: f32,
    pub ghost_attack_effectiveness: f32,
    pub dragon_attack_effectiveness: f32,
    pub dark_attack_effectiveness: f32,
    pub steel_attack_effectiveness: f32,
    pub fairy_attack_effectiveness: f32,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "abilities_table"]
pub struct Ability {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub ability: String,
}

#[derive(Debug, Queryable, Insertable)]
#[table_name = "egg_groups_table"]
pub struct EggGroup {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub egg_group: String,
}
#[derive(Debug, Queryable, Insertable)]
#[table_name = "evolutions_table"]
pub struct EvolvesFrom {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub evolves_from: Vec<u8>,
}
#[derive(Debug, Queryable, Insertable)]
#[table_name = "typing_table"]
pub struct Typing {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub typing: String,
}

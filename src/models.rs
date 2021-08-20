use ksuid::Ksuid;
use serde::{Serialize, Serializer};
use sqlx::{database::HasValueRef, Database, Decode};

pub struct PokemonId(Ksuid);

impl<'r, DB: Database> Decode<'r, DB> for PokemonId
where
    &'r [u8]: Decode<'r, DB>,
{
    fn decode(
        value: <DB as HasValueRef<'r>>::ValueRef,
    ) -> Result<
        PokemonId,
        Box<dyn std::error::Error + 'static + Send + Sync>,
    > {
        let value = <&[u8] as Decode<DB>>::decode(value)?;
        let base62_ksuid = std::str::from_utf8(&value)?;
        Ok(PokemonId(Ksuid::from_base62(
            base62_ksuid,
        )?))
    }
}

impl Serialize for PokemonId {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.0.to_base62())
    }
}
#[derive(Debug)]
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

#[derive(Serialize, sqlx::FromRow)]
pub struct PokemonProfile {
    pub id: PokemonId,
    pub name: String,
    pub slug: String,
    pub pokedex_id: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    pub abilities: Option<String>,
    // #[serde(deserialize_with = "from_comma_separated")]
    pub typings: Option<String>,
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
    pub genderless: bool,
    pub legendary_or_mythical: bool,
    pub is_default: bool,
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    pub egg_groups: Option<String>,
    pub base_happiness: u16,
    pub evolves_from: Option<Vec<u8>>,
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

#[derive(Debug)]
pub struct Ability {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub ability: String,
}

#[derive(Debug)]
pub struct EggGroup {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub egg_group: String,
}
#[derive(Debug)]
pub struct EvolvesFrom {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub evolves_from: Vec<u8>,
}
#[derive(Debug)]
pub struct Typing {
    pub id: Vec<u8>,
    pub pokemon_id: Vec<u8>,
    pub typing: String,
}

use crate::models::PokemonDB;
use eyre::{Result, WrapErr};
use inflector::Inflector;
use ksuid::Ksuid;
use serde::{de, Deserialize};

#[derive(Debug, Deserialize, Clone)]
pub struct PokemonCsv {
    pub name: String,
    pub pokedex_id: u16,
    #[serde(deserialize_with = "from_comma_separated")]
    pub abilities: Vec<String>,
    #[serde(deserialize_with = "from_comma_separated")]
    pub typing: Vec<String>,
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
    #[serde(deserialize_with = "from_capital_bool")]
    pub genderless: bool,
    #[serde(
        rename(deserialize = "legendary/mythical"),
        deserialize_with = "from_capital_bool"
    )]
    pub is_legendary_or_mythical: bool,
    #[serde(deserialize_with = "from_capital_bool")]
    pub is_default: bool,
    #[serde(deserialize_with = "from_capital_bool")]
    pub forms_switchable: bool,
    pub base_experience: u16,
    pub capture_rate: u8,
    #[serde(deserialize_with = "from_comma_separated")]
    pub egg_groups: Vec<String>,
    pub base_happiness: u8,
    pub evolves_from: Option<String>,
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

impl From<PokemonCsv> for PokemonDB {
    fn from(
        PokemonCsv {
            name,
            pokedex_id,
            abilities,
            typing,
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
            is_legendary_or_mythical,
            is_default,
            forms_switchable,
            base_experience,
            capture_rate,
            egg_groups,
            base_happiness,
            evolves_from,
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
        }: PokemonCsv,
    ) -> Self {
        let id: Vec<u8> =
            Ksuid::generate().to_base62().into_bytes();
        let slug = name.to_kebab_case();
        PokemonDB {
            id,
            slug,
            name,
            pokedex_id,
            hp: hp.into(),
            attack: attack.into(),
            defense: defense.into(),
            special_attack: special_attack.into(),
            special_defense: special_defense.into(),
            speed: speed.into(),
            height,
            weight,
            generation: generation.into(),
            female_rate,
            genderless,
            legendary_or_mythical: is_legendary_or_mythical,
            is_default,
            forms_switchable,
            base_experience,
            capture_rate: capture_rate.into(),
            base_happiness: base_happiness.into(),
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
        }
    }
}

fn from_capital_bool<'de, D>(
    deserializer: D,
) -> Result<bool, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer)?;

    match s {
        "True" => Ok(true),
        "False" => Ok(false),
        _ => Err(de::Error::custom("not a boolean!")),
    }
}

fn from_comma_separated<'de, D>(
    deserializer: D,
) -> Result<Vec<String>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str =
        de::Deserialize::deserialize(deserializer)?;

    Ok(s.split(", ").map(|v| v.to_string()).collect())
}

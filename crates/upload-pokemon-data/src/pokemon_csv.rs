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

    Ok(s.split(", ")
        .filter(|v| !v.is_empty())
        .map(|v| v.to_string())
        .collect())
}

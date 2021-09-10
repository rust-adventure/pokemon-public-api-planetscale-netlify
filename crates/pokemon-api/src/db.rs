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
    pub legendary_or_mythical: bool,
    pub base_experience: u16,
    pub capture_rate: u16,
    // #[serde(deserialize_with = "from_comma_separated")]
    pub egg_groups: Option<String>,
    pub base_happiness: u16,
    pub evolves_from: Option<Vec<u8>>,
    pub primary_color: String,
}

use diesel::{
    allow_tables_to_appear_in_same_query, table, *,
};

table! {
    abilities_table (id) {
        id -> Varbinary,
        pokemon_id -> Varbinary,
        ability -> Varchar,
    }
}

table! {
    egg_groups_table (id) {
        id -> Varbinary,
        pokemon_id -> Varbinary,
        egg_group -> Varchar,
    }
}

table! {
    evolutions_table (id) {
        id -> Varbinary,
        pokemon_id -> Varbinary,
        evolves_from -> Varbinary,
    }
}

table! {
    pokemon_table (id) {
        id -> Varbinary,
        slug -> Varchar,
        name -> Varchar,
        pokedex_id -> Unsigned<Smallint>,
        hp -> Unsigned<Smallint>,
        attack -> Unsigned<Smallint>,
        defense -> Unsigned<Smallint>,
        special_attack -> Unsigned<Smallint>,
        special_defense -> Unsigned<Smallint>,
        speed -> Unsigned<Smallint>,
        height -> Unsigned<Smallint>,
        weight -> Unsigned<Smallint>,
        generation -> Unsigned<Smallint>,
        female_rate -> Nullable<Float>,
        genderless -> Bool,
        legendary_or_mythical -> Bool,
        is_default -> Bool,
        forms_switchable -> Bool,
        base_experience -> Unsigned<Smallint>,
        capture_rate -> Unsigned<Smallint>,
        base_happiness -> Unsigned<Smallint>,
        primary_color -> Varchar,
        number_pokemon_with_typing -> Float,
        normal_attack_effectiveness -> Float,
        fire_attack_effectiveness -> Float,
        water_attack_effectiveness -> Float,
        electric_attack_effectiveness -> Float,
        grass_attack_effectiveness -> Float,
        ice_attack_effectiveness -> Float,
        fighting_attack_effectiveness -> Float,
        poison_attack_effectiveness -> Float,
        ground_attack_effectiveness -> Float,
        fly_attack_effectiveness -> Float,
        psychic_attack_effectiveness -> Float,
        bug_attack_effectiveness -> Float,
        rock_attack_effectiveness -> Float,
        ghost_attack_effectiveness -> Float,
        dragon_attack_effectiveness -> Float,
        dark_attack_effectiveness -> Float,
        steel_attack_effectiveness -> Float,
        fairy_attack_effectiveness -> Float,
    }
}

table! {
    typing_table (id) {
        id -> Varbinary,
        pokemon_id -> Varbinary,
        typing -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    abilities_table,
    egg_groups_table,
    evolutions_table,
    pokemon_table,
    typing_table,
);

CREATE TABLE IF NOT EXISTS pokemon_table(
    id VARBINARY(27) NOT NULL, -- ksuid
    slug VARCHAR(30) NOT NULL, -- generated
    name VARCHAR(30) NOT NULL,
    pokedex_id SMALLINT UNSIGNED NOT NULL,
    -- abilities -- new table
    -- typing -- new table
    hp SMALLINT UNSIGNED NOT NULL, -- u8
    attack SMALLINT UNSIGNED NOT NULL, -- u8
    defense SMALLINT UNSIGNED NOT NULL, -- u8
    special_attack SMALLINT UNSIGNED NOT NULL, -- u8
    special_defense SMALLINT UNSIGNED NOT NULL, -- u8
    speed SMALLINT UNSIGNED NOT NULL, -- u8
    height SMALLINT UNSIGNED NOT NULL,
    weight SMALLINT UNSIGNED NOT NULL,
    generation SMALLINT UNSIGNED NOT NULL, -- u8
    female_rate FLOAT,
    genderless BOOLEAN NOT NULL,
    legendary_or_mythical BOOLEAN NOT NULL,
    is_default BOOLEAN NOT NULL,
    forms_switchable BOOLEAN NOT NULL,
    base_experience SMALLINT UNSIGNED NOT NULL,
    capture_rate SMALLINT UNSIGNED NOT NULL, -- u8
    -- egg_groups -- new table
    base_happiness SMALLINT UNSIGNED NOT NULL, -- u8
    -- evolves_from -- new table
    primary_color VARCHAR(6) NOT NULL,
    number_pokemon_with_typing FLOAT NOT NULL,
    normal_attack_effectiveness FLOAT NOT NULL,
    fire_attack_effectiveness FLOAT NOT NULL,
    water_attack_effectiveness FLOAT NOT NULL,
    electric_attack_effectiveness FLOAT NOT NULL,
    grass_attack_effectiveness FLOAT NOT NULL,
    ice_attack_effectiveness FLOAT NOT NULL,
    fighting_attack_effectiveness FLOAT NOT NULL,
    poison_attack_effectiveness FLOAT NOT NULL,
    ground_attack_effectiveness FLOAT NOT NULL,
    fly_attack_effectiveness FLOAT NOT NULL,
    psychic_attack_effectiveness FLOAT NOT NULL,
    bug_attack_effectiveness FLOAT NOT NULL,
    rock_attack_effectiveness FLOAT NOT NULL,
    ghost_attack_effectiveness FLOAT NOT NULL,
    dragon_attack_effectiveness FLOAT NOT NULL,
    dark_attack_effectiveness FLOAT NOT NULL,
    steel_attack_effectiveness FLOAT NOT NULL,
    fairy_attack_effectiveness FLOAT NOT NULL,
    PRIMARY KEY ( id ),
    UNIQUE ( slug )
);

CREATE TABLE IF NOT EXISTS abilities_table(
    id VARBINARY(27) NOT NULL, -- ksuid
    pokemon_id VARBINARY(27) NOT NULL,
    ability VARCHAR(16) NOT NULL,
    PRIMARY KEY ( id )
);

CREATE TABLE IF NOT EXISTS typing_table(
    id VARBINARY(27) NOT NULL, -- ksuid
    pokemon_id VARBINARY(27) NOT NULL,
    typing VARCHAR(8) NOT NULL,
    PRIMARY KEY ( id )
);

CREATE TABLE IF NOT EXISTS egg_groups_table(
    id VARBINARY(27) NOT NULL, -- ksuid
    pokemon_id VARBINARY(27) NOT NULL,
    egg_group VARCHAR(13) NOT NULL,
    PRIMARY KEY ( id )
);

CREATE TABLE IF NOT EXISTS evolutions_table(
    id VARBINARY(27) NOT NULL, -- ksuid
    pokemon_id VARBINARY(27) NOT NULL,
    evolves_from VARBINARY(27) NOT NULL,
    PRIMARY KEY ( id )
);
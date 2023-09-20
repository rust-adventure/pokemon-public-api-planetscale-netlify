SELECT b.name as pokemon, GROUP_CONCAT(a.name) as evolves_into
FROM evolutions
LEFT JOIN pokemon a on a.id = evolutions.pokemon_id
LEFT JOIN pokemon b on b.id = evolves_from
GROUP BY pokemon;
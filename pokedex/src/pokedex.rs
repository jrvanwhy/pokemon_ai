use pokemon::Pokemon;

// The total number of Pokemon
const NUM_POKEMON: usize = 721;

#[derive(Clone)]
pub enum DamageClass
{
	Status,
	Physical,
	Special
}

#[derive(Clone)]
pub struct Move
{
	id: i32,
	name: String,
	type_id: i32,
	power: i32,
	pp: i32,
	accuracy: i32,
	effect_id: i32,
	effect_chance: i32,
	damage_class: DamageClass,
}

#[derive(Clone)]
pub struct PokeDesc
{
	id: i32,
	hp: i32
}

// Structure to handle the variable Pokemon configuration
// and to process the default stats for the different types of Pokemon
pub struct Pokedex
{
	// Vector of base-stat descriptions. Only includes pokemon that are currently enabled
	// (as determined by the config file, if found).
	base_pokemon: Vec<Option<PokeDesc>>,
}

impl Pokedex
{
	// Constructs a new Pokedex instance. This will look for the
	// configuration file and do all necessary Pokemon parsing.
	pub fn new() -> Pokedex
	{
		Pokedex { base_pokemon: Vec::with_capacity(NUM_POKEMON) }
	}
}

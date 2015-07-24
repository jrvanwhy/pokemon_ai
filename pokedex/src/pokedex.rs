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

// Struct keeping track of the active Pokemon IDs
struct ActivePokes
{
	active_ids: Vec<i32>
}

impl ActivePokes
{
	// Constructor -- looks for a configuration file and uses it if possible.
	fn new() -> ActivePokes
	{
		use std::env;
		use std::fs::File;

		// Our output variable
		let obj = ActivePokes { active_ids: Vec::new() };

		// Try to read the configuration environment variable.
		let env_result = env::var("POKEDEX_CONFIG");

		// If we failed to read it, then return none
		if env_result.is_err()
		{
			return obj
		}

		// Try to open the file
		let file_result = File::open(env_result.unwrap());

		obj
	}

	// Function to check if a given Pokemon ID is enabled.
	fn id_active(&self, id: i32) -> bool
	{
		if self.active_ids.is_empty()
		{
			true
		}
		else
		{
			self.active_ids.binary_search(&id).is_ok()
		}
	}
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
		// Create the Pokemon selector
		let active_ids = ActivePokes::new();

		Pokedex { base_pokemon: Vec::new() }
	}

	// Returns the pokemon description for the given Pokemon ID, if it exists.
	// Otherwise returns None
	pub fn get_poke_desc(&self, id: usize) -> &Option<PokeDesc>
	{
		&(self.base_pokemon[id-1])
	}
}

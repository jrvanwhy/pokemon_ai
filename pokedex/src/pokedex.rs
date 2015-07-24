use pokemon::Pokemon;

// The total number of Pokemon
const NUM_POKEMON: usize = 721;

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

// Structure to handle the variable Pokemon configuration
// and to process the default stats for the different types of Pokemon
pub struct Pokedex
{
	// Vector of base-stat Pokemon.
	base_pokemon: Vec<Pokemon>,

	// Each element of this list is None (Pokemon disabled)
	// or an index into base_pokemon
	id_map: [Option<usize>; NUM_POKEMON],
}

impl Pokedex
{
	// Constructs a new Pokedex instance. This will look for the
	// configuration file and do all necessary Pokemon parsing.
	pub fn new() -> Pokedex
	{
		let mut out = Pokedex { base_pokemon: Vec::new(), id_map: [None; NUM_POKEMON] };
		out
	}

	// Gives a reference to a base-stat Pokemon with the given ID,
	// 

	// Creates a new Pokemon with the base stats for its type.
	// If the given ID does not exist or is disabled, will return
	// None
	pub fn new_pokemon(&self, id: i32) -> Option<Pokemon>
	{
		None // TODO: This
	}
}

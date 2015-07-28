extern crate csv;

use std::fs::File;

#[derive(Clone)]
pub enum DamageClass
{
	Status,
	Physical,
	Special
}

#[derive(Clone)]
pub struct MoveDesc
{
	pub id: i32,
	pub name: String,
	pub type_id: i32,
	pub power: i32,
	pub pp: i32,
	pub accuracy: f64, // between 0 and 1
	pub effect_id: i32,
	pub effect_chance: i32,
	pub damage_class: DamageClass,
}

// all of these pubs can't be the best way to do this.
// I'm not sure what is though...
#[derive(Clone)]
pub struct PokeDesc
{
	pub id: i32,
	pub type_ids: Vec<i32>,
	pub hp: i32,
	pub attack: i32,
	pub defense: i32,
	pub sp_atk: i32,
	pub sp_def: i32,
	pub speed: i32,
	pub avail_moves: Vec<MoveDesc>
}

// Convenience function to get a CSV Reader for the given file
fn get_csv_rdr(csv_dir: String, filename: &str) -> csv::Reader<File> {
	use std::error::Error;

	let path = csv_dir + filename;
	match csv::Reader::from_file(&path)
	{
		Ok(rdr) => rdr,
		Err(why) => panic!("Could not open file {} (full path: {}). Reason: {}.", filename, path, Error::description(&why))
	}
}

// Structure to handle the variable Pokemon configuration
// and to process the default stats for the different types of Pokemon
pub struct Pokedex
{
	// Vector of base-stat descriptions.
	base_pokemon: Vec<PokeDesc>,
}

impl Pokedex
{
	// Constructs a new Pokedex instance. This will look for the
	// configuration file and do all necessary Pokemon parsing.
	pub fn new() -> Pokedex
	{
		use std::env;

		const CSV_DIR_ENV_VAR: &'static str = "POKEDEX_DIR";

		// Read the environment variable which has the location of the
		// configuration and CSV file directory
		let csv_dir =
			match env::var(CSV_DIR_ENV_VAR)
			{
				Ok(path) => path + "/",
				Err(_) => { panic!("Could not find the {} environment variable!", CSV_DIR_ENV_VAR) }
			};

		// Read the pokemon.csv file.
		for record in get_csv_rdr(csv_dir, "pokemon.csv").decode() {
			let (id, identifier, species_id, height, weight, base_experience, order, is_default):
			    (i32, String, i32, i32, i32, i32, i32, i32) = record.unwrap();

			println!("{:?}", (id, identifier, species_id, height, weight, base_experience, order, is_default));
		}

		// Temporary
		Pokedex { base_pokemon: Vec::new() }
	}

	// Returns the pokemon description for the given Pokemon ID, if it exists.
	// Otherwise returns None
	pub fn get_poke_desc(&self, id: usize) -> &PokeDesc
	{
		&(self.base_pokemon[id-1])
	}
}

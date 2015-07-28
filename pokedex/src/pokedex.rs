extern crate csv;

use std::fs::File;

#[derive(Clone,PartialEq,Debug)]
pub enum DamageClass
{
	Status,
	Physical,
	Special
}

#[derive(Clone,Debug)]
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
#[derive(Clone,Debug)]
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
	pub avail_moves: Vec<MoveDesc>,
	pub name: String
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

		// Output variable
		let mut out = Pokedex { base_pokemon: Vec::new() };

		// Read the pokemon.csv file. Stop once the Pokemon IDs become non-consecutive.
		for record in get_csv_rdr(csv_dir.clone(), "pokemon.csv").decode()
		{
			let (id, identifier, _, _, _, _, _, _):
			    (i32, String, i32, i32, i32, i32, i32, i32) = record.unwrap();

			if id != out.base_pokemon.len() as i32 + 1
			{
				break;
			}

			out.base_pokemon.push(PokeDesc { id: id,
			                                 type_ids: Vec::new(),
			                                 hp: 0,
			                                 attack: 0,
			                                 defense: 0,
			                                 sp_atk: 0,
			                                 sp_def: 0,
			                                 speed: 0,
			                                 avail_moves: Vec::new(),
			                                 name: identifier } );
		}

		// Read in the Pokemon types
		for record in get_csv_rdr(csv_dir.clone(), "pokemon_types.csv").decode()
		{
			let (poke_id, type_id, _): (usize, i32, i32) = record.unwrap();

			match out.base_pokemon.get_mut(poke_id - 1)
			{
				Some(p) => p.type_ids.push(type_id),
				None => {}
			}
		}

		// Read in the base stats
		for record in get_csv_rdr(csv_dir.clone(), "pokemon_stats.csv").decode()
		{
			let (poke_id, stat_id, base_stat, _): (usize, i32, i32, i32) = record.unwrap();

			let cur_poke = match out.base_pokemon.get_mut(poke_id - 1)
			{
				Some(n) => n,
				None => { continue }
			};

			*match stat_id
			{
				1 => &mut cur_poke.hp,
				2 => &mut cur_poke.attack,
				3 => &mut cur_poke.defense,
				4 => &mut cur_poke.sp_atk,
				5 => &mut cur_poke.sp_def,
				6 => &mut cur_poke.speed,
				_ => panic!("Unknown stat ID {} found for pokemon {}!", stat_id, poke_id)
			} = base_stat;
		}

		out
	}

	// Returns the pokemon description for the given Pokemon ID, if it exists.
	// Otherwise returns None
	pub fn get_poke_desc(&self, id: usize) -> Option<&PokeDesc>
	{
		self.base_pokemon.get(id-1)
	}
}

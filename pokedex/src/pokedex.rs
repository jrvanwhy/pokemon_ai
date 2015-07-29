extern crate csv;
extern crate sparse_vec;

use std::fs::File;
use self::sparse_vec::SparseVec;

#[derive(Clone,PartialEq,Debug)]
pub enum DamageClass
{
	Status,
	Physical,
	Special
}

#[derive(Clone,Debug)]
pub enum Accuracy
{
	Finite(f64), // between 0 and 1
	Infinite
}

impl DamageClass
{
	pub fn new(id: i32) -> Option<DamageClass>
	{
		match id
		{
			1 => Some(DamageClass::Status),
			2 => Some(DamageClass::Physical),
			3 => Some(DamageClass::Special),
			_ => None
		}
	}
}

#[derive(Clone,Debug)]
pub enum StatMod
{
	Hp(i32),
	Attack(i32),
	Defense(i32),
	SpAtk(i32),
	SpDef(i32),
	Speed(i32)
}

impl StatMod
{
	pub fn new(id: i32, change: i32) -> Option<StatMod>
	{
		match id
		{
			1 => Some(StatMod::Hp(change)),
			2 => Some(StatMod::Attack(change)),
			3 => Some(StatMod::Defense(change)),
			4 => Some(StatMod::SpAtk(change)),
			5 => Some(StatMod::SpDef(change)),
			6 => Some(StatMod::Speed(change)),
			_ => None
		}
	}
}

#[derive(Clone,Debug)]
pub struct MoveDesc
{
	pub id: i32,
	pub name: String,
	pub type_id: i32,
	pub power: i32,
	pub pp: i32,
	pub accuracy: Accuracy,
	pub effect_id: i32,
	pub effect_chance: i32,
	pub damage_class: DamageClass,
	pub user_stat_effects: Vec<StatMod>,
	pub recip_stat_effects: Vec<StatMod>,
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

impl PokeDesc
{
	pub fn get_move_desc(&self, id: i32) -> Option<&MoveDesc>
	{
		self.avail_moves.iter().filter(|m| m.id == id).next()
	}
}

// Convenience function to get a CSV Reader for the given file
pub fn get_csv_rdr(path: String) -> csv::Reader<File>
{
	use std::error::Error;

	match csv::Reader::from_file(&path)
	{
		Ok(rdr) => rdr,
		Err(why) => panic!("Could not open file {}. Reason: {}.", path, Error::description(&why))
	}
}

// Structure to handle the variable Pokemon configuration
// and to process the default stats for the different types of Pokemon
pub struct Pokedex
{
	// Vector of base-stat descriptions.
	base_pokemon: SparseVec<PokeDesc>,

	// Vectors of type efficacies
	type_efficacies: Vec<Vec<f64>>
}

impl Pokedex
{
	// Constructs a new Pokedex instance. This will look for the
	// configuration file and do all necessary Pokemon parsing.
	pub fn new(path: String) -> Pokedex
	{
		// This will contain all the move descriptions
		let mut moves = SparseVec::<MoveDesc>::new();

		// Maximum ID, above which we reject new Pokemon or moves.
		let max_id = 10000;

		// Read in move.csv until a non-consecutive move ID is read
		for record in get_csv_rdr(path.clone() + "moves.csv").decode()
		{
			let (id, identifier, _, type_id, power, pp, accuracy, _, _, damage_class_id, effect_id, effect_chance):
			    (i32, String, i32, i32, Option<i32>, Option<i32>, Option<i32>, i32, i32, i32, i32, Option<i32>) = record.unwrap();

			if id > max_id
			{
				break
			}

			moves.push_at(id as usize, MoveDesc
			{
				id: id,
				name: identifier,
				type_id: type_id,
				power: power.unwrap_or(0),
				pp: pp.unwrap_or(0),
				accuracy: match accuracy { Some(n) => Accuracy::Finite(n as f64 / 100.), None => Accuracy::Infinite },
				effect_id: effect_id,
				effect_chance: effect_chance.unwrap_or(0),
				damage_class: DamageClass::new(damage_class_id).expect("Invalid damage class in moves.csv"),
				user_stat_effects: Vec::new(),
				recip_stat_effects: Vec::new(),
			});
		}

		// Read in the move stat effects
		for record in get_csv_rdr(path.clone() + "move_meta_stat_changes.csv").decode()
		{
			let (move_id, stat_id, change): (usize, i32, i32) = record.unwrap();

			match (moves.get_mut(move_id), StatMod::new(stat_id, change))
			{
				(Some(m), Some(s)) =>
				{
					if change > 0
					{
						m.user_stat_effects.push(s);
					}
					else
					{
						m.recip_stat_effects.push(s);
					}
				},
				_ => {}
			}
		}

		// Output variable
		let mut out = Pokedex { base_pokemon: SparseVec::new(), type_efficacies: Vec::new() };

		// Read the pokemon.csv file. Stop once the Pokemon IDs become non-consecutive.
		for record in get_csv_rdr(path.clone() + "pokemon.csv").decode()
		{
			let (id, identifier, _, _, _, _, _, _):
			    (i32, String, i32, i32, i32, i32, i32, i32) = record.unwrap();

			if id > max_id
			{
				break
			}

			out.base_pokemon.push_at(id as usize, PokeDesc
			{
				id: id,
				type_ids: Vec::new(),
				hp: 0,
				attack: 0,
				defense: 0,
				sp_atk: 0,
				sp_def: 0,
				speed: 0,
				avail_moves: Vec::new(),
				name: identifier
			});
		}

		// Read in the Pokemon types
		for record in get_csv_rdr(path.clone() + "pokemon_types.csv").decode()
		{
			let (poke_id, type_id, _): (usize, i32, i32) = record.unwrap();

			match out.base_pokemon.get_mut(poke_id)
			{
				Some(p) => p.type_ids.push(type_id),
				None => {}
			}
		}

		// Read in the base stats
		for record in get_csv_rdr(path.clone() + "pokemon_stats.csv").decode()
		{
			let (poke_id, stat_id, base_stat, _): (usize, i32, i32, i32) = record.unwrap();

			let cur_poke = match out.base_pokemon.get_mut(poke_id)
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

		// Match Pokemon with their moves.
		for record in get_csv_rdr(path.clone() + "pokemon_moves.csv").decode()
		{
			let (poke_id, _, move_id, _, _, _): (usize, i32, usize, i32, i32, Option<i32>) = record.unwrap();

			match (out.base_pokemon.get_mut(poke_id), moves.get_mut(move_id))
			{
				(Some(p), Some(m)) => p.avail_moves.push(m.clone()),
				_ => {}
			}
		}

		// Read in type efficacies
		for record in get_csv_rdr(path.clone() + "type_efficacy.csv").decode()
		{
			let (damage_type_id, target_type_id, damage_factor): (usize, usize, f64) = record.unwrap();

			// Extend the vectors until they're the right size
			while out.type_efficacies.len() < damage_type_id
			{
				out.type_efficacies.push(Vec::new());
			}

			while out.type_efficacies[damage_type_id - 1].len() < target_type_id
			{
				out.type_efficacies[damage_type_id - 1].push(0.);
			}

			// Store the type efficacy in the (newly-created?) spot
			out.type_efficacies[damage_type_id-1][target_type_id-1] = damage_factor / 100.;
		}

		out
	}

	// Returns the pokemon description for the given Pokemon ID, if it exists.
	// Otherwise returns None
	pub fn get_poke_desc(&self, id: usize) -> Option<&PokeDesc>
	{
		self.base_pokemon.get(id)
	}

	// Returns the type efficacy for the given attacker and defender types
	pub fn get_type_efficacy(&self, attacker_type: i32, defender_type: i32) -> Option<f64>
	{
		match self.type_efficacies.get(attacker_type as usize - 1)
		{
			Some(v) =>
			{
				match v.get(defender_type as usize - 1)
				{
					Some(&n) => Some(n),
					None => None
				}
			},
			None => None
		}
	}
}

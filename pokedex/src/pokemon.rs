extern crate rand;

use self::rand::{random, Closed01};
use pokedex::{MoveDesc,PokeDesc,DamageClass};
use std::cmp;

#[derive(Clone)]
pub enum Status
{
	Paralyzed,
	Poisoned,
	Asleep,
	Healthy
}

#[derive(Clone)]
pub struct Move
{
	id: i32,
	pub desc: MoveDesc,
	pp: i32
}

#[derive(Clone)]
pub struct Pokemon
{
	id: i32,
	desc: PokeDesc,
	// set in config
	moves: Vec<Move>,
	pub level: i32,
	// change during battle
	hp: i32,
	status: Status,
	// these are used for
	// calculating stat
	// multiplier
	crit_stage: i32,
	accuracy_stage: i32,
	evasion_stage: i32,
	attack_stage: i32,
	defense_stage: i32,
	sp_atk_stage: i32,
	sp_def_stage: i32,
	speed_stage: i32,
}

// this is temporary...
fn type_efficacy(_mv_type: &i32, _defender_types: &Vec<i32>) -> (f64)
{
	1.0
}

impl Pokemon
{
	// use gen II calculation method,
	// this is due to simplicity
	pub fn calc_accuracy(&self) -> (f64)
	{
		if self.accuracy_stage < 0
		{
			100.0 / (100.0 - 33.0 * self.accuracy_stage as f64)
		}
		else
		{
			(100.0 + 33.0 * self.accuracy_stage as f64) / 100.0
		}
	}

	pub fn calc_evasion(&self) -> (f64)
	{
		if self.evasion_stage > 0
		{
			100.0 / (100.0 + 33.0 * self.evasion_stage as f64)
		}
		else
		{
			(100.0 - 33.0 * self.evasion_stage as f64) / 100.0
		}
	}

	// this uses the gen II method for the multiplier, but the results
	// are consistent across generations. don't ask me why there is a
	// difference...

	// we are ignoring EV and IV stuff as they are specific
	// to individuals not species
	pub fn calc_stat(&self, stage: i32, base: i32) -> (i32)
	{
		let multiplier = if stage > 0
		{
			(100.0 + 50.0 * stage as f64) / 100.0
		}
		else
		{
			100.0 / (100.0 + 50.0 * stage as f64)
		};

		(((base * self.level) / 50 + 5) as f64 * multiplier) as i32
	}

	
	pub fn calc_hp(&self) -> (i32)
	{
		(self.hp + 50) * self.level / 50 + 10
	}

	// not sure if we want these. I think they make it more
	// clear and concise, but you may think it's unnecessary
	pub fn calc_attack(&self) -> (i32)
	{
		self.calc_stat(self.attack_stage, self.desc.attack)
	}

	pub fn calc_defense(&self) -> (i32)
	{
		self.calc_stat(self.defense_stage, self.desc.defense)
	}

	pub fn calc_sp_atk(&self) -> (i32)
	{
		self.calc_stat(self.sp_atk_stage, self.desc.sp_atk)
	}

	pub fn calc_sp_def(&self) -> (i32)
	{
		self.calc_stat(self.sp_def_stage, self.desc.sp_def)
	}

	pub fn calc_speed(&self) -> (i32)
	{
		self.calc_stat(self.speed_stage, self.desc.speed)
	}

	// calculates the crit modifier based on stage value
	// does not (yet) affect other stat changes. rather it
	// follows the gen II method
	pub fn calc_crit(&self) -> (f64)
	{
		let prob = match self.crit_stage
		{
			0 => 0.0625,
			1 => 0.125,
			2 => 0.25,
			3 => 0.333,
			_ => 0.50,
		};

		let Closed01(res) = random::<Closed01<f64>>();
		if res < prob
		{
			2.0
		}
		else
		{
			1.0
		}
	}

	// 
	pub fn calc_damage(&self, mv: &Move, foe: &Pokemon) -> (i32)
	{
		// status moves don't do damage (hopefully...)
		if mv.desc.damage_class == DamageClass::Status
		{
			return 0;
		}

		// if move is same type as pokemon, does 1.5 damage
		let mut modifier = if foe.desc.type_ids.contains(&mv.desc.type_id)
		{
			1.50
		}
		else
		{
			1.0
		};

		modifier *= type_efficacy(&mv.desc.type_id, &self.desc.type_ids);
		modifier *= self.calc_crit();

		// random modifier from 0.85 to 1.0
		let Closed01(res) = random::<Closed01<f64>>();
		modifier *= 0.85 + 0.15 * res;

		let mut damage = (2 * foe.level + 10) as f64 / 250.0;

		// different damage classes use different stats
		damage *= match mv.desc.damage_class
		{
			DamageClass::Physical => foe.calc_attack() as f64 / self.calc_defense() as f64,
			DamageClass::Special => foe.calc_sp_atk() as f64 / self.calc_sp_def() as f64,
			_ => panic!("tried to deal damage with status move. this should never happen.")
		};

		damage *= mv.desc.power as f64;
		damage += 2.0;
		damage *= modifier;

		damage as i32
	}

	// this implements move effects on the user of the move
	// these include status changes and stat modification
	pub fn use_move(&self, mv: &Move, _foe: &Pokemon) -> ()
	{
		match mv.desc.damage_class
		{
			DamageClass::Status => println!("status moves are not implemented yet for the user. sorry."),
			_ => println!("physical and special moves are not implemented yet for the user. waiting on move data...")
		}
	}

	// this implements move effects on the recipient of the attack
	// these can be status, stat, or damage
	pub fn receive_move(&mut self, mv: &Move, foe: &Pokemon) -> ()
	{
		// currently just deals damage
		self.hp = cmp::max(0, self.hp - self.calc_damage(mv, foe));
	}

	pub fn is_ko(&self) -> (bool)
	{
		self.hp == 0
	}

	// could hard code 4 into AI or could make this return success
	// currently, I'm thinking that hard-coding makes the most sense
	// because returning success seems ambiguous or at least confusing
	pub fn add_move(&mut self, mv: Move) -> ()
	{
		if self.moves.len() < 4
		{
			self.moves.push(mv);
		}
		else
		{
			println!("Unable to add more than 4 moves. This message probably indicates a bug!!");
		}
	}
}

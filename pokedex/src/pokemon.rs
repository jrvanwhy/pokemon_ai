extern crate rand;

use pokedex::{MoveDesc,PokeDesc};

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
	desc: MoveDesc,
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
	accuracy_stage: i32,
	evasion_stage: i32,
	attack_stage: i32,
	defense_stage: i32,
	sp_atk_stage: i32,
	sp_def_stage: i32,
	speed_stage: i32,
}

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

	pub fn calc_damage(&self, attacker: &Pokemon, defender: &Pokemon, mv: &Move) -> (i32)
	{
		// need to get type efficacy information as well
		// TODO: determine interface for type efficacy info
		let mut modifier = if attacker.desc.type_ids.contains(&mv.desc.type_id)
		{
			1.50
		}
		else
		{
			1.0
		};

		modifier *= type_efficacy(&mv.desc.type_id, &defender.desc.type_ids);

		let mut damage = (2 * attacker.level + 10) as f64 / 250.0;
		damage *= attacker.calc_attack() as f64 / defender.calc_defense() as f64;
		damage *= mv.desc.power as f64;
		damage += 2.0;
		damage *= modifier;

		damage as i32
	}

	pub fn use_move(&self, _mv: &Move, _foe: &Pokemon) -> ()
	{
		// not sure yet...
	}

	pub fn receive_move(&self, _mv: &Move, _foe: &Pokemon) -> ()
	{
		// 
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

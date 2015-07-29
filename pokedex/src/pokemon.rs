extern crate rand;

use self::rand::{random, Closed01};
use pokedex::{MoveDesc,PokeDesc,DamageClass,StatMod};
use std::cmp;
use std::fmt;

#[derive(Clone)]
pub enum Status
{
	Paralyzed,
	Poisoned,
	Asleep,
	Healthy
}

#[derive(Clone)]
pub struct Move<'a>
{
	id: i32,
	pub desc: &'a MoveDesc,
	pub pp: i32
}

impl<'a> Move<'a>
{
	pub fn new<'b>(desc: &'b MoveDesc) -> Move<'b>
	{
		Move { id: desc.id,
		       desc: desc,
		       pp: desc.pp
		     }
	}
}

#[derive(Clone)]
pub struct Pokemon<'a>
{
	pub id: i32,
	pub desc: &'a PokeDesc,
	// set in config
	pub moves: Vec<Move<'a>>,
	pub level: i32,
	// change during battle
	pub hp: i32,
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

impl<'a> fmt::Display for Pokemon<'a>
{
	fn fmt(&self, f: &mut fmt::Formatter) -> (fmt::Result)
	{
		write!(f, "{} lvl: {}\n\thp: {}/{} atk: {} def: {}\n\tsp_atk: {} sp_def: {} speed: {}\n\t",
		       self.desc.name, self.level, self.hp, self.calc_hp(), self.calc_attack(), self.calc_defense(),
		       self.calc_sp_atk(), self.calc_sp_def(), self.calc_speed())
	}
}

impl<'a> Pokemon<'a>
{
	pub fn new<'b>(desc: &'b PokeDesc) -> Pokemon<'b>
	{
		Pokemon { id: desc.id,
		          desc: desc,
		          moves: Vec::new(),
		          level: 0,
		          hp: 0,
		          status: Status::Healthy,
		          crit_stage: 0,
		          accuracy_stage: 0,
		          evasion_stage: 0,
		          attack_stage: 0,
		          defense_stage: 0,
		          sp_atk_stage: 0,
		          sp_def_stage: 0,
		          speed_stage: 0
		        }
	}

	pub fn heal(&mut self) -> ()
	{
		self.hp = self.calc_hp();
		self.status = Status::Healthy;

		self.crit_stage = 0;
		self.accuracy_stage = 0;
		self.evasion_stage = 0;
		self.attack_stage = 0;
		self.defense_stage = 0;
		self.sp_atk_stage = 0;
		self.sp_def_stage = 0;
		self.speed_stage = 0;

		for mv in self.moves.iter_mut()
		{
			mv.pp = mv.desc.pp;
		}
	}

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
			100.0 / (100.0 - 50.0 * stage as f64)
		};

		(((base * self.level) / 50 + 5) as f64 * multiplier) as i32
	}

	
	pub fn calc_hp(&self) -> (i32)
	{
		(self.desc.hp + 50) * self.level / 50 + 10
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
			println!("critical hit!");
			2.0
		}
		else
		{
			1.0
		}
	}

	// 
	pub fn calc_damage(&self, mv: &MoveDesc, foe: &Pokemon) -> (i32)
	{
		// status moves don't do damage (hopefully...)
		if mv.damage_class == DamageClass::Status
		{
			return 0;
		}

		// if move is same type as pokemon, does 1.5 damage
		let mut modifier = if foe.desc.type_ids.contains(&mv.type_id)
		{
			println!("same type modifier!");
			1.50
		}
		else
		{
			1.0
		};

		modifier *= type_efficacy(&mv.type_id, &self.desc.type_ids);
		modifier *= self.calc_crit();

		// random modifier from 0.85 to 1.0
		let Closed01(res) = random::<Closed01<f64>>();
		modifier *= 0.85 + 0.15 * res;

		let mut damage = (2 * foe.level + 10) as f64 / 250.0;

		// different damage classes use different stats
		damage *= match mv.damage_class
		{
			DamageClass::Physical => foe.calc_attack() as f64 / self.calc_defense() as f64,
			DamageClass::Special => foe.calc_sp_atk() as f64 / self.calc_sp_def() as f64,
			_ => panic!("tried to deal damage with status move. this should never happen.")
		};

		damage *= mv.power as f64;
		damage += 2.0;
		damage *= modifier;

		damage as i32
	}

	// this implements move effects on the user of the move
	// these include status changes and stat modification
	pub fn use_move(&mut self, mv: &MoveDesc, _foe: &mut Pokemon) -> ()
	{
		// match mv.damage_class
		// {
		// 	DamageClass::Status => println!("status moves are not implemented yet for the user. sorry."),
		// 	_ => println!("physical and special moves are not implemented yet for the user. waiting on move data...")
		// }
		self.modify_stats(mv.user_stat_effects.clone());
	}

	// this implements move effects on the recipient of the attack
	// these can be status, stat, or damage
	pub fn receive_move(&mut self, mv: &MoveDesc, foe: &mut Pokemon) -> ()
	{
		// currently just deals damage
		let dmg = self.calc_damage(mv, foe);

		println!("attack dealt {} damage!", dmg);

		self.hp = cmp::max(0, self.hp - dmg);

		self.modify_stats(mv.recip_stat_effects.clone());
	}

	fn mod_stat(stat: &mut i32, change: i32) -> ()
	{
		*stat += change;

		if *stat > 6
		{
			*stat = 6;
		}
		else if *stat < -6
		{
			*stat = -6;
		}
	}

	pub fn modify_stats(&mut self, stat_effects: Vec<StatMod>) -> ()
	{
		for stat_mod in stat_effects.iter()
		{
			match *stat_mod
			{
				StatMod::Hp(_) => (),
				StatMod::Attack(change) => 
					{
						Pokemon::mod_stat(&mut self.attack_stage, change);
						println!("mod attack by {}", change)
					}
				StatMod::Defense(change) => 
					{
						Pokemon::mod_stat(&mut self.defense_stage, change);
						println!("mod defense by {}", change)
					}
				StatMod::SpAtk(change) => 
					{
						Pokemon::mod_stat(&mut self.sp_atk_stage, change);
						println!("mod sp_atk by {}", change)
					}
				StatMod::SpDef(change) => 
					{
						Pokemon::mod_stat(&mut self.sp_def_stage, change);
						println!("mod sp_def by {}", change)
					}
				StatMod::Speed(change) => 
					{
						Pokemon::mod_stat(&mut self.speed_stage, change);
						println!("mod speed by {}", change)
					}
			}
		}
	}

	

	pub fn is_ko(&self) -> (bool)
	{
		self.hp == 0
	}

	// could hard code 4 into AI or could make this return success
	// currently, I'm thinking that hard-coding makes the most sense
	// because returning success seems ambiguous or at least confusing
	pub fn add_move(&mut self, mv: Move<'a>) -> ()
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

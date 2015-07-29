extern crate pokedex;
extern crate rand;

use pokedex::{Pokemon, Move, MoveDesc, Accuracy};
use rand::{random, Open01};

pub enum Action
{
	Attack,
	Pokemon,
	Item
}

pub trait PokePlayer<'a>
{
	fn get_name(&self) -> (String);

	fn get_team(&self) -> (&Team);

	fn get_team_mut(&mut self) -> (&mut Team<'a>);

	fn choose_move(&self) -> (usize);

	fn choose_pkn(&self) -> (usize);

	fn choose_action(&self) -> (Action);
}

#[derive(Clone)]
pub struct Team<'a>
{
	team: Vec<Pokemon<'a>>,
}

impl<'a> Team<'a>
{
	pub fn new() -> Team<'a>
	{
		Team {team: Vec::new()}
	}

	pub fn get_team(&self) -> (Vec<Pokemon>)
	{
		self.team.clone()
	}

	fn is_defeated(&self) -> (bool)
	{
		let mut defeated = true;
		for pkn in self.team.iter()
		{
			defeated = defeated && pkn.is_ko();
		}

		defeated
	}

	pub fn add_pkn(&mut self, pkn: Pokemon<'a>) -> ()
	{
		self.team.push(pkn);
	}

	pub fn get_cur_pkn(&self) -> (Option<&Pokemon>)
	{
		match self.team.get(0)
		{
			Some(pkn) =>
				{
					if !pkn.is_ko()
					{
						Some(pkn)
					}
					else
					{
						None
					}
				},
			None => None
		}
	}

	fn get_cur_pkn_mut(&mut self) -> (Option<&mut Pokemon<'a>>)
	{
		match self.team.get_mut(0)
		{
			Some(pkn) =>
				{
					if !pkn.is_ko()
					{
						Some(pkn)
					}
					else
					{
						None
					}
				},
			None => None
		}
	}

	pub fn get_move(&self, ind: usize) -> (Option<&Move>)
	{
		self.get_cur_pkn().unwrap().moves.get(ind)
	}

	fn get_move_desc(&self, ind: usize) -> (Option<&MoveDesc>)
	{
		match self.get_move(ind)
		{
			Some(mv) => Some(mv.desc),
			None => None
		}
	}

	fn set_move_pp(&mut self, ind: usize) -> ()
	{
		match self.get_cur_pkn_mut().unwrap().moves.get_mut(ind)
		{
			Some(mv) => mv.pp -= 1,
			None => panic!("move not found. this is not good.")
		}
	}

	fn set_cur_pkn(&mut self, ind: usize) -> (bool)
	{
		if ind < self.team.len()
		{
			if !self.team[ind].is_ko()
			{
				self.team.swap(0, ind);

				true
			}
			else
			{
				println!("tried to send out ko'd pokemon!");

				false
			}
			
		}
		else
		{
			println!("tried to set cur pokemon to pokemon out of range!");

			false
		}
	}
}


pub fn battle<'a, T1: PokePlayer<'a>, T2: PokePlayer<'a>>(p1: &mut T1, p2: &mut T2) -> (String)
{
	loop
	{
		// need to figure out a better way...
		let finished = if p1.get_team().get_cur_pkn().unwrap().calc_speed() > p2.get_team().get_cur_pkn().unwrap().calc_speed()
		{
			play_turn(p1, p2)
		}
		else
		{
			play_turn(p2, p1)
		};

		if finished
		{
			if p1.get_team().is_defeated()
			{
				return p2.get_name();
			}
			else if p2.get_team().is_defeated()
			{
				return p1.get_name();
			}
		}
	}
}

fn play_turn<'a, T1: PokePlayer<'a>, T2: PokePlayer<'a>>(p1: &mut T1, p2: &mut T2) -> (bool)
{
	println!("{}'s turn", p1.get_name());
	play_action(p1, p2);

	if p1.get_team().is_defeated() || p2.get_team().is_defeated()
	{
		return true;
	}

	println!("{}'s turn", p2.get_name());
	play_action(p2, p1);

	if p1.get_team().is_defeated() || p2.get_team().is_defeated()
	{
		return true;
	}

	false
}

fn play_action<'a, T1: PokePlayer<'a>, T2: PokePlayer<'a>>(cur_p: &mut T1, oth_p: &mut T2) -> ()
{
	// choose between menu options
	match cur_p.choose_action()
	{
		Action::Attack =>
			{
				// moves can affect both attacker
				// and attacked
				let mv_ind = cur_p.choose_move();

				if cur_p.get_team().get_move(mv_ind).unwrap().pp > 0
				{
					let hit_chance = match cur_p.get_team().get_move_desc(mv_ind).unwrap().accuracy
					{
						Accuracy::Finite(val) => val * cur_p.get_team().get_cur_pkn().unwrap().calc_accuracy()
					               / oth_p.get_team().get_cur_pkn().unwrap().calc_evasion(),
					    Accuracy::Infinite => 1.0
					};

					// hit with prob hit_chance
					let Open01(res) = random::<Open01<f64>>();
					if hit_chance > res
					{
						cur_p.get_team_mut().set_move_pp(mv_ind);

						let mv = cur_p.get_team().get_move_desc(mv_ind).unwrap().clone();
						cur_p.get_team_mut().get_cur_pkn_mut().unwrap().use_move(&mv, &mut oth_p.get_team_mut().get_cur_pkn_mut().unwrap());
						
						// replace ko'd pokemon
						if !replace_ko(cur_p)
						{
							return ();
						}
						if !replace_ko(oth_p)
						{
							return ();
						}

						oth_p.get_team_mut().get_cur_pkn_mut().unwrap().receive_move(&mv, &mut cur_p.get_team_mut().get_cur_pkn_mut().unwrap());

						// replace ko'd pokemon
						if !replace_ko(cur_p)
						{
							return ();
						}
						if !replace_ko(oth_p)
						{
							return ();
						}
						
						// match cur_p.get_team().get_cur_pkn()
						// {
						// 	Some(_) => (),
						// 	None =>
						// 		{
						// 			if cur_p.get_team().is_defeated()
						// 			{
						// 				return;
						// 			}
						// 			println!("player 1, your pokemon fainted. pick another.");
						// 			loop
						// 			{
						// 				let pkn_ind = cur_p.choose_pkn();
						// 				if cur_p.get_team_mut().set_cur_pkn(pkn_ind)
						// 				{
						// 					break;
						// 				}
						// 			}
									
						// 		}
						// }

						// match oth_p.get_team().get_cur_pkn()
						// {
						// 	Some(_) => (),
						// 	None =>
						// 		{
						// 			if oth_p.get_team().is_defeated()
						// 			{
						// 				return ();
						// 			}
						// 			println!("player 2, your pokemon fainted. pick another.");
						// 			loop
						// 			{
						// 				let pkn_ind = oth_p.choose_pkn();
						// 				if oth_p.get_team_mut().set_cur_pkn(pkn_ind)
						// 				{
						// 					break;
						// 				}
						// 			}
						// 		}
						// }
					}
					else
					{
						println!("attack missed!");
					}
				}
				else
				{
					println!("move is out of power points");
				}
			},
		Action::Pokemon =>
			{
				loop
				{
					let pkn_ind = cur_p.choose_pkn();
					if cur_p.get_team_mut().set_cur_pkn(pkn_ind)
					{
						break;
					}
				}
			},
		Action::Item =>
			{
				println!("No items, sorry... Maybe later.");
				play_turn(cur_p, oth_p);
			}
	}
}

fn replace_ko<'a, T: PokePlayer<'a>>(p: &mut T) -> (bool)
{
	match p.get_team().get_cur_pkn()
	{
		Some(_) => true,
		None =>
			{
				if p.get_team().is_defeated()
				{
					return false;
				}
				println!("{}, your pokemon fainted. pick another.", p.get_name());
				loop
				{
					let pkn_ind = p.choose_pkn();
					if p.get_team_mut().set_cur_pkn(pkn_ind)
					{
						break;
					}
				}
				true
			}
	}
}
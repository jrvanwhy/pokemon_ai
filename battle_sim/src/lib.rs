extern crate pokedex;
extern crate rand;

use pokedex::{Pokemon, Move};
use rand::{random, Closed01};
use std::io;

pub enum Action
{
	Attack,
	Pokemon,
	Item
}

pub trait PokePlayer<'a>
{
	fn is_defeated(&self) -> (bool);

	fn get_cur_pkn(&mut self) -> (&mut Pokemon<'a>);

	fn choose_move(&mut self) -> (Move<'a>);

	fn choose_pkn(&mut self) -> ();

	fn choose_action(&self) -> (Action);
}

pub struct HumanPlayer<'a>
{
	pub team: Vec<Pokemon<'a>>,
}

impl<'a> HumanPlayer<'a>
{
	pub fn new<'b>() -> HumanPlayer<'b>
	{
		HumanPlayer {team: Vec::new() }
	}
}

impl<'a> PokePlayer<'a> for HumanPlayer<'a>
{
	fn is_defeated(&self) -> (bool)
	{
		let mut defeated = false;
		for pkn in self.team.iter()
		{
			defeated &= pkn.is_ko();
		}

		defeated
	}

	fn get_cur_pkn(&mut self) -> (&mut Pokemon<'a>)
	{
		if self.team.len() > 0
		{
			&mut self.team[0]
		}
		else
		{
			panic!("tried to get current pokemon when team is empty");
		}
	}

	fn choose_move(&mut self) -> (Move<'a>)
	{
		loop
		{
			for (i, mv) in self.get_cur_pkn().moves.iter().enumerate()
			{
				println!("\t[{}] - {}", i, mv.desc.name);
			}

			println!("choose move: ");

			let mut option = String::new();
			io::stdin().read_line(&mut option)
			    .ok()
			    .expect("failed to read line");

			let option: usize = option.trim().parse()
			    .ok()
			    .expect("please type a number!");

			if option >= self.get_cur_pkn().moves.len()
			{
				println!("please choose from the options displayed");
			}
			else
			{
				return self.get_cur_pkn().moves[option].clone();
			}
		}
	}

	fn choose_pkn(&mut self) -> ()
	{
		loop
		{
			for (i, pkn) in self.team.iter().enumerate()
			{
				println!("\t[{}] - {}", i, pkn.desc.name);
			}

			println!("choose pokemon: ");

			let mut option = String::new();
			io::stdin().read_line(&mut option)
			    .ok()
			    .expect("failed to read line");

			let option: usize = option.trim().parse()
			    .ok()
			    .expect("please type a number!");

			if option >= self.team.len()
			{
				println!("please choose from the options displayed");
			}
			else
			{
				self.team.swap(0, option);
				break;
			}
		}
	}

	fn choose_action(&self) -> (Action)
	{
		loop
		{
			println!("\t[0] - Attack");
			println!("\t[1] - Pokemon");
			println!("\t[2] - Item");

			println!("choose action: ");

			let mut option = String::new();
			io::stdin().read_line(&mut option)
			    .ok()
			    .expect("failed to read line");

			let option: u32 = option.trim().parse()
			    .ok()
			    .expect("please type a number!");

			match option
			{
				0 => return Action::Attack,
				1 => return Action::Pokemon,
				2 => return Action::Item,
				_ => println!("please choose from the options displayed")
			}
		}
	}
}

pub fn battle<'a, T1: PokePlayer<'a>, T2: PokePlayer<'a>>(p1: &mut T1, p2: &mut T2) -> (i32)
{
	loop
	{
		// need to figure out a better way...
		let finished = if p1.get_cur_pkn().calc_speed() > p2.get_cur_pkn().calc_speed()
		{
			play_turn(p1, p2)
		}
		else
		{
			play_turn(p2, p1)
		};

		if finished
		{
			if p1.is_defeated()
			{
				return 2;
			}
			else if p2.is_defeated()
			{
				return 1;
			}
		}
	}
}

fn play_turn<'a, T1: PokePlayer<'a>, T2: PokePlayer<'a>>(p1: &mut T1, p2: &mut T2) -> (bool)
{
	println!("player 1's turn");
	play_action(p1, p2);

	if p1.is_defeated() || p2.is_defeated()
	{
		return false;
	}

	println!("player 2's turn");
	play_action(p2, p1);

	if p1.is_defeated() || p2.is_defeated()
	{
		return false;
	}

	true
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
				let mv = cur_p.choose_move();

				let hit_chance = mv.desc.accuracy
				               * cur_p.get_cur_pkn().calc_accuracy()
				               / oth_p.get_cur_pkn().calc_evasion();

				// hit with prob hit_chance
				let Closed01(res) = random::<Closed01<f64>>();
				if hit_chance > res
				{
					cur_p.get_cur_pkn().use_move(&mv, &oth_p.get_cur_pkn());
					oth_p.get_cur_pkn().receive_move(&mv, &cur_p.get_cur_pkn());

					// replace ko'd pokemon
					if cur_p.get_cur_pkn().is_ko()
					{
						cur_p.choose_pkn();
					}

					if oth_p.get_cur_pkn().is_ko()
					{
						oth_p.choose_pkn();
					}
				}
			},
		Action::Pokemon =>
			{
				cur_p.choose_pkn();
			},
		Action::Item =>
			{
				println!("No items, sorry...");
				play_turn(cur_p, oth_p);
			}
	}
}
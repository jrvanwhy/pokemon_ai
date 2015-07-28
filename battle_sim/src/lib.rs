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


pub trait PokePlayer
{
	// fn get_team(&self) -> (Vec<Pokemon>);

	fn is_defeated(&self) -> (bool);

	fn get_cur_pkn(&self) -> (Pokemon);

	fn choose_move(&self) -> (Move);

	fn choose_pkn(&self) -> ();

	fn choose_action(&self) -> (Action);
}

struct HumanPlayer
{
	team: Vec<Pokemon>,
	name: str
}

impl PokePlayer for HumanPlayer
{
	fn is_defeated(&self) -> (bool)
	{
		let mut defeated = false;
		for pkn in self.team
		{
			defeated &= pkn.is_ko();
		}

		defeated
	}

	fn get_cur_pkn(&self) -> (Pokemon)
	{
		if self.team.len() > 0
		{
			self.team[0]
		}
		else
		{
			panic!("tried to get current pokemon when team is empty");
		}
	}

	fn choose_move(&self) -> Move
	{
		loop
		{
			for (i, mv) in self.get_cur_pkn().moves().enumerate()
			{
				println!("\t[{}] - {}", i, mv.desc.name);
			}

			print!("choose move: ");

			let mut option = String::new();
			io::stdin().read_line(&mut option)
			    .ok()
			    .expect("failed to read line");

			let option: u32 = option.trim().parse()
			    .ok()
			    .expect("please type a number!");

			if option >= self.get_cur_pkn().moves().len()
			{
				println!("please choose from the options displayed");
			}
			else
			{
				return self.get_cur_pkn.moves()[option];
			}
		}
		
	}

}

pub fn battle<T1: PokePlayer, T2: PokePlayer>(p1: T1, p2: T2) -> (i32)
{
	loop
	{
		// need to figure out a better way...
		let finished = if p1.get_cur_pkn().calc_speed() > p2.get_cur_pkn().calc_speed()
		{
			play_turn(&p1, &p2)
		}
		else
		{
			play_turn(&p2, &p1)
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

fn play_turn<T1: PokePlayer, T2: PokePlayer>(p1: &T1, p2: &T2) -> (bool)
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

fn play_action<T1: PokePlayer, T2: PokePlayer>(cur_p: &T1, oth_p: &T2) -> ()
{
	// choose between menu options
	match cur_p.choose_action()
	{
		Action::Attack =>
			{
				// moves can affect both attacker
				// and attacked
				let mv = cur_p.choose_move();

				// 
				let hit_chance = mv.desc.accuracy
				               * cur_p.get_cur_pkn().calc_accuracy()
				               / oth_p.get_cur_pkn().calc_evasion();

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
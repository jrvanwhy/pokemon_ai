extern crate battle_sim;
extern crate pokedex;

use battle_sim::PokePlayer;
use pokedex::Pokemon;
use pokedex::Move;
use pokedex::MoveDesc;
use battle_sim::Action;

use std::io;


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

	fn choose_move(&mut self) -> (usize)
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
				return option;
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

	fn choose_action(&mut self) -> (Action)
	{
		loop
		{
			let cur_pkn = self.get_cur_pkn();
			println!("cur pkn: {}; hp: {} / {}", cur_pkn.desc.name, cur_pkn.hp, cur_pkn.desc.hp);
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

	fn get_move(&mut self, ind: usize) -> (Move)
	{
		match self.get_cur_pkn().moves.get(ind)
		{
			Some(mv) => mv.clone(),
			None => panic!("I guess I should have done better error checking, but I'm tired...")
		}
	}

	fn get_move_desc(&mut self, ind: usize) -> (MoveDesc)
	{
		match self.get_cur_pkn().moves.get(ind)
		{
			Some(mv) => mv.desc.clone(),
			None => panic!("I guess I should have done better error checking, but I'm tired...")
		}
	}

	fn set_move_pp(&mut self, ind: usize) -> ()
	{
		match self.get_cur_pkn().moves.get_mut(ind)
		{
			Some(mv) => mv.pp -= 1,
			None => panic!("move not found. this is not good.")
		}
	}
}
extern crate battle_sim;
extern crate pokedex;

use battle_sim::PokePlayer;
use battle_sim::Action;
use battle_sim::Team;
// use pokedex::Pokemon;
// use pokedex::Move;
// use pokedex::MoveDesc;

use std::io;


pub struct HumanPlayer<'a>
{
	name: String,
	pub team: Team<'a>,
}

impl<'a> HumanPlayer<'a>
{
	pub fn new<'b>(name: String) -> HumanPlayer<'b>
	{
		HumanPlayer {name: name, team: Team::new()}
	}
}

impl<'a> PokePlayer<'a> for HumanPlayer<'a>
{
	fn get_name(&self) -> (String)
	{
		self.name.clone()
	}

	fn get_team(&self) -> (&Team)
	{
		&self.team
	}

	fn get_team_mut(&mut self) -> (&mut Team<'a>)
	{
		&mut self.team
	}

	fn choose_move(&self) -> (usize)
	{
		loop
		{
			for (i, mv) in self.team.get_cur_pkn().unwrap().moves.iter().enumerate()
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

			if option >= self.team.get_cur_pkn().unwrap().moves.len()
			{
				println!("please choose from the options displayed");
			}
			else
			{
				return option;
			}
		}
	}

	fn choose_pkn(&self) -> (usize)
	{
		loop
		{
			for (i, pkn) in self.team.get_team().iter().enumerate()
			{
				println!("\t[{}] - {}", i, pkn);
			}

			println!("choose pokemon: ");

			let mut option = String::new();
			io::stdin().read_line(&mut option)
			    .ok()
			    .expect("failed to read line");

			let option: usize = option.trim().parse()
			    .ok()
			    .expect("please type a number!");

			if option >= self.team.get_team().len()
			{
				println!("please choose from the options displayed");
			}
			else
			{
				return option;
			}
		}
	}

	fn choose_action(&self) -> (Action)
	{
		loop
		{
			let cur_pkn = self.team.get_cur_pkn().unwrap();
			println!("cur pkn:\n{}", cur_pkn);
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

	// fn get_move(&mut self, ind: usize) -> (Move)
	// {
	// 	match self.get_cur_pkn().moves.get(ind)
	// 	{
	// 		Some(mv) => mv.clone(),
	// 		None => panic!("I guess I should have done better error checking, but I'm tired...")
	// 	}
	// }

	// fn get_move_desc(&mut self, ind: usize) -> (MoveDesc)
	// {
	// 	match self.get_cur_pkn().moves.get(ind)
	// 	{
	// 		Some(mv) => mv.desc.clone(),
	// 		None => panic!("I guess I should have done better error checking, but I'm tired...")
	// 	}
	// }

	// fn set_move_pp(&mut self, ind: usize) -> ()
	// {
	// 	match self.get_cur_pkn().moves.get_mut(ind)
	// 	{
	// 		Some(mv) => mv.pp -= 1,
	// 		None => panic!("move not found. this is not good.")
	// 	}
	// }
}
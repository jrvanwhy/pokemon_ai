extern crate battle_sim;
extern crate pokedex;

use battle_sim::PokePlayer;
use battle_sim::Action;
use battle_sim::Team;

use std::io;


pub struct HumanPlayer<'a>
{
	name: String,
	team: Team<'a>,
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

	fn choose_move<'b, T: PokePlayer<'b>>(&self, _foe: &T) -> (usize)
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

	fn choose_pkn<'b, T: PokePlayer<'b>>(&self, _foe: &T) -> (usize)
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

	fn choose_action<'b, T: PokePlayer<'b>>(&self, foe: &T) -> (Action)
	{
		loop
		{
			let cur_pkn = self.team.get_cur_pkn().unwrap();
			let foe_pkn = foe.get_team().get_cur_pkn().unwrap();
			println!("cur pkn:\n{}", cur_pkn);
			println!("foe pkn:\n{}", foe_pkn);
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
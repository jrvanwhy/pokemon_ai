extern crate battle_sim;
extern crate pokedex;

use battle_sim::PokePlayer;
use battle_sim::Action;
use battle_sim::Team;


pub struct SimpleAI<'a>
{
	name: String,
	team: Team<'a>,
}

impl<'a> SimpleAI<'a>
{
	pub fn new<'b>(name: String) -> SimpleAI<'b>
	{
		SimpleAI {name: name, team: Team::new()}
	}
}

impl<'a> PokePlayer<'a> for SimpleAI<'a>
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
		// choose first move with pp left
		for (i, mv) in self.team.get_cur_pkn().unwrap().moves.iter().enumerate()
		{
			if mv.pp > 0
			{
				return i;
			}
		}
		return 0;
	}

	fn choose_pkn<'b, T: PokePlayer<'b>>(&self, _foe: &T) -> (usize)
	{
		// choose first conscious pokemon
		for (i, pkn) in self.team.get_team().iter().enumerate()
		{
			if !pkn.is_ko()
			{
				return i;
			}
		}
		return 0;
	}

	fn choose_action<'b, T: PokePlayer<'b>>(&self, _foe: &T) -> (Action)
	{
		// why not?
		Action::Attack
	}
}
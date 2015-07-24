extern crate pokedex;
use pokedex::Pokemon;

pub trait PokePlayer
{
	fn get_team(&self) -> (Vec<Pokemon>);

	fn is_defeated(&self) -> (bool);

	fn get_cur_pkn(&self) -> (Pokemon);

	fn choose_move(&self) -> (i32);

	fn choose_pkn(&self) -> (i32);

	fn choose_action(&self) -> (i32);
}

// struct HumanPlayer
// {
// 	team: Vec<Pokemon>,
// 	name: str
// }

// impl PokePlayer for HumanPlayer
// {
// 	fn get_team(self) -> Vec<Pokemon>
// 	{
// 		self.team.clone()
// 	}
// }

// struct BattleData
// {
//     cur_player: i32,
// }

pub fn battle<T1: PokePlayer, T2: PokePlayer>(p1: T1, p2: T2) -> (bool)
{
	loop
	{


		if p1.is_defeated()
		{
			return false;
		}
		else if p2.is_defeated()
		{
			return true;
		}
	}
}
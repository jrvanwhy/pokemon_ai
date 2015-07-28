extern crate pokedex;
extern crate rand;

use pokedex::{Pokemon, Move};
use rand::{random, Closed01};

pub enum Action
{
	Attack,
	Pokemon,
	Item
}


pub trait PokePlayer
{
	fn get_team(&self) -> (Vec<Pokemon>);

	fn is_defeated(&self) -> (bool);

	fn get_cur_pkn(&self) -> (Pokemon);

	fn set_cur_pkn(&self, Pokemon) -> ();

	fn choose_move(&self) -> (Move);

	fn choose_pkn(&self) -> (Pokemon);

	fn choose_action(&self) -> (Action);
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

		        // if hit_chance > rand::thread_rng().gen_range::<f64>(0, 1)
		        let Closed01(res) = random::<Closed01<f64>>();
		        if hit_chance > res
				{
					cur_p.get_cur_pkn().use_move(&mv, &oth_p.get_cur_pkn());
					oth_p.get_cur_pkn().receive_move(&mv, &cur_p.get_cur_pkn());

					// replace ko'd pokemon
					if cur_p.get_cur_pkn().is_ko()
					{
						let pkn = cur_p.choose_pkn();
						cur_p.set_cur_pkn(pkn);
					}

					if oth_p.get_cur_pkn().is_ko()
					{
						let pkn = oth_p.choose_pkn();
						oth_p.set_cur_pkn(pkn);
					}
				}
			},
		Action::Pokemon =>
			{
				let pkn = cur_p.choose_pkn();
				cur_p.set_cur_pkn(pkn);
			},
		Action::Item =>
			{
				println!("No items, sorry...");
				play_turn(cur_p, oth_p);
			}
	}
}
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
	fn is_defeated(&self) -> (bool);

	fn get_cur_pkn(&mut self) -> (&mut Pokemon<'a>);

	fn choose_move(&mut self) -> (usize);

	fn choose_pkn(&mut self) -> ();

	fn choose_action(&self) -> (Action);

	fn get_move(&mut self, usize) -> (Move);

	fn get_move_desc(&mut self, usize) -> (MoveDesc);

	fn set_move_pp(&mut self, usize) -> ();
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
				let mv_ind = cur_p.choose_move();

				if cur_p.get_move(mv_ind).pp > 0
				{
					let hit_chance = match cur_p.get_move_desc(mv_ind).accuracy
					{
						Accuracy::Finite(val) => val * cur_p.get_cur_pkn().calc_accuracy()
					               / oth_p.get_cur_pkn().calc_evasion(),
					    Accuracy::Infinite => 1.0
					};

					// hit with prob hit_chance
					let Open01(res) = random::<Open01<f64>>();
					if hit_chance > res
					{
						let mv = cur_p.get_move_desc(mv_ind);
						cur_p.get_cur_pkn().use_move(&mv, &mut oth_p.get_cur_pkn());
						oth_p.get_cur_pkn().receive_move(&mv, &mut cur_p.get_cur_pkn());

						cur_p.set_move_pp(mv_ind);

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
				}
				else
				{
					println!("move is out of power points");
				}
			},
		Action::Pokemon =>
			{
				cur_p.choose_pkn();
			},
		Action::Item =>
			{
				println!("No items, sorry... Maybe later.");
				play_turn(cur_p, oth_p);
			}
	}
}
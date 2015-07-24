extern crate pokedex;
use pokedex::Pokemon;

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

	fn choose_move(&self) -> (i32);

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
	let mut whose_turn = if p1.get_cur_pkn().speed > p2.get_cur_pkn().speed
	{
		1
	}
	else
	{
		2
	};

	loop
	{
		if whose_turn == 1
		{
			println!("player 1's turn");
			play_turn(&p1, &p2);
			whose_turn = 2;
		}
		else
		{
			println!("player 2's turn");
			play_turn(&p2, &p1);
			whose_turn = 1;
		}

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

fn play_turn<T1: PokePlayer, T2: PokePlayer>(cur_p: &T1, oth_p: &T2) -> ()
{
	// choose between menu options
	match cur_p.choose_action()
	{
		Action::Attack =>
			{
				// moves can affect both attacker
				// and attacked
				let mv = cur_p.choose_move();
				cur_p.get_cur_pkn().use_move(mv);
				oth_p.get_cur_pkn().use_move_on(mv);

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
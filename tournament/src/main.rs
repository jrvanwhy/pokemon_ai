extern crate pokedex;
extern crate battle_sim;
extern crate toml;

mod human_player;
// use std::fs::File;
use std::env;

use pokedex::Pokedex;
use pokedex::Pokemon;
use pokedex::Move;
use human_player::HumanPlayer;

fn configure_player<'a>(dex: &'a Pokedex, player: &mut HumanPlayer<'a>, team: Vec<(usize, Vec<i32>)>)
{
	for (id, moves) in team
	{
		let desc = match dex.get_poke_desc(id)
		{
			Some(desc) => desc,
			None => panic!("failed to load pokemon {} from pokedex", id)
		};

		let mut pkn = Pokemon::new(&desc);

		pkn.level = 50;

		for move_id in moves
		{
			let move_desc = match pkn.desc.get_move_desc(move_id)
			{
				Some(move_desc) => move_desc,
				None => panic!("failed to load move {} from pokedex", move_id)
			};

			let mv = Move::new(&move_desc);

			pkn.add_move(mv);
		}

		player.team.push(pkn);
	}
}

fn main() {
	let args: Vec<_> = env::args().collect();

	let dex: Pokedex;
	if args.len() == 2
	{
		dex = Pokedex::new(args[1].clone());
	}
	else
	{
		panic!("need 1 arg");
	}

	let mut p1 = HumanPlayer::new();
	let mut p2 = HumanPlayer::new();

	let mut mv1 = Vec::new();
	mv1.push(14);
	mv1.push(15);
	mv1.push(22);
	mv1.push(33);

	let mut mv2 = Vec::new();
	mv2.push(189);
	mv2.push(202);
	mv2.push(203);
	mv2.push(207);

	let mut mv3 = Vec::new();
	mv3.push(164);
	mv3.push(188);
	mv3.push(148);
	mv3.push(218);

	let mut mv4 = Vec::new();
	mv4.push(157);
	mv4.push(184);
	mv4.push(200);
	mv4.push(241);

	let mut mv5 = Vec::new();
	mv5.push(99);
	mv5.push(102);
	mv5.push(104);
	mv5.push(126);

	let mut mv6 = Vec::new();
	mv6.push(52);
	mv6.push(53);
	mv6.push(91);
	mv6.push(92);

	let mut p1_team = Vec::new();
	p1_team.push((1, mv1));
	p1_team.push((2, mv2));
	p1_team.push((3, mv3));

	let mut p2_team = Vec::new();
	p2_team.push((4, mv4));
	p2_team.push((5, mv5));
	p2_team.push((6, mv6));

	configure_player(&dex, &mut p1, p1_team);
	configure_player(&dex, &mut p2, p2_team);

	println!("winner: {}", battle_sim::battle(&mut p1, &mut p2));
}

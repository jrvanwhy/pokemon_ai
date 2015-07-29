extern crate pokedex;
extern crate battle_sim;
extern crate toml;

// use std::fs::File;
use std::env;

use pokedex::Pokedex;
use pokedex::Pokemon;
use pokedex::Move;
use battle_sim::HumanPlayer;

// fn load_player_config(String config_path) -> (HumanPlayer)
// {
// 	let mut config_file = try!(File::open(config_path));
// 	let mut config_contents = String::new();

// 	try!(config_file.read_to_string(&mut config_contents));

// 	let mut parser = toml::Parser::new(&config_contents);
// 	let toml_val = match parser.parse()
// 	{
// 		Some(toml_val) => toml_val,
// 		None => panic!("failed to parse TOML configuration");
// 	}

// 	let mut decoder = toml::Deocder::new(toml_val);
// 	let config = try!(HumanPlayer::decode(&mut decoder));

// 	config
// }

fn configure_player<'a>(dex: &'a Pokedex, player: &mut HumanPlayer<'a>, team: Vec<(usize, Vec<usize>)>)
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

			let mut mv = Move::new(&move_desc);

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

	let mut p1_team = Vec::new();
	p1_team.push((1, Vec::new()));
	p1_team.push((2, Vec::new()));
	p1_team.push((3, Vec::new()));

	let mut p2_team = Vec::new();
	p2_team.push((4, Vec::new()));
	p2_team.push((5, Vec::new()));
	p2_team.push((6, Vec::new()));

	configure_player(&dex, &mut p1, p1_team);
	configure_player(&dex, &mut p2, p2_team);

	println!("winner: {}", battle_sim::battle(&mut p1, &mut p2));
}

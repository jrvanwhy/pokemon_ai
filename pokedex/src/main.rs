use std::env;

mod pokedex;

fn main() {
	let args: Vec<_> = env::args().collect();

	let pd: pokedex::Pokedex;
	if args.len() == 2
	{
		pd = pokedex::Pokedex::new(args[1].clone());
	}
	else
	{
		panic!("need 1 arg");
	}
	println!("{:?}", pd.get_poke_desc(10));
	println!("{:?}", pd.get_poke_desc(15));
	println!("{:?}", pd.get_poke_desc(15).unwrap().get_move_desc(2));
	println!("{:?}", pd.get_type_efficacy(4, 8));
}

mod pokedex;

fn main() {
	let pd = pokedex::Pokedex::new("/home/ryan/repos/pokemon_ai/csv/".to_string());
	println!("{:?}", pd.get_poke_desc(10));
	println!("{:?}", pd.get_poke_desc(15));
	println!("{:?}", pd.get_poke_desc(15).unwrap().get_move_desc(2));
	println!("{:?}", pd.get_type_efficacy(4, 8));
}

mod pokedex;

fn main() {
	let pd = pokedex::Pokedex::new();
	println!("{:?}", pd.get_poke_desc(10));
	println!("{:?}", pd.get_poke_desc(15));
}

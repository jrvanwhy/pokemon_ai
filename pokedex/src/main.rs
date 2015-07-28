mod pokedex;

fn main() {
	let pd = pokedex::Pokedex::new();
	println!("{:?}", pd.get_poke_desc(10));
}

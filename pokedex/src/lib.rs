// Import the sub-modules
mod pokedex;
mod pokemon;

// Define the public interface
pub use pokedex::Pokedex;
pub use pokemon::Pokemon;
pub use pokemon::Move;
pub use pokedex::MoveDesc;
pub use pokedex::StatMod;
pub use pokedex::PokeDesc;
pub use pokedex::Accuracy;

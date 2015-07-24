pub enum Status
{
	Paralyzed,
	Poisoned,
	Asleep,
	Healthy
}

#[derive(Clone)]
pub struct Pokemon
{
	// identifiers
	id: i32,
	// current state
	status: i32,
	accuracy: i32,
	hp: i32,
	attack: i32,
	defense: i32,
	sp_atk: i32,
	sp_def: i32,
	pub speed: i32,
}

impl Pokemon
{
	pub fn use_move(&self, mv: i32) -> ()
	{
	}

	pub fn use_move_on(&self, mv: i32) -> ()
	{
	}

	pub fn is_ko(&self) -> (bool)
	{
		false
	}
}
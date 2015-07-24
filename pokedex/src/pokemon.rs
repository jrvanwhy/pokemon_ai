use pokedex::PokeDesc;

pub enum Status
{
	Paralyzed,
	Poisoned,
	Asleep,
	Healthy
}

pub enum DamageClass
{
	Status,
	Physical,
	Special
}

#[derive(Clone)]
pub struct Pokemon
{
	// identifiers
	id: i32,
	// current state
	status: Status,
	accu_evas_stage: i32,
	hp: i32,
	attack_stage: i32,
	defense_stage: i32,
	sp_atk_stage: i32,
	sp_def_stage: i32,
	speed_stage: i32,
	stats: PokeDesc,
	moves: Vec<Move>,
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

	pub fn add_move(&self, mv: Move) -> ()
	{
	}
}

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
	speed: i32,
}

impl Pokemon
{
	fn get_attack(self) -> (i32)
	{
		42
	}

	fn get_defense(self) -> (i32)
	{
		42
	}
	
	fn get_sp_atk(self) -> (i32)
	{
		42
	}
	
	fn get_sp_def(self) -> (i32)
	{
		42
	}
	
	fn get_hp(self) -> (i32)
	{
		42
	}
	
	fn get_speed(self) -> (i32)
	{
		42
	}
	
}

pub trait PokePlayer
{
	fn get_team(self) -> (Vec<Pokemon>);

	fn is_defeated(self) -> (bool);

	fn get_cur_pkn(self) -> (Pokemon);

	// fn 

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

// struct BattleData
// {
//     cur_player: i32,
// }

pub fn battle<T1: PokePlayer, T2: PokePlayer>(_p1: T1, _p2: T2) -> (bool)
{



	true
}
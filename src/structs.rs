use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CharInfo {
    pub defense: String,
    pub guts: String,
    pub guard_balance: String,
    pub prejump: String,
    pub umo: String,
    pub forward_dash: String,
    pub backdash: String,
    pub backdash_duration: String,
    pub backdash_invincibility: String,
    pub backdash_airborne: String,
    pub backdash_distance: String,
    pub jump_duration: String,
    pub jump_height: String,
    pub high_jump_duration: String,
    pub high_jump_height: String,
    pub earliest_iad: String,
    pub ad_duration: String,
    pub ad_distance: String,
    pub abd_duration: String,
    pub abd_distance: String,
    pub movement_tension: String,
    pub jump_tension: String,
    pub airdash_tension: String,
    pub walk_speed: String,
    pub back_walk_speed: String,
    pub dash_initial_speed: String,
    pub dash_acceleration: String,
    pub dash_friction: String,
    pub jump_gravity: String,
    pub high_jump_gravity: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveInfo {
    pub input: String,
    pub name: String,
    pub damage: String,
    pub guard: String,
    pub startup: String,
    pub active: String,
    pub recovery: String,
    pub on_hit: String,
    pub on_block: String,
    pub level: String,
    pub counter: String,
    pub move_type: String,
    pub risc_gain: String,
    pub risc_loss: String,
    pub wall_damage: String,
    pub input_tension: String,
    pub chip_ratio: String,
    pub otg_ratio: String,
    pub scaling: String,
    pub invincibility: String,
    pub cancel: String,
    pub caption: String,
    pub notes: String,
    pub image: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CharMoves {
    pub id: usize,
    pub input: String,
    pub name: String,
    pub alias: String,
    pub move_type: String, 
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HitboxLinks {
    pub hitbox: String,
    pub hitbox_caption: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MoveAliases {
    pub input: String,
    pub aliases: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Nicknames {
    pub character: String,
    pub nicknames: Vec<String>,
}


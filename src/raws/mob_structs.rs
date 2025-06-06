use serde::{Deserialize};
use super::Renderable;
use crate::components::AttackEffect;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct Mob {
    pub name : String,
    pub renderable : Option<Renderable>,
    pub blocks_tile : bool,
    pub vision_range : i32,
    pub movement : String,
    pub quips : Option<Vec<String>>,
    pub health : String,
    pub attributes : MobAttributes,
    pub skills : Option<HashMap<String, i32>>,
    pub mana : Option<i32>,
    pub equipped : Option<Vec<String>>,
    pub natural : Option<MobNatural>,
    pub loot_table : Option<String>,
    pub light : Option<MobLight>,
    pub faction : Option<String>,
    pub gold : Option<String>,
    pub vendor : Option<Vec<String>>,
    pub abilities : Option<Vec<MobAbility>>,
    pub on_death : Option<Vec<MobAbility>>
}

#[derive(Deserialize, Debug)]
pub struct MobAttributes {
    pub strength: Option<i32>,
    pub energy: Option<i32>, // How many action points the creature recovers per per turn
    // pub attack_action_mult: Option<f32>, // The creature's multiplier to the cost to perform an attack action
    // pub move_action_mult: Option<f32>, // The creature's multiplier to the cost to perform a move action
}


#[derive(Deserialize, Debug)]
pub struct MobNatural {
    pub armor_class : Option<i32>,
    pub attacks: Option<Vec<NaturalAttack>>
}

#[derive(Deserialize, Debug)]
pub struct NaturalAttack {
    pub name : String,
    pub hit_bonus : i32,
    pub damage : String,
    pub on_hit: Option<AttackEffect>
}

#[derive(Deserialize, Debug)]
pub struct MobLight {
    pub range : i32,
    pub color : String
}

#[derive(Deserialize, Debug)]
pub struct MobAbility {
    pub spell : String,
    pub chance : f32,
    pub range : f32,
    pub min_range : f32
}

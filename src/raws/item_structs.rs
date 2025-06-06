use serde::{Deserialize};
use std::collections::HashMap;
use crate::components::EffectValues;

#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub name : String,
    pub renderable : Option<Renderable>,
    pub consumable : Option<Consumable>,
    pub weapon : Option<Weapon>,
    pub wearable : Option<Wearable>,
    pub weight_lbs : Option<f32>,
    pub base_value : Option<f32>,
    pub vendor_category : Option<String>,
    pub magic : Option<MagicItem>,
    pub attributes : Option<ItemAttributeBonus>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Renderable {
    pub glyph: String,
    pub fg : String,
    pub bg : Option<String>,
    pub order: i32,
    pub x_size : Option<i32>,
    pub y_size : Option<i32>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Consumable {
    pub effects : HashMap<String, EffectValues>,
    pub charges : Option<i32>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Weapon {
    pub range: String,
    pub base_damage: String,
    pub hit_bonus: i32,
    pub properties : Option<Vec<String>>,
    pub proc_chance : Option<f32>,
    pub proc_target : Option<String>,
    pub proc_effects : Option<HashMap<String, EffectValues>>
}

#[derive(Deserialize, Debug, Clone)]
pub struct Wearable {
    pub armor_class: f32,
    pub slot : String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct MagicItem {
    pub class: String,
    pub naming: String,
    pub cursed: Option<bool>
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemAttributeBonus {
    pub strength : Option<i32>,
    pub constitution : Option<i32>,
    pub dexterity : Option<i32>,
    pub intelligence : Option<i32>
}

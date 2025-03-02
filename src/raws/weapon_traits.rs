use serde::{Deserialize};
use std::collections::HashMap;
use crate::components::EffectValues;

#[derive(Deserialize, Debug)]
pub struct WeaponTrait {
    pub name : String,
    pub effects : HashMap<String, EffectValues>
}

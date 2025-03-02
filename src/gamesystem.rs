use super::{Skill, Skills};

pub fn attr_bonus(value: i32) -> i32 {
    (value-10)/2 // See: https://roll20.net/compendium/dnd5e/Ability%20Scores#content
}

pub fn skill_bonus(skill : Skill, skills: &Skills) -> i32 {
    if skills.skills.contains_key(&skill) {
        skills.skills[&skill]
    } else {
        -4
    }
}

pub fn saving_throw(bonus: i32) -> i32 {
    let nat_roll = crate::rng::roll_dice(1, 20);
    return nat_roll + bonus;
}

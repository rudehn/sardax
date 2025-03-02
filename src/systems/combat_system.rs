use specs::prelude::*;
use crate::{Skills, WantsToMelee, Name, Pools, Equipped, Weapon, EquipmentSlot,
    Wearable, NaturalAttackDefense, AttackEffect, effects::*, WantsToShoot,Initiative, Position, Map, Attributes};
use super::ai::apply_attack_action_cost;
use rltk::{to_cp437, RGB, Point};


pub struct MeleeCombatSystem {}

impl<'a> System<'a> for MeleeCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToMelee>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Attributes>,
                        ReadStorage<'a, Skills>,
                        ReadStorage<'a, Pools>,
                        ReadStorage<'a, Equipped>,
                        ReadStorage<'a, Weapon>,
                        ReadStorage<'a, Wearable>,
                        ReadStorage<'a, NaturalAttackDefense>,
                        WriteStorage<'a, Initiative>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut wants_melee, names, attributes, skills, pools, equipped_items, weapon, wearables, natural, mut initiatives) = data;
        
        for (entity, wants_melee, name, attacker_attributes, attacker_pools, attacker_initiative) in (&entities, &wants_melee, &names, &attributes, &pools, &mut initiatives).join() {
            // Are the attacker and defender alive? Only attack if they are
            let target_pools = pools.get(wants_melee.target).unwrap();
            let target_attributes = attributes.get(wants_melee.target).unwrap();
            if attacker_pools.hit_points.current > 0 && target_pools.hit_points.current > 0 {
                let target_name = names.get(wants_melee.target).unwrap();
                // All attacks, including multi attack, take the same amount of action cost
                apply_attack_action_cost(attacker_initiative);

                // For melee combat, we have several scenarios to cover
                // - The entity has 1 or more natual attacks (IE bite + claw + claw) and we want to roll all attacks
                // - The entity is unarmed - use the default 1d4 unarmed attack
                // - The entity has a weapon - Use the weapon attack + any modifiers
                // - The entity has natural attacks + a weapon - NOT IMPLEMENTED (yet?)

                
                // Define the basic unarmed attack - overridden by wielding check below if a weapon is equipped
                // or overwritten by natural attacks
                let mut weapon_info = Weapon{
                    range: None,
                    hit_bonus : 0,
                    damage_n_dice : 1,
                    damage_die_type : 4,
                    damage_bonus : 0,
                    proc_chance : None,
                    proc_target : None
                };

                let mut use_nat_attack = false;

                // First, check for natural attacks, we'll need to roll for each
                if let Some(nat) = natural.get(entity) {
                    for attack in &nat.attacks {
                        use_nat_attack = true;
                        weapon_info.hit_bonus = attack.hit_bonus;
                        weapon_info.damage_n_dice = attack.damage_n_dice;
                        weapon_info.damage_die_type = attack.damage_die_type;
                        weapon_info.damage_bonus = attack.damage_bonus;

                        let natural_roll = crate::rng::roll_dice(1, 100);
                       
                        let hit_chance = natural_roll +  weapon_info.hit_bonus;
                        let evade_chance = target_attributes.dodge;
                        if hit_chance < 100 - evade_chance {
                            // Target hit!
                            let base_damage = crate::rng::roll_dice(weapon_info.damage_n_dice, weapon_info.damage_die_type);
                            let damage = i32::max(0, base_damage + weapon_info.damage_bonus);
                            // println!("Damage: {} + {}weapon = {}",
                            //     base_damage, weapon_damage_bonus, damage
                            // );
                            do_attack_hit(&entity, &wants_melee.target, &name, &target_name, damage, &attack.name);
                            // Trigger any proc effects from natural attacks
                            if let Some(effects) = &attack.effect{
                                trigger_proc_effects_nat_attack(&entity, &wants_melee.target, &effects);
                            }
                        } else {
                            // Miss
                            log_miss(&name, &target_name, &attack.name);
                            add_attack_miss_particle(&wants_melee.target);
                        }
                    }
                    
                } 
                // No natural attack, attack with unarmed or weapon
                if !use_nat_attack {
                    // Get equipped weapon stats
                    let mut weapon_entity : Option<Entity> = None;
                    for (weaponentity,wielded,melee) in (&entities, &equipped_items, &weapon).join() {
                        if wielded.owner == entity && wielded.slot == EquipmentSlot::Melee {
                            weapon_info = melee.clone();
                            weapon_entity = Some(weaponentity);
                        }
                    }
                    let natural_roll = crate::rng::roll_dice(1, 100);
                    
                    let hit_chance = natural_roll + weapon_info.hit_bonus;
                    let evade_chance = target_attributes.dodge;
                    if hit_chance < 100 - evade_chance {
                        // Target hit!
                        let base_damage = crate::rng::roll_dice(weapon_info.damage_n_dice, weapon_info.damage_die_type);
                        let damage = i32::max(0, base_damage + weapon_info.damage_bonus);
                        // println!("Damage: {} + {}weapon = {}",
                        //     base_damage, weapon_damage_bonus, damage
                        // );
                        do_attack_hit(&entity, &wants_melee.target, &name, &target_name, damage, "attacks");
                        // Proc effects
                        trigger_proc_effects(&entity, &wants_melee.target, &weapon_info, weapon_entity);
                    } else {
                        // Miss
                        log_miss(&name, &target_name, "attacks");
                        add_attack_miss_particle(&wants_melee.target);
                    }
                }
            }
        }
        wants_melee.clear();
    }
}


pub struct RangedCombatSystem {}

impl<'a> System<'a> for RangedCombatSystem {
    #[allow(clippy::type_complexity)]
    type SystemData = ( Entities<'a>,
                        WriteStorage<'a, WantsToShoot>,
                        ReadStorage<'a, Name>,
                        ReadStorage<'a, Attributes>,
                        ReadStorage<'a, Skills>,
                        ReadStorage<'a, Pools>,
                        ReadStorage<'a, Equipped>,
                        ReadStorage<'a, Weapon>,
                        ReadStorage<'a, Wearable>,
                        ReadStorage<'a, Position>,
                        ReadExpect<'a, Map>,
                        WriteStorage<'a, Initiative>
                      );

    fn run(&mut self, data : Self::SystemData) {
        let (entities, mut wants_shoot, names, attributes, skills, pools, equipped_items, weapon, wearables,
            positions, map, mut initiatives) = data;

        for (entity, wants_shoot, name, attacker_attributes, attacker_pools, attacker_initiative) in (&entities, &wants_shoot, &names, &attributes, &pools, &mut initiatives).join() {
            // Are the attacker and defender alive? Only attack if they are
            let target_pools = pools.get(wants_shoot.target).unwrap();
            let target_attributes = attributes.get(wants_shoot.target).unwrap();
            if attacker_pools.hit_points.current > 0 && target_pools.hit_points.current > 0 {
                let target_name = names.get(wants_shoot.target).unwrap();
                // All attacks, including multi attack, take the same amount of action cost
                apply_attack_action_cost(attacker_initiative);

                // Fire projectile effect
                let apos = positions.get(entity).unwrap();
                let dpos = positions.get(wants_shoot.target).unwrap();
                add_effect(
                    None, 
                    EffectType::ParticleProjectile{ 
                        glyph: to_cp437('*'),
                        fg : RGB::named(rltk::CYAN), 
                        bg : RGB::named(rltk::BLACK), 
                        lifespan : 300.0, 
                        speed: 50.0, 
                        path: rltk::line2d(
                            rltk::LineAlg::Bresenham, 
                            Point::new(apos.x, apos.y), 
                            Point::new(dpos.x, dpos.y)
                        )
                     }, 
                    Targets::Tile{tile_idx : map.xy_idx(apos.x, apos.y) as i32}
                );

                // Define the basic unarmed attack - overridden by wielding check below if a weapon is equipped
                let mut weapon_info = Weapon{
                    range: None,
                    hit_bonus : 0,
                    damage_n_dice : 1,
                    damage_die_type : 4,
                    damage_bonus : 0,
                    proc_chance : None,
                    proc_target : None
                };

                let mut weapon_entity : Option<Entity> = None;
                for (weaponentity,wielded,melee) in (&entities, &equipped_items, &weapon).join() {
                    if wielded.owner == entity && wielded.slot == EquipmentSlot::Melee {
                        weapon_info = melee.clone();
                        weapon_entity = Some(weaponentity);
                    }
                }

                let natural_roll = crate::rng::roll_dice(1, 100);
                let hit_chance = natural_roll + weapon_info.hit_bonus;
       
                // let mut armor_item_bonus_f = 0.0;
                // for (wielded,armor) in (&equipped_items, &wearables).join() {
                //     if wielded.owner == wants_shoot.target {
                //         armor_item_bonus_f += armor.armor_class;
                //     }
                // }
                // let base_armor_class = match natural.get(wants_shoot.target) {
                //     None => 10,
                //     Some(nat) => nat.armor_class.unwrap_or(10)
                // };
                // let armor_dexterity_bonus = target_attributes.dexterity.bonus;
                // let mut armor_skill_bonus = 0;
                // if let Some(target_skills) = skills.get(wants_shoot.target){
                //     armor_skill_bonus = skill_bonus(Skill::Defense, &*target_skills);
                // }
                // let armor_item_bonus = armor_item_bonus_f as i32;
                // let armor_class = base_armor_class + armor_dexterity_bonus + armor_skill_bonus
                //     + armor_item_bonus;

                //println!("Armor class: {}", armor_class);
                // Apply a 10% hit penalty to ranged attacks
                let evade_chance = 10 + target_attributes.dodge;
                if hit_chance < 100 - evade_chance {
                    // Target hit!
                    let base_damage = crate::rng::roll_dice(weapon_info.damage_n_dice, weapon_info.damage_die_type);
                    let damage = i32::max(0, base_damage + weapon_info.damage_bonus);
                    do_attack_hit(&entity, &wants_shoot.target, &name, &target_name, damage, "shoots");
                    // Proc effects
                    trigger_proc_effects(&entity, &wants_shoot.target, &weapon_info, weapon_entity);
                }
                else {
                    log_miss(&name, &target_name, "shoots");
                    add_attack_miss_particle(&wants_shoot.target);
                }
            }
        }

        wants_shoot.clear();
    }
}

fn trigger_proc_effects(attacker: &Entity, defender: &Entity, weapon_info: &Weapon, weapon_entity: Option<Entity>){
    // Proc effects
    if let Some(chance) = &weapon_info.proc_chance {
        let roll = crate::rng::roll_dice(1, 100);
        if roll <= (chance * 100.0) as i32 {
            let effect_target = if weapon_info.proc_target.clone().unwrap_or("Target".to_string()) == "Self" {
                Targets::Single{ target: *attacker }
            } else {
                Targets::Single { target : *defender }
            };
            add_effect(
                Some(*attacker),
                EffectType::ItemUse{ item: weapon_entity.unwrap() },
                effect_target
            )
        }
    }
}

fn trigger_proc_effects_nat_attack(attacker: &Entity, defender: &Entity, nat_attack: &AttackEffect){
    // Proc effects
    if let Some(chance) = &nat_attack.proc_chance {
        let roll = crate::rng::roll_dice(1, 100);
        if roll <= (chance * 100.0) as i32 {
            let effect_target = if nat_attack.proc_target.clone().unwrap_or("Target".to_string()) == "Self" {
                Targets::Single{ target: *attacker }
            } else {
                Targets::Single { target : *defender }
            };
            add_effect(
                Some(*attacker),
                EffectType::NatAttack{ effects: nat_attack.proc_effects.clone()},
                effect_target
            )
        }
    }
}

fn do_attack_hit(
    attacker_entity: &Entity, defender_entity: &Entity,
    attacker_name: &Name, defender_name: &Name, damage: i32, damage_verb: &str
) {
    add_effect(
        Some(*attacker_entity),
        EffectType::Damage{ amount: damage },
        Targets::Single{ target: *defender_entity }
    );
    crate::gamelog::Logger::new()
        .npc_name(&attacker_name.name)
        .append(damage_verb)
        .npc_name(&defender_name.name)
        .append("for")
        .damage(damage)
        .append("hp.")
        .log();
}

fn log_miss(attacker_name: &Name, defender_name: &Name, damage_verb: &str) {
    // Log the attack missed
    crate::gamelog::Logger::new()
        .npc_name(&attacker_name.name)
        .append(damage_verb)
        .npc_name(&defender_name.name)
        .color(rltk::WHITE)
        .append("but misses.")
        .log();
}

fn add_attack_miss_particle(defender_entity: &Entity){
    // Show a !! indication on the target that the attack missed
    add_effect(
        None,
        EffectType::Particle{ glyph: rltk::to_cp437('‼'), fg: rltk::RGB::named(rltk::CYAN), bg : rltk::RGB::named(rltk::BLACK), lifespan: 200.0 },
        Targets::Single{ target: *defender_entity }
        );
}
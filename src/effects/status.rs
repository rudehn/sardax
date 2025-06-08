// Status Effects
use specs::prelude::{Builder, Entity, Join, LendJoin, World, WorldExt};
use crate::components::{Duration, Burn, Name, SerializeMe, Slow, Haste, Stun, StatusEffect as StatusEffectComponent};

use specs::saveload::{MarkedBuilder, SimpleMarker};

#[derive(Debug)]
pub enum StatusEffect { 
    // Berserk,
    // Bleed, lose x% hp per turn
    // Blind, // vision reduced/removed
    // Confuse, // attack both ally and enemy
    // Curse, // Drop in some stat
    // Fear, // cause enemy to run away from source of fear
    // Poison, // lose hp per turn?
    // Silence, // Target unable to use magic spells
    Haste, // increase energy gain by 100%
    Slow, // reduce energy gain by 50%
    Stun, // prevent all actions for 1 turn
    Burn // lose hp per turn?
}

pub fn apply_status_effect(ecs: &mut World, effect: &StatusEffect, target: Entity) {
    match effect {
        StatusEffect::Burn => burn(ecs, target),
        StatusEffect::Haste => haste(ecs, target),
        StatusEffect::Slow => slow(ecs, target),
        StatusEffect::Stun => stun(ecs, target),
    }
}

pub fn tile_status_effect_hits_entities(effect: &StatusEffect) -> bool {
    match effect {
        StatusEffect::Burn => true,
        StatusEffect::Haste => true,
        StatusEffect::Slow => true,
        StatusEffect::Stun => true,
    }
}


fn burn(ecs: &mut World, target: Entity) {
    let burn_duration = 4;
    
    // If the target is currently burning, just reset the duration counter
    let mut found_status = false;

    {
        let mut durations = ecs.write_storage::<Duration>();
        let status_effect = ecs.read_storage::<StatusEffectComponent>();
        let burning = ecs.read_storage::<Burn>();
        for (effect, _burn, duration) in (&status_effect, &burning, &mut durations).join() {
            if effect.target == target {
                found_status = true;
                duration.turns = burn_duration;
                break;
            }
        }
    }
    if !found_status {
        {
            ecs.create_entity()
                .with(StatusEffectComponent{ target })
                .with(Burn{})
                .with(Duration{ turns : burn_duration, total_turns: burn_duration})
                .with(Name{ name : "Burning".to_string() })
                .marked::<SimpleMarker<SerializeMe>>()
                .build();
        }
    }
}

fn haste(ecs: &mut World, target: Entity) {
    // Haste will cancel out any slow status effect on the entity
    let haste_duration = 5;

    let mut found_status = false;
    let mut entities_to_delete : Vec<Entity> = Vec::new();
    {
        let mut durations = ecs.write_storage::<Duration>();
        let status_effect = ecs.read_storage::<StatusEffectComponent>();
        let slowed = ecs.read_storage::<Slow>();
        let hasted = ecs.read_storage::<Haste>();
        let entities = ecs.entities();

        
        for (entity, effect, slow, haste, duration) in (&entities, &status_effect, (&slowed).maybe(), (&hasted).maybe(), &mut durations).join() {
            
            // If the target is currently slowed, cancel out the status effect
            if let Some(_slow) = slow {
                entities_to_delete.push(entity);
                found_status = true;
                break;
            }
            // If the target is currently hasted, just reset the duration counter
            if let Some(_haste) = haste {
                if effect.target == target {
                    found_status = true;
                    duration.turns = haste_duration;
                    break;
                }
            }
        }
    }
    
    for entity in entities_to_delete {
        ecs.delete_entity(entity).expect("Unable to delete");
    }

    if !found_status{
        ecs.create_entity()
            .with(StatusEffectComponent{ target })
            .with(Haste{})
            .with(Duration{ turns : haste_duration, total_turns: haste_duration})
            .with(Name{ name : "Hasted".to_string() })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}


fn slow(ecs: &mut World, target: Entity) {
    // Slow will cancel out any haste status effect
    let slow_duration = 5;
    let mut found_status = false;
    let mut entities_to_delete : Vec<Entity> = Vec::new();
    {
        let mut durations = ecs.write_storage::<Duration>();
        let status_effect = ecs.read_storage::<StatusEffectComponent>();
        let slowed = ecs.read_storage::<Slow>();
        let hasted = ecs.read_storage::<Haste>();
        let entities = ecs.entities();

        for (entity, effect, slow, haste, duration) in (&entities, &status_effect, (&slowed).maybe(), (&hasted).maybe(), &mut durations).join() {
            
            // If the target is currently hasted, cancel out the status effect
            if let Some(_haste) = haste {
                entities_to_delete.push(entity);
                found_status = true;
                break;
            }
            // If the target is currently slowed, just reset the duration counter
            if let Some(_slow) = slow {
                if effect.target == target {
                    found_status = true;
                    duration.turns = slow_duration;
                    break;
                }
            }
        }
    }
    
    for entity in entities_to_delete {
        ecs.delete_entity(entity).expect("Unable to delete");
    }

    if !found_status{
        ecs.create_entity()
            .with(StatusEffectComponent{ target })
            .with(Slow{})
            .with(Duration{ turns : slow_duration, total_turns: slow_duration})
            .with(Name{ name : "Slowed".to_string() })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}


fn stun(ecs: &mut World, target: Entity) {
    ecs.create_entity()
        .with(StatusEffectComponent{ target })
        .with(Stun{})
        .with(Name{ name : "Stunned".to_string() })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();
}
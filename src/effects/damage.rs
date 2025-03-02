use specs::prelude::*;
use super::*;
use crate::components::{Pools, Player, Burning, Paralysis, SerializeMe, Duration, StatusEffect, 
    Name, Slow, Haste};
use crate::map::Map;
use specs::saveload::{MarkedBuilder, SimpleMarker};

pub fn inflict_damage(ecs: &mut World, damage: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    let player_entity = ecs.fetch::<Entity>();
    if let Some(pool) = pools.get_mut(target) {
        if !pool.god_mode {
            if let Some(creator) = damage.creator {
                if creator == target { 
                    return; 
                }
            }
            if let EffectType::Damage{amount} = damage.effect_type {
                pool.hit_points.current -= amount;
                add_effect(None, EffectType::Bloodstain, Targets::Single{target});
                add_effect(None, 
                    EffectType::Particle{ 
                        glyph: rltk::to_cp437('‼'),
                        fg : rltk::RGB::named(rltk::ORANGE),
                        bg : rltk::RGB::named(rltk::BLACK),
                        lifespan: 200.0
                    }, 
                    Targets::Single{target}
                );
                if target == *player_entity {
                    crate::gamelog::record_event("Damage Taken", amount);
                }
                if let Some(creator) = damage.creator {
                    if creator == *player_entity {
                        crate::gamelog::record_event("Damage Inflicted", amount);
                    }
                }

                if pool.hit_points.current < 1 {
                    add_effect(damage.creator, EffectType::EntityDeath, Targets::Single{target});
                }
            }
        }
    }
}

pub fn bloodstain(ecs: &mut World, tile_idx : i32) {
    let mut map = ecs.fetch_mut::<Map>();
    map.bloodstains.insert(tile_idx as usize);
}

pub fn death(ecs: &mut World, effect: &EffectSpawner, target : Entity) {
    let mut gold_gain = 0.0f32;

    let mut pools = ecs.write_storage::<Pools>();

    if let Some(pos) = entity_position(ecs, target) {
        crate::spatial::remove_entity(target, pos as usize);
    }

    if let Some(source) = effect.creator {
        if ecs.read_storage::<Player>().get(source).is_some() {
            if let Some(stats) = pools.get(target) {
                gold_gain += stats.gold;
            }

            if gold_gain != 0.0 {
                let mut player_stats = pools.get_mut(source).unwrap();
                player_stats.gold += gold_gain;
            }
        }
    }
}

pub fn heal_damage(ecs: &mut World, heal: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    if let Some(pool) = pools.get_mut(target) {
        if let EffectType::Healing{amount} = heal.effect_type {
            pool.hit_points.current = i32::min(pool.hit_points.max, pool.hit_points.current + amount);
            add_effect(None, 
                EffectType::Particle{ 
                    glyph: rltk::to_cp437('‼'),
                    fg : rltk::RGB::named(rltk::GREEN),
                    bg : rltk::RGB::named(rltk::BLACK),
                    lifespan: 200.0
                }, 
                Targets::Single{target}
            );
        }
    }
}

pub fn restore_mana(ecs: &mut World, mana: &EffectSpawner, target: Entity) {
    let mut pools = ecs.write_storage::<Pools>();
    if let Some(pool) = pools.get_mut(target) {
        if let EffectType::Mana{amount} = mana.effect_type {
            pool.mana.current = i32::min(pool.mana.max, pool.mana.current + amount);
            add_effect(None, 
                EffectType::Particle{ 
                    glyph: rltk::to_cp437('‼'),
                    fg : rltk::RGB::named(rltk::BLUE),
                    bg : rltk::RGB::named(rltk::BLACK),
                    lifespan: 200.0
                }, 
                Targets::Single{target}
            );
        }
    }
}

pub fn add_paralysis(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Paralysis{turns} = &effect.effect_type {
        ecs.create_entity()
            .with(StatusEffect{ target })
            .with(Paralysis{})
            .with(Duration{turns : *turns, total_turns: *turns})
            .with(Name{ name : "Paralysis".to_string() })
            .marked::<SimpleMarker<SerializeMe>>()
            .build();
    }
}


pub fn add_burning(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Burning{turns} = &effect.effect_type {
        // If the target is currently burning, just reset the duration counter
        let mut found_status = false;

        {
            let mut durations = ecs.write_storage::<Duration>();
            let status_effect = ecs.read_storage::<StatusEffect>();
            let burning = ecs.read_storage::<Burning>();
            for (effect, _burn, duration) in (&status_effect, &burning, &mut durations).join() {
                if effect.target == target {
                    found_status = true;
                    duration.turns = *turns;
                    break;
                }
            }
        }
        if !found_status {
            {
                ecs.create_entity()
                    .with(StatusEffect{ target })
                    .with(Burning{})
                    .with(Duration{ turns : *turns, total_turns: *turns})
                    .with(Name{ name : "Burning".to_string() })
                    .marked::<SimpleMarker<SerializeMe>>()
                    .build();
            }
        }
    }
}

pub fn slow(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    if let EffectType::Slow = &effect.effect_type {
        let slow_duration = 5;

        let mut found_status = false;
        let mut entities_to_delete : Vec<Entity> = Vec::new();
        {
            let mut durations = ecs.write_storage::<Duration>();
            let status_effect = ecs.read_storage::<StatusEffect>();
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
                .with(StatusEffect{ target })
                .with(Slow{})
                .with(Duration{ turns : slow_duration, total_turns: slow_duration})
                .with(Name{ name : "Slowed".to_string() })
                .marked::<SimpleMarker<SerializeMe>>()
                .build();
        }
    }
}

pub fn haste(ecs: &mut World, effect: &EffectSpawner, target: Entity) {
    // Haste will cancel out any slow status effect on the entity
    if let EffectType::Haste = &effect.effect_type {
        let haste_duration = 5;

        let mut found_status = false;
        let mut entities_to_delete : Vec<Entity> = Vec::new();
        {
            let mut durations = ecs.write_storage::<Duration>();
            let status_effect = ecs.read_storage::<StatusEffect>();
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
                .with(StatusEffect{ target })
                .with(Haste{})
                .with(Duration{ turns : haste_duration, total_turns: haste_duration})
                .with(Name{ name : "Hasted".to_string() })
                .marked::<SimpleMarker<SerializeMe>>()
                .build();
        }
    }
}

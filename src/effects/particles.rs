use specs::prelude::*;
use super::*;
use crate::systems::particle_system::ParticleBuilder;
use crate::raws::{RAWS, spawn_named_entity, SpawnType};
use crate::map::{Map, tile_burnable};
use crate::components::{InflictsBurning, Name, ParticleAnimation, ParticleLifetime, Renderable, Position};
use crate::TileType;

pub fn particle_to_tile(ecs: &mut World, tile_idx : i32, effect: &EffectSpawner) {
    if let EffectType::Particle{ glyph, fg, bg, lifespan } = effect.effect_type {
        let map = ecs.fetch::<Map>();
        let mut particle_builder = ecs.fetch_mut::<ParticleBuilder>();
        particle_builder.request(
            tile_idx % map.width, 
            tile_idx / map.width, 
            fg, 
            bg, 
            glyph, 
            lifespan
        );
    }
}

pub fn projectile(ecs: &mut World, tile_idx : i32, effect: &EffectSpawner) {
    if let EffectType::ParticleProjectile{ glyph, fg, bg, 
        lifespan: _, speed, path } = &effect.effect_type 
    {
        let map = ecs.fetch::<Map>();
        let x = tile_idx % map.width;
        let y = tile_idx / map.width;
        std::mem::drop(map);
        ecs.create_entity()
            .with(Position{ x, y })
            .with(Renderable{ fg: *fg, bg: Some(*bg), glyph: *glyph, render_order: 0 })
            .with(ParticleLifetime{
                lifetime_ms: path.len() as f32 * speed,
                animation: Some(ParticleAnimation{
                    step_time: *speed,
                    path: path.to_vec(),
                    current_step: 0,
                    timer: 0.0
                })
            })
            .build();
    }
}

pub fn create_fire(ecs: &mut World, tile_idx :i32, effect: &EffectSpawner) {
    if let EffectType::Burning{..} = &effect.effect_type 
    {
        // First check that the specified tile isn't already ablaze
        let mut found_fire = false;
        
        let map = ecs.fetch::<Map>();
        let x = tile_idx % map.width;
        let y = tile_idx / map.width;

        {
            let names = ecs.read_storage::<Name>();
            let burnings = ecs.read_storage::<InflictsBurning>();
            let positions = ecs.read_storage::<Position>();
            for (name, _burn, pos) in (&names, &burnings, &positions).join() {
                if name.name == "Fire"  && pos.x == x && pos.y == y {
                    found_fire = true;
                    break;
                }
            }
        }
        if !found_fire {
            // Now check if this tile is burnable & we can put a fire on it
            // if tile_burnable(map.tiles[map.xy_idx(x, y)]){
            if tile_burnable(map.tiles[tile_idx as usize]){
                std::mem::drop(map);
                spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Fire", SpawnType::AtPosition { x, y });
            }
        }
    }
}

pub fn dig_out_tunnel(ecs: &mut World, tile_idx :i32, effect: &EffectSpawner) {
    if let EffectType::CreatesTunnel = &effect.effect_type 
    {
        let mut map = ecs.fetch_mut::<Map>();
        let idx = tile_idx as usize;
        if map.tiles[idx] == TileType::Wall {
            map.tiles[idx] = TileType::Floor;
        }
    }
}
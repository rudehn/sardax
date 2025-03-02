use specs::prelude::{Entity, World, WorldExt};

use crate::components::{Faction, Viewshed};
use crate::Map;

pub fn get_characters_in_vision(ecs : &World, entity: &Entity) -> Vec<Entity> {
    let mut entities_in_view : Vec<Entity> = Vec::new();
    
    let viewsheds = ecs.read_storage::<Viewshed>();
    let factions = ecs.read_storage::<Faction>();
    let map = ecs.fetch::<Map>();

    // Check the entity has a vision
    if let Some(vs) = viewsheds.get(*entity) {
        for tile_point in vs.visible_tiles.iter() {
            let tile_idx = map.xy_idx(tile_point.x, tile_point.y);
            // let distance_to_target = rltk::DistanceAlg::Pythagoras.distance2d(*tile_point, rltk::Point::new(player_pos.x, player_pos.y));
            // if distance_to_target < range as f32 {
            crate::spatial::for_each_tile_content(tile_idx, |possible_target| {
                if possible_target != *entity && factions.get(possible_target).is_some() {
                    // entities_in_view.push((distance_to_target, possible_target));
                    entities_in_view.push(possible_target);
                }
            });
            // }
        }
    }
    // TODO - sort by range
    entities_in_view
}
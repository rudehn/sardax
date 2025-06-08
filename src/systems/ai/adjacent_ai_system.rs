use specs::prelude::*;
use rltk::Rect;
use crate::{MyTurn, Faction, Position, Map, raws::Reaction, WantsToAttack, AttackType, TileSize};

pub struct AdjacentAI {}

impl<'a> System<'a> for AdjacentAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteStorage<'a, MyTurn>,
        ReadStorage<'a, Faction>,
        ReadStorage<'a, Position>,
        ReadExpect<'a, Map>,
        WriteStorage<'a, WantsToAttack>,
        Entities<'a>,
        ReadExpect<'a, Entity>,
        ReadStorage<'a, TileSize>
    );

    fn run(&mut self, data : Self::SystemData) {
        let (mut turns, factions, positions, map, mut want_attack, entities, player, sizes) = data;
        let mut turn_done : Vec<Entity> = Vec::new();
        for (entity, _turn, my_faction, pos) in (&entities, &turns, &factions, &positions).join() {
            if entity != *player {
                let mut reactions : Vec<(Entity, Reaction)> = Vec::new();
                let idx = map.xy_idx(pos.x, pos.y);
                let w = map.width;
                let h = map.height;

                if let Some(size) = sizes.get(entity) {
                    let mob_rect = Rect::with_size(pos.x, pos.y, size.x, size.y).point_set();
                    let parent_rect = Rect::with_size(pos.x -1, pos.y -1, size.x+2, size.y + 2);
                    parent_rect.point_set().iter().filter(|t| !mob_rect.contains(t)).for_each(|t| {
                        if t.x > 0 && t.x < w-1 && t.y > 0 && t.y < h-1 {
                            let target_idx = map.xy_idx(t.x, t.y);
                            evaluate(target_idx, &map, &factions, &my_faction.name, &mut reactions);
                        }
                    });
                } else {

                    // Add possible reactions to adjacents for each direction
                    if pos.x > 0 { evaluate(idx-1, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.x < w-1 { evaluate(idx+1, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y > 0 { evaluate(idx-w as usize, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y < h-1 { evaluate(idx+w as usize, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y > 0 && pos.x > 0 { evaluate((idx-w as usize)-1, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y > 0 && pos.x < w-1 { evaluate((idx-w as usize)+1, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y < h-1 && pos.x > 0 { evaluate((idx+w as usize)-1, &map, &factions, &my_faction.name, &mut reactions); }
                    if pos.y < h-1 && pos.x < w-1 { evaluate((idx+w as usize)+1, &map, &factions, &my_faction.name, &mut reactions); }

                }

                let mut done = false;
                for reaction in reactions.iter() {
                    if let Reaction::Attack = reaction.1 {
                        want_attack.insert(entity, WantsToAttack{ target: reaction.0, attack_type: AttackType::Melee}).expect("Error inserting melee");
                        done = true;
                    }
                }

                if done { turn_done.push(entity); }
            }
        }

        // Remove turn marker for those that are done
        for done in turn_done.iter() {
            turns.remove(*done);
        }
    }
}

fn evaluate(idx : usize, map : &Map, factions : &ReadStorage<Faction>, my_faction : &str, reactions : &mut Vec<(Entity, Reaction)>) {
    crate::spatial::for_each_tile_content(idx, |other_entity| {
        if let Some(faction) = factions.get(other_entity) {
            reactions.push((
                other_entity,
                crate::raws::faction_reaction(my_faction, &faction.name, &crate::raws::RAWS.lock().unwrap())
            ));
        }
    });
}

use rltk::{ RGB, Rect };
use specs::prelude::*;
use super::{Pools, Pool, Player, Renderable, Name, Position, Viewshed,
    SerializeMe, random_table::MasterTable, HungerClock, HungerState, Map, TileType, raws::*,
    Attributes, Attribute, Skills, Skill, LightSource, Initiative, Faction,
    OtherLevelPosition, MasterDungeonMap, EntryTrigger, TeleportTo, SingleActivation, KnownSpells,
    EquipmentChanged, DEFAULT_ENERGY_GAIN};
use crate::combat::stats::attr_bonus;
use specs::saveload::{MarkedBuilder, SimpleMarker};
use std::collections::HashMap;
use crate::raws::SpawnTableType;

/// Spawns the player and returns his/her entity object.
pub fn player(ecs : &mut World, player_x : i32, player_y : i32) -> Entity {
    spawn_all_spells(ecs);

    let mut skills = Skills{ skills: HashMap::new() };
    skills.skills.insert(Skill::Melee, 1);
    skills.skills.insert(Skill::Defense, 1);
    skills.skills.insert(Skill::Magic, 1);

    let player = ecs
        .create_entity()
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: None,
            render_order: 0
        })
        .with(Player{})
        .with(Viewshed{ visible_tiles : Vec::new(), range: 18, dirty: true })
        .with(Name{name: "You".to_string() })
        .with(HungerClock{ state: HungerState::WellFed, duration: 200 })
        .with(Attributes{
            strength: Attribute{base: 11, modifiers: 0, bonus: attr_bonus(11)}
        })
        .with(skills)
        .with(Pools{
            hit_points : Pool{
                current: 100,
                max: 100
            },
            mana: Pool{
                current: 10,
                max: 10
            },
            total_weight : 0.0,
            gold : 0.0,
            god_mode : false
        })
        .with(EquipmentChanged{})
        .with(LightSource{ color: rltk::RGB::from_f32(1.0, 1.0, 1.0), range: 12 })
        .with(Initiative{
            energy_gain: DEFAULT_ENERGY_GAIN,
            current: 0})
        .with(Faction{name : "Player".to_string() })
        .with(KnownSpells{ spells : Vec::new() })
        .marked::<SimpleMarker<SerializeMe>>()
        .build();

    // Starting equipment
    // spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Dried Sausage", SpawnType::Carried{by : player} );
    // spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Beer", SpawnType::Carried{by : player});
    spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Shortbow", SpawnType::Carried{by : player});
    // spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Rod of Fireballs2", SpawnType::Carried{by : player});
    // spawn_named_entity(&RAWS.lock().unwrap(), ecs, "Rod of Tunneling", SpawnType::Carried{by : player});

    
    player
}

const MAX_MONSTERS : i32 = 1;

fn room_table(map_depth: i32) -> MasterTable {
    get_spawn_table_for_depth(&RAWS.lock().unwrap(), map_depth)
}

/// Fills a room with stuff!
pub fn spawn_room(map: &Map, room : &Rect, map_depth: i32, spawn_list : &mut Vec<(usize, String)>, spawn_type: SpawnTableType) {
    let mut possible_targets : Vec<usize> = Vec::new();
    { // Borrow scope - to keep access to the map separated
        for y in room.y1 + 1 .. room.y2 {
            for x in room.x1 + 1 .. room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(map, &possible_targets, map_depth, spawn_list, spawn_type);
}

/// Fills a region with stuff!
pub fn spawn_region(_map: &Map, area : &[usize], map_depth: i32, spawn_list : &mut Vec<(usize, String)>, spawn_type: SpawnTableType) {
    let spawn_table = room_table(map_depth);
    let mut spawn_points : HashMap<usize, String> = HashMap::new();
    let mut areas : Vec<usize> = Vec::from(area);

    // Scope to keep the borrow checker happy
    {
        let num_spawns = i32::min(areas.len() as i32, crate::rng::roll_dice(1, MAX_MONSTERS) + (map_depth - 1));
        if num_spawns == 0 { return; }

        for _i in 0 .. num_spawns {
            let array_index = if areas.len() == 1 { 0usize } else { (crate::rng::roll_dice(1, areas.len() as i32)-1) as usize };

            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, spawn_table.roll_by_type(spawn_type));
            areas.remove(array_index);
        }
    }

    // Actually spawn the monsters
    for spawn in spawn_points.iter() {
        spawn_list.push((*spawn.0, spawn.1.to_string()));
    }
}

/// Spawns a named entity (name in tuple.1) at the location in (tuple.0)
pub fn spawn_entity(ecs: &mut World, spawn : &(&usize, &String)) {
    let map = ecs.fetch::<Map>();
    let width = map.width as usize;
    let x = (*spawn.0 % width) as i32;
    let y = (*spawn.0 / width) as i32;
    std::mem::drop(map);

    let spawn_result = spawn_named_entity(&RAWS.lock().unwrap(), ecs, &spawn.1, SpawnType::AtPosition{ x, y});
    if spawn_result.is_some() {
        return;
    }

    if spawn.1 != "None" {
        rltk::console::log(format!("WARNING: We don't know how to spawn [{}]!", spawn.1));
    }
}

pub fn spawn_town_portal(ecs: &mut World) {
    // Get current position & depth
    let map = ecs.fetch::<Map>();
    let player_depth = map.depth;
    let player_pos = ecs.fetch::<rltk::Point>();
    let player_x = player_pos.x;
    let player_y = player_pos.y;
    std::mem::drop(player_pos);
    std::mem::drop(map);

    // Find part of the town for the portal
    let dm = ecs.fetch::<MasterDungeonMap>();
    let town_map = dm.get_map(1).unwrap();
    let mut stairs_idx = 0;
    for (idx, tt) in town_map.tiles.iter().enumerate() {
        if *tt == TileType::DownStairs {
            stairs_idx = idx;
        }
    }
    let portal_x = (stairs_idx as i32 % town_map.width)-2;
    let portal_y = stairs_idx as i32 / town_map.width;

    std::mem::drop(dm);

    // Spawn the portal itself
    ecs.create_entity()
        .with(OtherLevelPosition { x: portal_x, y: portal_y, depth: 1 })
        .with(Renderable {
            glyph: rltk::to_cp437('♥'),
            fg: RGB::named(rltk::CYAN),
            bg: None,
            render_order: 0
        })
        .with(EntryTrigger{})
        .with(TeleportTo{ x: player_x, y: player_y, depth: player_depth, player_only: true })
        .with(SingleActivation{})
        .with(Name{ name : "Town Portal".to_string() })
        .build();
}

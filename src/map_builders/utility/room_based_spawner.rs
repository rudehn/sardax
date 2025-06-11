use crate::map_builders::{MetaMapBuilder, BuilderMap, spawner};
use crate::raws::SpawnTableType;

pub struct RoomBasedMobSpawner {}

impl MetaMapBuilder for RoomBasedMobSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl RoomBasedMobSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedMobSpawner> {
        Box::new(RoomBasedMobSpawner{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            for room in rooms.iter().skip(1) {
                spawner::spawn_room(&build_data.map, room, build_data.map.depth, &mut build_data.spawn_list, SpawnTableType::Mob);
            }
        } else {
            panic!("Room Based Spawning only works after rooms have been created");
        }
    }
}

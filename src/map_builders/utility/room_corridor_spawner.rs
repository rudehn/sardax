use crate::map_builders::{MetaMapBuilder, BuilderMap, spawner};
use crate::raws::SpawnTableType;

pub struct CorridorMobSpawner {}

impl MetaMapBuilder for CorridorMobSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl CorridorMobSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<CorridorMobSpawner> {
        Box::new(CorridorMobSpawner{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if let Some(corridors) = &build_data.corridors {
            for c in corridors.iter() {
                let depth = build_data.map.depth;
                spawner::spawn_region(&build_data.map,
                    &c,
                    depth,
                    &mut build_data.spawn_list,
                    SpawnTableType::Mob
                );
            }
        } else {
            panic!("Corridor Based Spawning only works after corridors have been created");
        }
    }
}

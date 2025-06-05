use crate::map_builders::{MetaMapBuilder, BuilderMap, Position, TileType};
use crate::map;

pub struct DungeonExitSpawner {}

impl MetaMapBuilder for DungeonExitSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl DungeonExitSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<DungeonExitSpawner> {
        Box::new(DungeonExitSpawner{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        let mut found_exit = false;
        for (idx, mut tiletype) in build_data.map.tiles.iter_mut().enumerate() {
            if *tiletype == TileType::DownStairs {
                *tiletype = TileType::DungeonExit;
                found_exit = true;
                build_data.take_snapshot();
                break;
            }
        }
        if !found_exit {
            panic!("Dungeon Exit Spawner needs a down stairs point!");
        }
    }
}
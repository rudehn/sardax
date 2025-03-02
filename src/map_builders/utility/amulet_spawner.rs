use crate::map_builders::{MetaMapBuilder, BuilderMap, TileType};

pub struct AmuletSpawner {}

impl MetaMapBuilder for AmuletSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl AmuletSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<AmuletSpawner> {
        Box::new(AmuletSpawner{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        let starting_pos = build_data.starting_position.as_ref().unwrap().clone();
        let start_idx = build_data.map.xy_idx(
            starting_pos.x,
            starting_pos.y
        );
        build_data.map.populate_blocked();
        let map_starts : Vec<usize> = vec![start_idx];
        let dijkstra_map = rltk::DijkstraMap::new(build_data.map.width as usize, build_data.map.height as usize, &map_starts , &build_data.map, 3000.0);
        let mut amulet_tile = (0, 0.0f32);
        for (i, tile) in build_data.map.tiles.iter_mut().enumerate() {
            if *tile == TileType::Floor {
                let distance_to_start = dijkstra_map.map[i];
                if distance_to_start != std::f32::MAX {
                    // If it is further away than our current exit candidate, move the exit
                    if distance_to_start > amulet_tile.1 {
                        amulet_tile.0 = i;
                        amulet_tile.1 = distance_to_start;
                    }
                }
            }
        }
        
        // Add the amulet
        // TODO - actually create a room with a guardian
        build_data.spawn_list.push((amulet_tile.0, "Amulet of Endulo".to_string()));
        build_data.take_snapshot();
    }
}

use crate::map_builders::{MetaMapBuilder, BuilderMap, TileType, spawner};
use std::collections::HashMap;
use crate::raws::SpawnTableType;

pub struct VoronoiMobSpawning {}

impl MetaMapBuilder for VoronoiMobSpawning {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl VoronoiMobSpawning {
    #[allow(dead_code)]
    pub fn new() -> Box<VoronoiMobSpawning> {
        Box::new(VoronoiMobSpawning{})
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, build_data : &mut BuilderMap) {
        let mut noise_areas : HashMap<i32, Vec<usize>> = HashMap::new();
        let mut noise = rltk::FastNoise::seeded(crate::rng::roll_dice(1, 65536) as u64);
        noise.set_noise_type(rltk::NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(rltk::CellularDistanceFunction::Manhattan);

        for y in 1 .. build_data.map.height-1 {
            for x in 1 .. build_data.map.width-1 {
                let idx = build_data.map.xy_idx(x, y);
                if build_data.map.tiles[idx] == TileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if noise_areas.contains_key(&cell_value) {
                        noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    } else {
                        noise_areas.insert(cell_value, vec![idx]);
                    }
                }
            }
        }

        // Spawn the entities
        for area in noise_areas.iter() {
            spawner::spawn_region(&build_data.map, area.1, build_data.map.depth, &mut build_data.spawn_list, SpawnTableType::Mob);
        }
    }
}

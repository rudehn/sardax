use crate::map_builders::{InitialMapBuilder, MetaMapBuilder, BuilderMap, TileType};

pub struct RexBuilder {
    xp_filename : String
}


impl InitialMapBuilder for RexBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.build(build_data);
    }
}

impl MetaMapBuilder for RexBuilder {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.build(build_data);
    }
}

impl RexBuilder {
    #[allow(dead_code)]
    pub fn new(filename: String) -> Box<RexBuilder> {
        Box::new(RexBuilder{xp_filename: filename})
    }

    #[allow(clippy::map_entry)]
    fn build(&mut self, build_data : &mut BuilderMap) {
        
        let xp_file = &rltk::rex::XpFile::from_resource(&self.xp_filename).unwrap();
        
        for layer in &xp_file.layers {
            for y in 0..layer.height {
                for x in 0..layer.width {
                    let cell = layer.get(x, y).unwrap();
                    if x < build_data.map.width as usize && y < build_data.map.height as usize {
                        let idx = build_data.map.xy_idx(x as i32, y as i32);
                        match cell.ch {
                            // 32 => build_data.map.tiles[idx] = TileType::Floor, // ' '
                            46 => build_data.map.tiles[idx] = TileType::Floor, // .
                            35 => build_data.map.tiles[idx] = TileType::Wall, // #
                            126 => build_data.map.tiles[idx] = TileType::Road, // ~
                            45 => build_data.map.tiles[idx] = TileType::ShallowWater, // -
                            61 => build_data.map.tiles[idx] = TileType::DeepWater, // =
                            _ => {}
                        }
                    }
                }
            }
        }
        build_data.take_snapshot();
    }
}

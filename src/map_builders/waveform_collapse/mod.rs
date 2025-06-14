use super::{MetaMapBuilder, BuilderMap, Map, TileType};
mod common;
use common::*;
mod constraints;
use constraints::*;
mod solver;
use solver::*;

/// Provides a map builder using the Wave Function Collapse algorithm.
pub struct WaveformCollapseBuilder {
    chunk_size: i32
}

impl MetaMapBuilder for WaveformCollapseBuilder {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl WaveformCollapseBuilder {
    /// Constructor for waveform collapse.
    #[allow(dead_code)]
    pub fn new() -> Box<WaveformCollapseBuilder> {
        WaveformCollapseBuilder::chunked(6)
    }

    pub fn chunked(chunk_size: i32) -> Box<WaveformCollapseBuilder> {
        Box::new(WaveformCollapseBuilder{chunk_size: chunk_size})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        build_data.take_snapshot();

        let patterns = build_patterns(&build_data.map, self.chunk_size, true, true);
        let constraints = patterns_to_constraints(patterns, self.chunk_size);
        self.render_tile_gallery(&constraints, self.chunk_size, build_data);

        let old_map = build_data.map.clone();

        build_data.map = Map::new(build_data.map.depth, build_data.width, build_data.height, &build_data.map.name);
        build_data.spawn_list.clear();
        build_data.rooms = None;
        build_data.corridors = None;
        let mut tries = 0;
        loop {
            let mut solver = Solver::new(constraints.clone(), self.chunk_size, &build_data.map);
            while !solver.iteration(&mut build_data.map) {
                build_data.take_snapshot();
            }
            build_data.take_snapshot();
            if solver.possible { break; } // If it has hit an impossible condition, try again
            tries += 1;
            if tries > 10 { break; }
        }

        if tries > 10 {
            // Restore the old one
            build_data.map = old_map;
        }
    }

    fn render_tile_gallery(&mut self, constraints: &[MapChunk], chunk_size: i32, build_data : &mut BuilderMap) {
        build_data.map = Map::new(build_data.map.depth, build_data.width, build_data.height, &build_data.map.name);
        let mut counter = 0;
        let mut x = 1;
        let mut y = 1;
        while counter < constraints.len() {
            render_pattern_to_map(&mut build_data.map, &constraints[counter], chunk_size, x, y);

            x += chunk_size + 1;
            if x + chunk_size > build_data.map.width {
                // Move to the next row
                x = 1;
                y += chunk_size + 1;

                if y + chunk_size > build_data.map.height {
                    // Move to the next page
                    build_data.take_snapshot();
                    build_data.map = Map::new(build_data.map.depth, build_data.width, build_data.height, &build_data.map.name);

                    x = 1;
                    y = 1;
                }
            }

            counter += 1;
        }
        build_data.take_snapshot();
    }
}
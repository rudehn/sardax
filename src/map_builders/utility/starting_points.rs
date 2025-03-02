use crate::map_builders::{MetaMapBuilder, BuilderMap, Position, TileType};
use crate::map;

#[allow(dead_code)]
pub enum XStart { LEFT, CENTER, RIGHT }

#[allow(dead_code)]
pub enum YStart { TOP, CENTER, BOTTOM }

pub struct AreaStartingPosition {
    x : XStart,
    y : YStart
}

impl MetaMapBuilder for AreaStartingPosition {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl AreaStartingPosition {
    #[allow(dead_code)]
    pub fn new(x : XStart, y : YStart) -> Box<AreaStartingPosition> {
        Box::new(AreaStartingPosition{
            x, y
        })
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        let seed_x;
        let seed_y;

        match self.x {
            XStart::LEFT => seed_x = 1,
            XStart::CENTER => seed_x = build_data.map.width / 2,
            XStart::RIGHT => seed_x = build_data.map.width - 2
        }

        match self.y {
            YStart::TOP => seed_y = 1,
            YStart::CENTER => seed_y = build_data.map.height / 2,
            YStart::BOTTOM => seed_y = build_data.map.height - 2
        }

        let mut available_floors : Vec<(usize, f32)> = Vec::new();
        for (idx, tiletype) in build_data.map.tiles.iter().enumerate() {
            if map::tile_walkable(*tiletype) {
                available_floors.push(
                    (
                        idx,
                        rltk::DistanceAlg::PythagorasSquared.distance2d(
                            rltk::Point::new(idx as i32 % build_data.map.width, idx as i32 / build_data.map.width),
                            rltk::Point::new(seed_x, seed_y)
                        )
                    )
                );
            }
        }
        if available_floors.is_empty() {
            panic!("No valid floors to start on");
        }

        available_floors.sort_by(|a,b| a.1.partial_cmp(&b.1).unwrap());

        let start_x = available_floors[0].0 as i32 % build_data.map.width;
        let start_y = available_floors[0].0 as i32 / build_data.map.width;

        build_data.starting_position = Some(Position{x : start_x, y: start_y});
    }
}


pub struct RoomBasedStartingPosition {}

impl MetaMapBuilder for RoomBasedStartingPosition {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl RoomBasedStartingPosition {
    #[allow(dead_code)]
    pub fn new() -> Box<RoomBasedStartingPosition> {
        Box::new(RoomBasedStartingPosition{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if let Some(rooms) = &build_data.rooms {
            let start_pos = rooms[0].center();
            build_data.starting_position = Some(Position{ x: start_pos.x, y: start_pos.y });
        } else {
            panic!("Room Based Staring Position only works after rooms have been created");
        }
    }
}


pub struct DungeonEntranceSpawner {}

impl MetaMapBuilder for DungeonEntranceSpawner {
    fn build_map(&mut self, build_data : &mut BuilderMap)  {
        self.build(build_data);
    }
}

impl DungeonEntranceSpawner {
    #[allow(dead_code)]
    pub fn new() -> Box<DungeonEntranceSpawner> {
        Box::new(DungeonEntranceSpawner{})
    }

    fn build(&mut self, build_data : &mut BuilderMap) {
        if build_data.map.depth == 1 {
            match &build_data.starting_position {
                Some(sp) => {
                    let dungeon_exit_idx = build_data.map.xy_idx(sp.x, sp.y);
                    build_data.map.tiles[dungeon_exit_idx] = TileType::DungeonExit;
                    build_data.take_snapshot();
                },
                None => panic!("Dungeon Entrance Spawner needs a map starting point!"),
            }
        } else {
            panic!("Dungeon Entrance Spawner can only be used on the first floor!");
        }
    }
}
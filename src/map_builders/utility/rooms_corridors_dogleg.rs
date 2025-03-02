use crate::map_builders::{MetaMapBuilder, BuilderMap, apply_horizontal_tunnel, apply_vertical_tunnel };
use rltk::Rect;
pub struct DoglegCorridors {}

impl MetaMapBuilder for DoglegCorridors {
    #[allow(dead_code)]
    fn build_map(&mut self, build_data : &mut BuilderMap) {
        self.corridors(build_data);
    }
}

impl DoglegCorridors {
    #[allow(dead_code)]
    pub fn new() -> Box<DoglegCorridors> {
        Box::new(DoglegCorridors{})
    }

    fn corridors(&mut self, build_data : &mut BuilderMap) {
        let rooms : Vec<Rect>;
        if let Some(rooms_builder) = &build_data.rooms {
            rooms = rooms_builder.clone();
        } else {
            panic!("Dogleg Corridors require a builder with room structures");
        }

        let mut corridors : Vec<Vec<usize>> = Vec::new();
        for (i,room) in rooms.iter().enumerate() {
            if i > 0 {
                let new_point = room.center();
                let prev_point = rooms[i as usize -1].center();
                if crate::rng::range(0,2) == 1 {
                    let mut c1 = apply_horizontal_tunnel(&mut build_data.map, prev_point.x, new_point.x, prev_point.y);
                    let mut c2 = apply_vertical_tunnel(&mut build_data.map, prev_point.y, new_point.y, new_point.x);
                    c1.append(&mut c2);
                    corridors.push(c1);
                } else {
                    let mut c1 = apply_vertical_tunnel(&mut build_data.map, prev_point.y, new_point.y, prev_point.x);
                    let mut c2 = apply_horizontal_tunnel(&mut build_data.map, prev_point.x, new_point.x, new_point.y);
                    c1.append(&mut c2);
                    corridors.push(c1);
                }
                build_data.take_snapshot();
            }
        }
        build_data.corridors = Some(corridors);
    }
}

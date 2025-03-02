use super::{Map, TileType};
use rltk::RGB;

pub fn tile_glyph(idx: usize, map : &Map) -> (rltk::FontCharType, RGB, RGB) {
    let (glyph, mut fg, mut bg) = match map.depth {
       
        _ => get_tile_glyph_default(idx, map)
    };

    if map.bloodstains.contains(&idx) { bg = RGB::from_f32(0.75, 0., 0.); }
    let visibility_mult = 0.16;
    // let visibility_mult = 0.08;
    if !map.visible_tiles[idx] {
        // fg = fg.desaturate();
        // fg = fg.to_greyscale();

        fg = fg * visibility_mult;
        bg = bg * visibility_mult;
        // bg = bg.desaturate();
        // bg = RGB::from_f32(0., 0., 0.); // Don't show stains out of visual range
    } else if !map.outdoors {
        // We need to cap the light multiplier to the max of the out-of-view multiplier
        // Otherwise we end up with a black circle surrounding the player, and then the out-of-view
        // Tiles are shown
        let light_mult = RGB::from_f32(
            f32::max(visibility_mult, map.light[idx].r),
            f32::max(visibility_mult, map.light[idx].g),
            f32::max(visibility_mult, map.light[idx].b),
        );
        fg = fg * light_mult;
        bg = bg * light_mult;
    }

    (glyph, fg, bg)
}


fn get_tile_glyph_default(idx: usize, map : &Map) -> (rltk::FontCharType, RGB, RGB) {
    let glyph;
    let fg;
    // let mut bg = RGB::from_f32(0., 0., 0.);
    let bg = RGB::from_f32(0.1, 0.1, 0.1);

    match map.tiles[idx] {
        TileType::Floor => { 
            glyph = rltk::to_cp437('.');
            // fg = RGB::from_u8(0x80, 0x80, 0x80);
            // bg = RGB::from_u8(0x40, 0x40, 0x40);
            fg = RGB::from_f32(0.3, 0.3, 0.3);
            // bg = RGB::from_f32(0.1, 0.1, 0.1);
            // fg = RGB::named(rltk::DARKGRAY);
            // bg = RGB::named(rltk::GRAY1);
        }
        TileType::WoodFloor => { glyph = rltk::to_cp437('░'); fg = RGB::named(rltk::CHOCOLATE); }
        TileType::Wall => {
            let x = idx as i32 % map.width;
            let y = idx as i32 / map.width;
            // glyph = wall_glyph(&*map, x, y);
            glyph = rltk::to_cp437('#');
            
            fg = RGB::from_f32(0.2, 0.2, 0.2);
            // bg = RGB::from_f32(0.1, 0.1, 0.1);
            // fg = RGB::from_u8(0x40, 0x40, 0x40);
            // bg = RGB::from_u8(0x80, 0x80, 0x80);
        }
        TileType::DownStairs => { glyph = rltk::to_cp437('>'); fg = RGB::from_f32(0., 1.0, 1.0); }
        TileType::DungeonExit => { glyph = rltk::to_cp437('Ω'); fg = RGB::from_f32(1.0, 1.0, 1.0); }
        TileType::UpStairs => { glyph = rltk::to_cp437('<'); fg = RGB::from_f32(0., 1.0, 1.0); }
        TileType::Bridge => { glyph = rltk::to_cp437('.'); fg = RGB::named(rltk::CHOCOLATE); }
        TileType::Road => { glyph = rltk::to_cp437('≡'); fg = RGB::named(rltk::GRAY); }
        TileType::Grass => { glyph = rltk::to_cp437('"'); fg = RGB::named(rltk::GREEN); }
        TileType::ShallowWater => { glyph = rltk::to_cp437('~'); fg = RGB::named(rltk::CYAN); }
        TileType::DeepWater => { glyph = rltk::to_cp437('~'); fg = RGB::named(rltk::BLUE); }
        TileType::Gravel => { glyph = rltk::to_cp437(';'); fg = RGB::from_f32(0.5, 0.5, 0.5); }
        TileType::Stalactite => { glyph = rltk::to_cp437('╨'); fg = RGB::from_f32(0.5, 0.5, 0.5); }
        TileType::Stalagmite => { glyph = rltk::to_cp437('╥'); fg = RGB::from_f32(0.5, 0.5, 0.5); }
    }

    (glyph, fg, bg)
}

fn wall_glyph(map : &Map, x: i32, y:i32) -> rltk::FontCharType {
    if x < 1 || x > map.width-2 || y < 1 || y > map.height-2 as i32 { return 35; }
    let mut mask : u8 = 0;

    if is_revealed_and_wall(map, x, y - 1) { mask +=1; }
    if is_revealed_and_wall(map, x, y + 1) { mask +=2; }
    if is_revealed_and_wall(map, x - 1, y) { mask +=4; }
    if is_revealed_and_wall(map, x + 1, y) { mask +=8; }

    match mask {
        0 => { 9 } // Pillar because we can't see neighbors
        1 => { 186 } // Wall only to the north
        2 => { 186 } // Wall only to the south
        3 => { 186 } // Wall to the north and south
        4 => { 205 } // Wall only to the west
        5 => { 188 } // Wall to the north and west
        6 => { 187 } // Wall to the south and west
        7 => { 185 } // Wall to the north, south and west
        8 => { 205 } // Wall only to the east
        9 => { 200 } // Wall to the north and east
        10 => { 201 } // Wall to the south and east
        11 => { 204 } // Wall to the north, south and east
        12 => { 205 } // Wall to the east and west
        13 => { 202 } // Wall to the east, west, and south
        14 => { 203 } // Wall to the east, west, and north
        15 => { 206 }  // ╬ Wall on all sides
        _ => { 35 } // We missed one?
    }
}

fn is_revealed_and_wall(map: &Map, x: i32, y: i32) -> bool {
    let idx = map.xy_idx(x, y);
    map.tiles[idx] == TileType::Wall && map.revealed_tiles[idx]
}

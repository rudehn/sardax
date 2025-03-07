use rltk::prelude::*;
use specs::prelude::*;
use crate::{gamelog, Consumable, Duration, Equipped, HungerClock, HungerState, InBackpack, KnownSpells, Map, Name, Pools, Renderable, StatusEffect, Weapon, Paralysis, Burning, Slow, Haste };
use super::{draw_tooltips, get_item_display_name, get_item_color};
use crate::vision::get_characters_in_vision;
use crate::constants::{STATUS_BURNING_COLOR, STATUS_GENERIC_COLOR, STATUS_HASTE_COLOR, STATUS_PARALYSIS_COLOR, STATUS_SLOW_COLOR};


pub fn draw_bar_horizontal(
    draw_batch: &mut DrawBatch,
    point: Point,
    width: i32,
    n: i32,
    max: i32,
    fg: RGBA,
    bg: RGBA,
) {
    let percent = n as f32 / max as f32;
    let fill_width = (percent * width as f32) as i32;
    for x in 0..width {
        if x <= fill_width {
            draw_batch.set(Point::new(point.x + x, point.y), ColorPair::new(fg, bg), to_cp437('█'));
        } else {
            draw_batch.set(Point::new(point.x + x, point.y), ColorPair::new(fg*0.6, bg), to_cp437('█'));
        }
    }
}

fn box_framework(draw_batch : &mut DrawBatch) {
    let box_gray : RGB = RGB::from_hex("#999999").expect("Oops");
    let black = RGB::named(rltk::BLACK);

    draw_batch.draw_hollow_box(Rect::with_size(0, 0, 119, 69), ColorPair::new(box_gray, black)); // Overall box
    draw_batch.draw_hollow_box(Rect::with_size(0, 0, 30, 8), ColorPair::new(box_gray, black)); // Top-left panel
    draw_batch.draw_hollow_box(Rect::with_size(30, 0, 89, 55), ColorPair::new(box_gray, black)); // Map box
    draw_batch.draw_hollow_box(Rect::with_size(0, 55, 119, 14), ColorPair::new(box_gray, black)); // Log box

    // Draw box connectors
    draw_batch.set(Point::new(0, 55), ColorPair::new(box_gray, black), to_cp437('├'));
    draw_batch.set(Point::new(0, 8), ColorPair::new(box_gray, black), to_cp437('├'));
    draw_batch.set(Point::new(30, 0), ColorPair::new(box_gray, black), to_cp437('┬'));
    draw_batch.set(Point::new(30, 55), ColorPair::new(box_gray, black), to_cp437('┴'));
    draw_batch.set(Point::new(30, 8), ColorPair::new(box_gray, black), to_cp437('┤'));
    draw_batch.set(Point::new(119, 55), ColorPair::new(box_gray, black), to_cp437('┤'));
}

pub fn map_label(ecs: &World, draw_batch: &mut DrawBatch) {
    let box_gray : RGB = RGB::from_hex("#999999").expect("Oops");
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);

    let map = ecs.fetch::<Map>();
    let name_length = map.name.len() + 2;
    let x_pos = (75 - (name_length / 2)) as i32;
    draw_batch.set(Point::new(x_pos, 0), ColorPair::new(box_gray, black), to_cp437('┤'));
    draw_batch.set(Point::new(x_pos + name_length as i32 - 1, 0), ColorPair::new(box_gray, black), to_cp437('├'));
    draw_batch.print_color(Point::new(x_pos+1, 0), &map.name, ColorPair::new(white, black));
}

fn draw_stats(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);
    let pools = ecs.read_storage::<Pools>();
    let player_pools = pools.get(*player_entity).unwrap();
    let health = format!("Health: {}/{}", player_pools.hit_points.current, player_pools.hit_points.max);
    let mana =   format!("Mana:   {}/{}", player_pools.mana.current, player_pools.mana.max);
    draw_batch.print_color(Point::new(1, 1), &health, ColorPair::new(white, black));
    draw_batch.print_color(Point::new(1, 2), &mana, ColorPair::new(white, black));
    draw_bar_horizontal(
        draw_batch,
        Point::new(15, 1), 
        14, 
        player_pools.hit_points.current, 
        player_pools.hit_points.max, 
        RGBA::named(rltk::RED),
        RGBA::named(rltk::BLACK)
    );
    draw_bar_horizontal(
        draw_batch,
        Point::new(15, 2), 
        14, 
        player_pools.mana.current, 
        player_pools.mana.max, 
        RGBA::named(rltk::BLUE), 
        RGBA::named(rltk::BLACK)
    );
}

fn equipped(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) -> i32 {
    let black = RGB::named(rltk::BLACK);
    let yellow = RGB::named(rltk::YELLOW);
    let mut y = 13;
    let entities = ecs.entities();
    let equipped = ecs.read_storage::<Equipped>();
    let weapon = ecs.read_storage::<Weapon>();
    for (entity, equipped_by) in (&entities, &equipped).join() {
        if equipped_by.owner == *player_entity {
            let name = get_item_display_name(ecs, entity);
            draw_batch.print_color(
                Point::new(1, y), 
                &name,
                ColorPair::new(get_item_color(ecs, entity), black));
            y += 1;

            if let Some(weapon) = weapon.get(entity) {
                let mut weapon_info = if weapon.damage_bonus < 0 {
                    format!("┤ {} ({}d{}{})", &name, weapon.damage_n_dice, weapon.damage_die_type, weapon.damage_bonus)
                } else if weapon.damage_bonus == 0 {
                    format!("┤ {} ({}d{})", &name, weapon.damage_n_dice, weapon.damage_die_type)
                } else {
                    format!("┤ {} ({}d{}+{})", &name, weapon.damage_n_dice, weapon.damage_die_type, weapon.damage_bonus)
                };

                if let Some(range) = weapon.range {
                    weapon_info += &format!(" (range: {}, F to fire, V cycle targets)", range);
                }
                weapon_info += " ├";
                draw_batch.print_color(
                    Point::new(3, 55),
                    &weapon_info,
                    ColorPair::new(yellow, black));
            }
        }
    }
    y
}

fn consumables(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity, mut y : i32) -> i32 {
    y += 1;
    let black = RGB::named(rltk::BLACK);
    let yellow = RGB::named(rltk::YELLOW);
    let entities = ecs.entities();
    let consumables = ecs.read_storage::<Consumable>();
    let backpack = ecs.read_storage::<InBackpack>();
    let mut index = 1;
    for (entity, carried_by, _consumable) in (&entities, &backpack, &consumables).join() {
        if carried_by.owner == *player_entity && index < 10 {
            draw_batch.print_color(
                Point::new(1, y), 
                &format!("↑{}", index),
                ColorPair::new(yellow, black)
            );
            draw_batch.print_color(
                Point::new(4, y), 
                &get_item_display_name(ecs, entity),
                ColorPair::new(get_item_color(ecs, entity), black)
            );
            y += 1;
            index += 1;
        }
    }
    y
}

fn spells(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity, mut y : i32) -> i32 {
    y += 1;
    let black = RGB::named(rltk::BLACK);
    let blue = RGB::named(rltk::CYAN);
    let known_spells_storage = ecs.read_storage::<KnownSpells>();
    let known_spells = &known_spells_storage.get(*player_entity).unwrap().spells;
    let mut index = 1;
    for spell in known_spells.iter() {
        draw_batch.print_color(
            Point::new(1, y),
            &format!("^{}", index),
            ColorPair::new(blue, black)
        );
        draw_batch.print_color(
            Point::new(4, y),
            &format!("{} ({})", &spell.display_name, spell.mana_cost),
            ColorPair::new(blue, black)
        );
        index += 1;
        y += 1;
    }
    y
}

fn status(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let mut y = 54;
    let hunger = ecs.read_storage::<HungerClock>();
    let hc = hunger.get(*player_entity).unwrap();
    match hc.state {
        HungerState::WellFed => {
            draw_batch.print_color(
                Point::new(1, y), 
                "Well Fed",
                ColorPair::new(RGB::named(rltk::GREEN), RGB::named(rltk::BLACK))
            );
            y -= 1;
        }
        HungerState::Normal => {}
        HungerState::Hungry => {
            draw_batch.print_color(
                Point::new(1, y),
                "Hungry",
                ColorPair::new(RGB::named(rltk::ORANGE), RGB::named(rltk::BLACK))
            );
            y -= 1;
        }
        HungerState::Starving => {
            draw_batch.print_color(
                Point::new(1, y),
                "Starving",
                ColorPair::new(RGB::named(rltk::RED), RGB::named(rltk::BLACK))
            );
            y -= 1;
        }
    }
    let statuses = ecs.read_storage::<StatusEffect>();
    let durations = ecs.read_storage::<Duration>();
    let names = ecs.read_storage::<Name>();
    for (status, duration, name) in (&statuses, &durations, &names).join() {
        if status.target == *player_entity {
            draw_batch.print_color(
                Point::new(1, y),
                &format!("{} ({})", name.name, duration.turns),
                ColorPair::new(RGB::named(rltk::RED), RGB::named(rltk::BLACK)),
            );
            y -= 1;
        }
    }
}

fn draw_entity_in_view(ecs: &World, draw_batch: &mut DrawBatch, entity: &Entity, y: i32) -> i32 {
    
    let black = RGB::named(rltk::BLACK);
    let white = RGB::named(rltk::WHITE);
    let names = ecs.read_storage::<Name>();
    let renderables = ecs.read_storage::<Renderable>();
    let pools = ecs.read_storage::<Pools>();
    let statuses = ecs.read_storage::<StatusEffect>();
    let durations = ecs.read_storage::<Duration>();
    let paralysis = ecs.read_storage::<Paralysis>();
    let burning = ecs.read_storage::<Burning>();
    let slow = ecs.read_storage::<Slow>();
    let haste = ecs.read_storage::<Haste>();
    let entities = ecs.entities();

    let mut lines_drawn = 0;
    let mut display_name = "";
    let mut display_char = 0;
    let mut fg = RGB::named(rltk::WHITE);
    if let Some(name) = names.get(*entity){
        display_name = &name.name;
    }
    if let Some(renderable) = renderables.get(*entity){
        display_char = renderable.glyph;
        fg = renderable.fg;
    }

    if !display_name.is_empty() && display_char != 0{
        lines_drawn += 1;
        draw_batch.set(
            Point::new(1, y),
            ColorPair::new(fg, black),
            display_char
        );
        draw_batch.print_color(
            Point::new(2, y),
            &format!(": {}", display_name),
            ColorPair::new(fg, black),
        );
    }
    if let Some(pools) = pools.get(*entity){
        let health = "Health";
        draw_batch.print_color(Point::new(1, y + lines_drawn), &health, ColorPair::new(white, black));
        draw_bar_horizontal(
            draw_batch,
            Point::new(8, y + lines_drawn), 
            14, 
            pools.hit_points.current, 
            pools.hit_points.max, 
            RGBA::named(rltk::RED),
            RGBA::named(rltk::BLACK)
        );
        lines_drawn += 1;
    }
    // TODO - optimize, can we call in entities function & pass in hashmap?
    for (status_entity, status, duration, name) in (&entities, &statuses, &durations, &names).join() {
        let mut color = STATUS_GENERIC_COLOR;
        if let Some(_para) = paralysis.get(status_entity){
            color = STATUS_PARALYSIS_COLOR;
        } else if let Some(_burn) = burning.get(status_entity) {
            color = STATUS_BURNING_COLOR;
        } else if let Some(_slow) = slow.get(status_entity) {
            color = STATUS_SLOW_COLOR;
        } else if let Some(_haste) = haste.get(status_entity) {
            color = STATUS_HASTE_COLOR;
        }
        if status.target == *entity {
            // Print status name
            draw_batch.print_color(Point::new(1, y + lines_drawn), &name.name, ColorPair::new(white, black));
            draw_bar_horizontal(
                draw_batch,
                Point::new(10, y + lines_drawn), 
                14, 
                duration.turns, 
                duration.total_turns, 
                RGBA::named(color),
                RGBA::named(rltk::BLACK)
            );
            lines_drawn += 1;
        }
    }

    lines_drawn
}

fn entities(ecs: &World, draw_batch: &mut DrawBatch, player_entity: &Entity) {
    let entities = get_characters_in_vision(ecs, player_entity);
    let mut y = 1;
    let player_lines_drawn = draw_entity_in_view(ecs, draw_batch, player_entity, y);
    y += player_lines_drawn + 1; // Give an extra row to space out entities

    for entity in entities.iter(){
        let lines_drawn = draw_entity_in_view(ecs, draw_batch, entity, y);
        y += lines_drawn + 1; // Give an extra row to space out entities
    }
}

pub fn draw_ui(ecs: &World, ctx : &mut Rltk) {
    let mut draw_batch = DrawBatch::new();
    let player_entity = ecs.fetch::<Entity>();

    // box_framework(&mut draw_batch);
    map_label(ecs, &mut draw_batch);
    // draw_stats(ecs, &mut draw_batch, &player_entity);
    entities(ecs, &mut draw_batch, &player_entity);
    // let mut y = equipped(ecs, &mut draw_batch, &player_entity);
    // y += consumables(ecs, &mut draw_batch, &player_entity, y);
    // spells(ecs, &mut draw_batch, &player_entity, y);
    // status(ecs, &mut draw_batch, &player_entity);
    gamelog::print_log(&mut rltk::BACKEND_INTERNAL.lock().consoles[1].console, Point::new(30, 0));
    draw_tooltips(ecs, ctx);

    draw_batch.submit(5000);
}
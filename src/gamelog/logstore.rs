use std::sync::Mutex;
use super::LogFragment;
use rltk::prelude::*;
use crate::constants::GAME_LOG_MESSAGES;

lazy_static! {
    static ref LOG : Mutex<Vec<Vec<LogFragment>>> = Mutex::new(Vec::new());
}

pub fn append_entry(fragments : Vec<LogFragment>) {
    LOG.lock().unwrap().push(fragments);
}

pub fn clear_log() {
    LOG.lock().unwrap().clear();
}

pub fn print_log(console: &mut Box<dyn Console>, pos: Point) {
    let mut y = pos.y;
    let mut x = pos.x;
    let n = GAME_LOG_MESSAGES as usize;
    // Grab the last 4 messages, then reverse to print them so the most recent is at the bottom
    LOG.lock().unwrap().iter().rev().take(n).rev().for_each(|log| {
        log.iter().for_each(|frag| {
            console.print_color(x, y, frag.color.to_rgba(1.0), RGBA::named(rltk::BLACK), &frag.text);
            x += frag.text.len() as i32;
            x += 1;
        });
        y += 1;
        x = pos.x;
    });
}

pub fn clone_log() -> Vec<Vec<crate::gamelog::LogFragment>> {
    LOG.lock().unwrap().clone()
}

pub fn restore_log(log : &mut Vec<Vec<crate::gamelog::LogFragment>>) {
    LOG.lock().unwrap().clear();
    LOG.lock().unwrap().append(log);
}
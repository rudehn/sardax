use rltk::{BLUE, GREEN, ORANGE, PURPLE, YELLOW};

// DEBUG
pub const SHOW_MAPGEN_VISUALIZER : bool = false;
pub const SHOW_FPS : bool = true;

// Sizing
pub const SCREEN_WIDTH: i32 = 120;
pub const SCREEN_HEIGHT: i32 = 80;
pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const GAME_LOG_MESSAGES: i32 = 4;
pub const PANEL_PADDING: i32 = 2;
pub const LEFT_PANEL_WIDTH: i32 = 30;

// Game Mechanics
pub const AMULET_LEVEL: i32 = 10;

// Colors for consistent UI displaying status effects
pub const STATUS_GENERIC_COLOR: (u8, u8, u8) = BLUE;
pub const STATUS_PARALYSIS_COLOR: (u8, u8, u8) = YELLOW;
pub const STATUS_BURNING_COLOR: (u8, u8, u8) = ORANGE;
pub const STATUS_HASTE_COLOR: (u8, u8, u8) = GREEN;
pub const STATUS_SLOW_COLOR: (u8, u8, u8) = PURPLE;


// Energy/Action point related
// These determine how frequently the entitity can take actions
pub const TICKS_PER_TURN: i32 = 100;
pub const DEFAULT_ACTION_COST: i32 = 100; // Default global action cost needed to perform a generic action
pub const DEFAULT_ENERGY_GAIN: i32 = DEFAULT_ACTION_COST / TICKS_PER_TURN;  // Default global action recovery speed per tick for all entities
pub const MOVE_ACTION_COST: i32 = 100;    // Default amount of energy needed to perform a move action
pub const ATTACK_ACTION_COST: i32 = 100;  // Default amount of energy needed to perform an attack action
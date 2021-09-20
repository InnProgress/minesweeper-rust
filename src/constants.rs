use crate::position::Position;
use eframe::egui::Color32;

pub const ADJACENT_TILE_OFFSETS: [Position; 8] = [
    Position { x: -1, y: -1 },
    Position { x: 0, y: -1 },
    Position { x: 1, y: -1 },
    Position { x: 1, y: 0 },
    Position { x: 1, y: 1 },
    Position { x: 0, y: 1 },
    Position { x: -1, y: 1 },
    Position { x: -1, y: 0 },
];

pub const DEFAULT_BOARD_HEIGHT: u8 = 9;
pub const DEFAULT_BOARD_WIDTH: u8 = 9;
pub const DEFAULT_BOARD_MINES: u8 = 10;
pub const CELL_SIZE: f32 = 30.0;
pub const BLUE: Color32 = Color32::from_rgb(0, 0, 253);
pub const GREEN: Color32 = Color32::from_rgb(1, 126, 0);
pub const RED: Color32 = Color32::from_rgb(254, 0, 0);
pub const DARK_BLUE: Color32 = Color32::from_rgb(1, 1, 128);
pub const DARK_RED: Color32 = Color32::from_rgb(126, 3, 3);
pub const PERSIAN_GREEN: Color32 = Color32::from_rgb(0, 128, 128);
pub const GREY: Color32 = Color32::from_rgb(128, 128, 128);
pub const BLACK: Color32 = Color32::BLACK;
pub const WINDOW_X_OFFSET: f32 = 20.;
pub const WINDOW_Y_OFFSET: f32 = 60.;

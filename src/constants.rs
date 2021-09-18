use crate::position::Position;

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

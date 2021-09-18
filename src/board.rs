use crate::constants;
use crate::tile::{PublicTile, Tile};
use rand::Rng;

#[derive(PartialEq, Eq)]
pub enum FinishedState {
    Won,
    Lost,
}

#[derive(PartialEq, Eq)]
pub enum State {
    New,
    Playing,
    Finished(FinishedState),
}

pub struct Board {
    pub state: State,
    initial_mines: u8,
    pub mines: u8,
    pub visible: Vec<Vec<PublicTile>>,
    pub height: u8,
    pub width: u8,
    tiles: Vec<Vec<Tile>>,
}

impl Board {
    pub fn new(height: u8, width: u8, mines: u8) -> Self {
        let tiles = vec![vec![Tile::Empty; width as usize]; height as usize];
        let visible = vec![vec![PublicTile::Hidden; width as usize]; height as usize];

        Self {
            state: State::New,
            initial_mines: mines,
            mines,
            visible,
            height,
            width,
            tiles,
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new(self.height, self.width, self.initial_mines);
    }

    pub fn capture(&mut self, x: u8, y: u8) -> Option<FinishedState> {
        if self.state == State::New {
            self.generate_tiles(x, y);
            self.state = State::Playing;
        } else if self.state != State::Playing || self.get_visible_tile(x, y) == PublicTile::Mine {
            return None;
        }

        self.set_tile_visible(x, y);

        if self.get_tile(x, y) == Tile::Mine {
            self.state = State::Finished(FinishedState::Lost);
            return Some(FinishedState::Lost);
        } else if self.get_tile(x, y) == Tile::Empty {
            self.capture_empty_path(x, y);
        }

        if self.is_everything_captured() {
            self.state = State::Finished(FinishedState::Won);
            return Some(FinishedState::Won);
        }

        None
    }

    pub fn capture_mine(&mut self, x: u8, y: u8) {
        if self.state != State::Playing {
            return;
        }
        self.toggle_tile_mine_capture(x, y);
    }

    fn generate_tiles(&mut self, starting_x: u8, starting_y: u8) {
        self.generate_mines(starting_x, starting_y);
        self.generate_tips();
    }

    fn generate_mines(&mut self, starting_x: u8, starting_y: u8) {
        let mut rng = rand::thread_rng();
        for _ in 0..self.mines {
            let mut x = rng.gen_range(0..self.width);
            let mut y = rng.gen_range(0..self.height);
            while self.get_tile(x, y) != Tile::Empty
                || (x == starting_x.into() && y == starting_y.into())
            {
                x = rng.gen_range(0..self.width);
                y = rng.gen_range(0..self.height);
            }
            self.set_tile(x, y, Tile::Mine);
        }
    }

    fn generate_tips(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_tile(x, y) != Tile::Empty {
                    continue;
                }

                let adjacent_mines = constants::ADJACENT_TILE_OFFSETS
                    .iter()
                    .filter(|offset| self.is_tile_mine(x as i8 + offset.x, y as i8 + offset.y))
                    .count() as u8;

                if adjacent_mines > 0 {
                    self.set_tile(x, y, Tile::Tip(adjacent_mines));
                }
            }
        }
    }

    fn capture_empty_path(&mut self, x: u8, y: u8) {
        for adjacent_coordinate in constants::ADJACENT_TILE_OFFSETS {
            let adjacent_x = x as i8 + adjacent_coordinate.x;
            let adjacent_y = y as i8 + adjacent_coordinate.y;

            if !self.is_valid_coordinate(adjacent_x, adjacent_y) {
                continue;
            }

            let adjacent_x = adjacent_x as u8;
            let adjacent_y = adjacent_y as u8;

            let adjacent_tile_before_visibility = self.get_visible_tile(adjacent_x, adjacent_y);
            self.set_tile_visible(adjacent_x, adjacent_y);
            if self.get_tile(adjacent_x, adjacent_y) == Tile::Empty
                && adjacent_tile_before_visibility == PublicTile::Hidden
            {
                self.capture_empty_path(adjacent_x, adjacent_y);
            }
        }
    }

    fn is_everything_captured(&mut self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_visible_tile(x, y) == PublicTile::Hidden
                    && self.get_tile(x, y) != Tile::Mine
                {
                    return false;
                }
            }
        }

        true
    }

    fn get_tile(&self, x: u8, y: u8) -> Tile {
        self.tiles[y as usize][x as usize].clone()
    }

    fn get_visible_tile(&self, x: u8, y: u8) -> PublicTile {
        self.visible[y as usize][x as usize].clone()
    }

    fn set_tile(&mut self, x: u8, y: u8, tile: Tile) {
        self.tiles[y as usize][x as usize] = tile;
    }

    fn set_tile_visible(&mut self, x: u8, y: u8) {
        self.visible[y as usize][x as usize] = PublicTile::Visible(self.get_tile(x, y).clone());
    }

    fn toggle_tile_mine_capture(&mut self, x: u8, y: u8) {
        if self.get_visible_tile(x, y) == PublicTile::Hidden && self.mines > 0 {
            self.visible[y as usize][x as usize] = PublicTile::Mine;
            self.mines -= 1;
        } else if self.get_visible_tile(x, y) == PublicTile::Mine {
            self.visible[y as usize][x as usize] = PublicTile::Hidden;
            self.mines += 1;
        }
    }

    fn is_tile_mine(&self, x: i8, y: i8) -> bool {
        self.is_valid_coordinate(x, y) && self.tiles[y as usize][x as usize] == Tile::Mine
    }

    fn is_valid_coordinate(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < self.width as i8 && y >= 0 && y < self.height as i8
    }
}

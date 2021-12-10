use crate::cell::{Cell, VisibleCell};
use crate::constants;
use crate::memento::{BoardMemento, Originator};
use crate::position::Position;
use crate::state::{FinishedState, State};
use rand::rngs::StdRng;
use rand::{Rng, RngCore, SeedableRng};

#[derive(Clone)]
pub struct Board {
    pub seed: Option<u64>,
    state: State,
    height: u8,
    width: u8,
    initial_mines: u8,
    mines: u8,
    visible_cells: Vec<Vec<VisibleCell>>,
    cells: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(height: u8, width: u8, mines: u8) -> Self {
        let cells = vec![vec![Cell::Empty; width as usize]; height as usize];
        let visible_cells = vec![vec![VisibleCell::Covered; width as usize]; height as usize];

        Self {
            seed: None,
            state: State::New,
            height,
            width,
            initial_mines: mines,
            mines,
            visible_cells,
            cells,
        }
    }

    pub fn reset(&mut self) {
        *self = Self::new(self.height, self.width, self.initial_mines);
    }

    pub fn uncover_cell(&mut self, x: u8, y: u8) {
        if self.state == State::New {
            self.fill_cells(x, y);
            self.state = State::Playing;
        } else if self.state != State::Playing
            || self.get_visible_cell(x, y) == VisibleCell::Flagged
        {
            return;
        }

        self.set_cell_visible(x, y);
        if self.get_cell(x, y) == Cell::Empty {
            self.uncover_empty_cells(x, y);
        }

        self.check_for_end_of_game(x, y);
    }

    pub fn flag_cell(&mut self, x: u8, y: u8) {
        if self.state != State::Playing {
            return;
        }

        if self.get_visible_cell(x, y) == VisibleCell::Covered && self.mines > 0 {
            self.visible_cells[y as usize][x as usize] = VisibleCell::Flagged;
            self.mines -= 1;
        } else if self.get_visible_cell(x, y) == VisibleCell::Flagged {
            self.visible_cells[y as usize][x as usize] = VisibleCell::Covered;
            self.mines += 1;
        }
    }

    fn check_for_end_of_game(&mut self, x: u8, y: u8) {
        if self.get_cell(x, y) == Cell::Mine {
            self.state = State::Finished(FinishedState::Lost);
        } else if self.is_everything_uncovered() {
            self.state = State::Finished(FinishedState::Won);
        }
    }

    fn fill_cells(&mut self, starting_x: u8, starting_y: u8) {
        let starting_positions = self.get_starting_positions(starting_x, starting_y);
        self.generate_mines(starting_positions);
    }

    fn get_starting_positions(&mut self, starting_x: u8, starting_y: u8) -> Vec<Position> {
        let mut starting_positions = vec![Position {
            x: starting_x as i8,
            y: starting_y as i8,
        }];

        for adjacent_coordinate in constants::ADJACENT_TILE_OFFSETS {
            let adjacent_x = starting_x as i8 + adjacent_coordinate.x;
            let adjacent_y = starting_y as i8 + adjacent_coordinate.y;

            if self.is_valid_coordinate(adjacent_x, adjacent_y) {
                starting_positions.push(Position {
                    x: adjacent_x,
                    y: adjacent_y,
                });
            }
        }

        starting_positions
    }

    fn generate_mines(&mut self, starting_positions: Vec<Position>) {
        let mut rng: Box<dyn RngCore> = match self.seed {
            Some(seed) => Box::new(StdRng::seed_from_u64(seed)),
            None => Box::new(rand::thread_rng()),
        };

        for _ in 0..self.mines {
            let mut x;
            let mut y;
            while {
                x = rng.gen_range(0..self.width);
                y = rng.gen_range(0..self.height);

                self.get_cell(x, y) != Cell::Empty
                    || starting_positions
                        .iter()
                        .find(|position| position.x == x as i8 && position.y == y as i8)
                        .is_some()
            } {}
            self.set_cell(x, y, Cell::Mine);
            println!("mine is here {} {}", x, y);
        }
    }

    fn generate_cell_clue(&mut self, x: u8, y: u8) {
        if self.get_cell(x, y) != Cell::Empty {
            return;
        }

        let adjacent_mines = constants::ADJACENT_TILE_OFFSETS
            .iter()
            .filter(|offset| self.is_mine(x as i8 + offset.x, y as i8 + offset.y))
            .count() as u8;

        if adjacent_mines > 0 {
            self.set_cell(x, y, Cell::Clue(adjacent_mines));
        }
    }

    fn uncover_empty_cells(&mut self, x: u8, y: u8) {
        for adjacent_coordinate in constants::ADJACENT_TILE_OFFSETS {
            let adjacent_x = x as i8 + adjacent_coordinate.x;
            let adjacent_y = y as i8 + adjacent_coordinate.y;

            if !self.is_valid_coordinate(adjacent_x, adjacent_y) {
                continue;
            }

            let adjacent_x = adjacent_x as u8;
            let adjacent_y = adjacent_y as u8;

            let adjacent_tile_before_visibility = self.get_visible_cell(adjacent_x, adjacent_y);
            self.set_cell_visible(adjacent_x, adjacent_y);
            if self.get_cell(adjacent_x, adjacent_y) == Cell::Empty
                && adjacent_tile_before_visibility == VisibleCell::Covered
            {
                self.uncover_empty_cells(adjacent_x, adjacent_y);
            }
        }
    }

    fn is_everything_uncovered(&mut self) -> bool {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.get_visible_cell(x, y) == VisibleCell::Covered
                    && self.get_cell(x, y) != Cell::Mine
                {
                    return false;
                }
            }
        }

        true
    }

    pub fn get_cell(&self, x: u8, y: u8) -> Cell {
        self.cells[y as usize][x as usize].clone()
    }

    pub fn get_visible_cell(&self, x: u8, y: u8) -> VisibleCell {
        self.visible_cells[y as usize][x as usize].clone()
    }

    fn set_cell(&mut self, x: u8, y: u8, cell: Cell) {
        self.cells[y as usize][x as usize] = cell;
    }

    fn set_cell_visible(&mut self, x: u8, y: u8) {
        self.generate_cell_clue(x, y);
        self.visible_cells[y as usize][x as usize] =
            VisibleCell::Uncovered(self.get_cell(x, y).clone());
    }

    fn is_mine(&self, x: i8, y: i8) -> bool {
        self.is_valid_coordinate(x, y) && self.get_cell(x as u8, y as u8) == Cell::Mine
    }

    fn is_valid_coordinate(&self, x: i8, y: i8) -> bool {
        x >= 0 && x < self.width as i8 && y >= 0 && y < self.height as i8
    }

    pub fn get_state(&self) -> &State {
        &self.state
    }

    pub fn get_mines_number(&self) -> u8 {
        self.mines
    }

    pub fn get_height(&self) -> u8 {
        self.height
    }

    pub fn get_width(&self) -> u8 {
        self.width
    }
}

impl Originator<BoardMemento> for Board {
    fn save_memento(&self) -> Box<BoardMemento> {
        Box::new(BoardMemento {
            state: self.state.clone(),
            height: self.height,
            width: self.width,
            initial_mines: self.initial_mines,
            mines: self.mines,
            visible_cells: self.visible_cells.clone(),
            cells: self.cells.clone(),
        })
    }

    fn restore_from_memento(&mut self, memento: Box<BoardMemento>) {
        self.state = memento.state;
        self.height = memento.height;
        self.width = memento.width;
        self.initial_mines = memento.initial_mines;
        self.mines = memento.mines;
        self.visible_cells = memento.visible_cells;
        self.cells = memento.cells;
    }
}

use crate::board::Board;

#[derive(Clone)]
pub struct BoardBuilder {
    board: Board,
}

impl BoardBuilder {
    pub fn new(height: u8, width: u8, mines: u8) -> Self {
        Self {
            board: Board::new(height, width, mines),
        }
    }
    pub fn set_seed(&mut self, seed: u64) -> Self {
        self.board.seed = Some(seed);
        self.clone()
    }
    pub fn build(&self) -> Result<Board, &'static str> {
        if self.board.get_mines_number() as i16
            > self.board.get_height() as i16 * self.board.get_width() as i16 - 3 as i16
        {
            return Err("Wrong amount of mines in comparison with width and height");
        }

        Ok(self.board.clone())
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board_builder::BoardBuilder,
        memento::{Caretaker, Originator},
        state::{FinishedState, State},
    };

    fn get_test_builder() -> BoardBuilder {
        BoardBuilder::new(3, 3, 2)
    }

    #[test]
    fn wins_game() {
        /*
           SEED 1
           1_🚩
           __2
           __🚩
        */
        let mut board = get_test_builder().set_seed(1).build().unwrap();

        board.uncover_cell(0, 0);
        board.uncover_cell(2, 1);
        let state = board.get_state();
        assert_eq!(*state, State::Finished(FinishedState::Won));
    }

    #[test]
    fn loses_game() {
        /*
           SEED 2
           1_🚩
           _💣_
           ___
        */
        let mut board = get_test_builder().set_seed(2).build().unwrap();

        board.uncover_cell(0, 0);
        board.uncover_cell(1, 2);
        let state = board.get_state();
        assert_eq!(*state, State::Finished(FinishedState::Lost));
    }

    #[test]
    fn restores_board_state() {
        /*
           SEED 3
           1__
           __🚩
           💣__
        */
        let mut caretaker = Caretaker::new();
        let mut board = get_test_builder().set_seed(3).build().unwrap();
        caretaker.add_memento(board.save_memento());

        board.uncover_cell(0, 0);
        caretaker.add_memento(board.save_memento());

        board.uncover_cell(0, 2);
        let state = board.get_state();
        assert_eq!(*state, State::Finished(FinishedState::Lost));

        board.restore_from_memento(caretaker.get_last_memento().unwrap());

        let state = board.get_state();
        assert_eq!(*state, State::Playing);
    }
}

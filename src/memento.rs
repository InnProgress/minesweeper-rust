use crate::{
    cell::{Cell, VisibleCell},
    state::State,
};

pub struct Caretaker<G: Memento> {
    mementos: Vec<Box<G>>,
}

impl<G: Memento> Caretaker<G> {
    pub fn new() -> Self {
        Self { mementos: vec![] }
    }
    pub fn add_memento(&mut self, memento: Box<G>) {
        self.mementos.push(memento);
    }
    pub fn get_last_memento(&mut self) -> Option<Box<G>> {
        self.mementos.pop()
    }
}

pub trait Originator<G: Memento> {
    fn save_memento(&self) -> Box<G>;
    fn restore_from_memento(&mut self, memento: Box<G>);
}

pub trait Memento {}

pub struct BoardMemento {
    pub state: State,
    pub height: u8,
    pub width: u8,
    pub initial_mines: u8,
    pub mines: u8,
    pub visible_cells: Vec<Vec<VisibleCell>>,
    pub cells: Vec<Vec<Cell>>,
}

impl Memento for BoardMemento {}

#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Mine,
    Clue(u8),
    Empty,
}

#[derive(Clone, PartialEq, Eq)]
pub enum VisibleCell {
    Uncovered(Cell),
    Covered,
    Flagged,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Tile {
    Mine,
    Tip(u8),
    Empty,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum PublicTile {
    Visible(Tile),
    Hidden,
    Mine,
}

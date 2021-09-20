#[derive(Clone, PartialEq, Eq)]
pub enum FinishedState {
    Won,
    Lost,
}

#[derive(Clone, PartialEq, Eq)]
pub enum State {
    New,
    Playing,
    Finished(FinishedState),
}

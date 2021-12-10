#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FinishedState {
    Won,
    Lost,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum State {
    New,
    Playing,
    Finished(FinishedState),
}

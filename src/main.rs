use minesweeper::{constants, gui};

fn main() {
    let app = gui::MinesweeperApp::default();
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(gui::MinesweeperApp::calculate_size(
        constants::DEFAULT_BOARD_HEIGHT,
        constants::DEFAULT_BOARD_WIDTH,
    ));
    eframe::run_native(Box::new(app), options);
}

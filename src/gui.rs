use crate::{
    board::Board,
    cell::{Cell, VisibleCell},
    constants,
    state::{FinishedState, State},
};
use eframe::{
    egui::{self, Color32, TextStyle, Ui, Vec2},
    epi,
};

pub struct MinesweeperApp {
    board: Board,
    error: Option<String>,
    settings_modal_opened: bool,
    height_input: String,
    width_input: String,
    mines_input: String,
}

impl MinesweeperApp {
    pub fn calculate_size(height: u8, width: u8) -> Vec2 {
        Vec2 {
            x: constants::WINDOW_X_OFFSET + width as f32 * constants::CELL_SIZE,
            y: constants::WINDOW_Y_OFFSET + height as f32 * constants::CELL_SIZE,
        }
    }

    fn draw_top_menu(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Settings").clicked() {
                        self.settings_modal_opened = true;
                    }
                    if ui.button("Restart").clicked() {
                        self.board.reset();
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });
    }

    fn draw_board_panel(&mut self, ctx: &egui::CtxRef) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new(format!("Mines: {}", self.board.mines)));

            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 0.0;
                for y in 0..self.board.height {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 0.0;

                        for x in 0..self.board.width {
                            self.draw_cell(ui, x, y);
                        }
                    });
                }
            });
        });
    }

    fn draw_end_of_game_modal(&mut self, ctx: &egui::CtxRef, finished_state: FinishedState) {
        egui::Window::new("End of game").show(ctx, |ui| {
            match finished_state {
                FinishedState::Won => ui.label("You have won!"),
                FinishedState::Lost => ui.label("You have lost!"),
            };

            if ui.button("Start new game").clicked() {
                self.board.reset();
            }
        });
    }

    fn draw_settings_modal(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::Window::new("Settings").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("Height: ");
                ui.text_edit_singleline(&mut self.height_input);
            });
            ui.horizontal(|ui| {
                ui.label("Width: ");
                ui.text_edit_singleline(&mut self.width_input);
            });
            ui.horizontal(|ui| {
                ui.label("Mines: ");
                ui.text_edit_singleline(&mut self.mines_input);
            });
            if let Some(error) = &self.error {
                ui.label(format!("Error: {}", error));
            }
            if ui.button("Change").clicked() {
                let height = match self.height_input.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        return;
                    }
                };
                let width = match self.width_input.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        return;
                    }
                };
                let mines = match self.mines_input.parse() {
                    Ok(value) => value,
                    Err(_) => {
                        return;
                    }
                };
                let new_board = Board::new(height, width, mines);
                match new_board {
                    Ok(board) => self.board = board,
                    Err(err) => {
                        self.error = Some(err.to_string());
                        return;
                    }
                };
                frame.set_window_size(Self::calculate_size(height, width));
                self.settings_modal_opened = false;
                self.error = None;
            }
            if ui.button("Quit").clicked() {
                self.settings_modal_opened = false;
            }
        });
    }

    fn draw_cell(&mut self, ui: &mut Ui, x: u8, y: u8) {
        let cell = self.board.get_visible_cell(x, y);

        let cell_button = ui.add_sized(
            [constants::CELL_SIZE, constants::CELL_SIZE],
            egui::Button::new(Self::get_cell_text(&cell))
                .text_color(Self::get_cell_text_color(&cell))
                .text_style(TextStyle::Heading)
                .fill(Color32::WHITE)
                .enabled(match &cell {
                    VisibleCell::Uncovered(_) => false,
                    _ => true,
                }),
        );

        if cell_button.clicked() {
            self.board.uncover_cell(x, y);
        } else if cell_button.secondary_clicked() {
            self.board.flag_cell(x, y);
        }
    }

    fn get_cell_text(visible_cell: &VisibleCell) -> char {
        match visible_cell {
            VisibleCell::Uncovered(cell) => match cell {
                Cell::Mine => '💥',
                Cell::Clue(number) => char::from_digit(*number as u32, 10).unwrap(),
                Cell::Empty => ' ',
            },
            VisibleCell::Covered => ' ',
            VisibleCell::Flagged => '🚩',
        }
    }

    fn get_cell_text_color(visible_cell: &VisibleCell) -> Color32 {
        match visible_cell {
            VisibleCell::Uncovered(cell) => match cell {
                Cell::Mine => constants::RED,
                Cell::Clue(number) => match number {
                    1 => constants::BLUE,
                    2 => constants::GREEN,
                    3 => constants::RED,
                    4 => constants::DARK_BLUE,
                    5 => constants::DARK_RED,
                    6 => constants::PERSIAN_GREEN,
                    7 => constants::BLACK,
                    8 => constants::GREY,
                    _ => constants::BLACK,
                },
                Cell::Empty => constants::BLACK,
            },
            VisibleCell::Covered => constants::BLACK,
            VisibleCell::Flagged => constants::RED,
        }
    }
}

impl Default for MinesweeperApp {
    fn default() -> Self {
        Self {
            board: Board::new(
                constants::DEFAULT_BOARD_HEIGHT,
                constants::DEFAULT_BOARD_WIDTH,
                constants::DEFAULT_BOARD_MINES,
            )
            .unwrap(),
            error: None,
            settings_modal_opened: false,
            height_input: format!("{}", constants::DEFAULT_BOARD_HEIGHT),
            width_input: format!("{}", constants::DEFAULT_BOARD_WIDTH),
            mines_input: format!("{}", constants::DEFAULT_BOARD_MINES),
        }
    }
}

impl epi::App for MinesweeperApp {
    fn name(&self) -> &str {
        "Minesweeper"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        self.draw_top_menu(ctx, frame);
        self.draw_board_panel(ctx);
        match self.board.state.clone() {
            State::Finished(finished_state) => self.draw_end_of_game_modal(ctx, finished_state),
            _ => {}
        };
        if self.settings_modal_opened {
            self.draw_settings_modal(ctx, frame);
        }
    }
}

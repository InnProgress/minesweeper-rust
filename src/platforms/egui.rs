use crate::{
    board::{Board, FinishedState, State},
    tile::{PublicTile, Tile},
};
use eframe::{
    egui::{self, Color32},
    epi,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    board: Board,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            board: Board::new(9, 9, 10),
        }
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "Minesweeper"
    }

    fn setup(
        &mut self,
        _ctx: &egui::CtxRef,
        _frame: &mut epi::Frame<'_>,
        _storage: Option<&dyn epi::Storage>,
    ) {
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        #[cfg(feature = "persistence")]
        if let Some(storage) = _storage {
            *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
        }
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Restart").clicked() {
                        self.board.restart();
                    }
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(egui::Label::new(format!("Mines: {}", self.board.mines)));

            for y in 0..self.board.height {
                ui.horizontal_wrapped(|ui| {
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.spacing_mut().item_spacing.y = 0.0;
                    for x in 0..self.board.width {
                        let tile_button = ui.add_sized(
                            [25., 25.],
                            egui::Button::new(match &self.board.visible[y as usize][x as usize] {
                                PublicTile::Visible(inner_tile) => match inner_tile {
                                    Tile::Mine => String::from("💥"),
                                    Tile::Tip(number) => format!("{}", number),
                                    Tile::Empty => String::from(" "),
                                },
                                PublicTile::Hidden => String::from(" "),
                                PublicTile::Mine => String::from("🚩"),
                            })
                            .enabled(match &self.board.visible[y as usize][x as usize] {
                                PublicTile::Visible(_) => false,
                                _ => true,
                            })
                            .text_color(match &self.board.visible[y as usize][x as usize] {
                                PublicTile::Visible(inner_tile) => match inner_tile {
                                    Tile::Mine => Color32::RED,
                                    Tile::Tip(number) => match number {
                                        1 => Color32::from_rgb(0, 0, 253),
                                        2 => Color32::from_rgb(1, 126, 0),
                                        3 => Color32::from_rgb(254, 0, 0),
                                        4 => Color32::from_rgb(1, 1, 128),
                                        5 => Color32::from_rgb(126, 3, 3),
                                        6 => Color32::from_rgb(0, 128, 128),
                                        7 => Color32::from_rgb(0, 0, 0),
                                        8 => Color32::from_rgb(128, 128, 128),
                                        _ => Color32::BLACK,
                                    },
                                    Tile::Empty => Color32::BLACK,
                                },
                                PublicTile::Hidden => Color32::BLACK,
                                PublicTile::Mine => Color32::RED,
                            })
                            .fill(Color32::WHITE)
                            .wrap(true),
                        );
                        if tile_button.clicked() {
                            self.board.capture(x, y);
                        } else if tile_button.secondary_clicked() {
                            self.board.capture_mine(x, y);
                        }
                    }
                });
            }
        });

        match &self.board.state {
            State::Finished(finished_state) => {
                match finished_state {
                    FinishedState::Won => {
                        egui::Window::new("Win").show(ctx, |ui| {
                            ui.label("You have won!");
                            if ui.button("Start new game").clicked() {
                                self.board.restart();
                            }
                        });
                    }
                    FinishedState::Lost => {
                        egui::Window::new("Lose").show(ctx, |ui| {
                            ui.label("You have lost!");
                            if ui.button("Start new game").clicked() {
                                self.board.restart();
                            }
                        });
                    }
                };
            }
            _ => {}
        }
    }
}

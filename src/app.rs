use egui::{TextStyle, WidgetText, Color32};

use crate::Board;

pub struct App {
    board: Board,
}
impl App {
    pub fn new(board: Board) -> Self {
        Self { board }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Time for the Sweepster");
            egui_extras::TableBuilder::new(ui)
                .columns(egui_extras::Column::auto(), self.board.width())
                .body(|body| {
                    let mut r = 0;
                    body.rows(1.0, self.board.height(), |mut row| {
                        for c in 0..self.board.width() {
                            row.col(|ui| {
                                let c = (r, c);
                                let cell = cell_ui(ui, &mut self.board, c);
                                if cell.clicked() {
                                    self.board.expose(c);
                                }
                                if cell.secondary_clicked() {
                                    self.board.plant_bomb(c);
                                }
                            });
                        }
                        r += 1;
                    });
                });
        });
    }
}

fn cell_ui(ui: &mut egui::Ui, board: &mut Board, c: (usize, usize)) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(1.0, 1.0);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        response.mark_changed();
    }

    let color = match board.get_cell(c).state {
        crate::board::CellState::Covered => Color32::BLUE,
        crate::board::CellState::Empty => Color32::TRANSPARENT,
        crate::board::CellState::Flagged => Color32::GOLD,
        crate::board::CellState::Detonated => Color32::RED
    };

    if ui.is_rect_visible(rect) {
        let visuals = ui
            .style()
            .interact(&response);
        ui.painter()
            .rect(rect, 2.0, color, visuals.fg_stroke);
        if !(board.get_cell(c).is_covered() || board.get_cell(c).value == 0) {
            let visuals = ui.style().noninteractive();
            let galley = WidgetText::from(format!("{}", board.get_cell(c).value)).into_galley(
                ui,
                None,
                rect.size().y,
                TextStyle::Button,
            );
            ui.painter()
                .galley(rect.center_top(), galley, visuals.text_color());
        }
    }

    response
}

use crate::{Board, color};
use egui::{Color32, Grid, ScrollArea, Slider, TextStyle, WidgetText};
use log::info;

const CELL_SIZE: f32 = 1.5;

pub struct App {
    board: Board,
    bombs: usize,
    height: usize,
    width: usize,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
            let visuals = egui::Visuals {
            override_text_color: None,
            hyperlink_color: color::IRIS,
            faint_bg_color: color::SURFACE, // Table stripes
            extreme_bg_color: color::HIGHLIGHT_LOW,
            code_bg_color: color::HIGHLIGHT_MED,
            warn_fg_color: color::GOLD,
            error_fg_color: color::LOVE,
            window_fill: color::OVERLAY, // Widget background
            panel_fill: color::BASE,     // Background background
            widgets: egui::style::Widgets {
                noninteractive: egui::style::WidgetVisuals {
                    bg_fill: color::SURFACE,
                    weak_bg_fill: color::SURFACE,
                    bg_stroke: egui::Stroke::new(1.0, color::HIGHLIGHT_MED), // Separator color
                    rounding: egui::Rounding::same(4.0),
                    fg_stroke: egui::Stroke::new(1.0, color::TEXT),
                    expansion: 1.0,
                },
                inactive: egui::style::WidgetVisuals {
                    bg_fill: color::MUTED,
                    weak_bg_fill: color::MUTED,
                    bg_stroke: egui::Stroke::new(1.0, color::OVERLAY),
                    rounding: egui::Rounding::same(4.0),
                    fg_stroke: egui::Stroke::new(1.0, color::TEXT),
                    expansion: 1.0,
                },
                hovered: egui::style::WidgetVisuals {
                    bg_fill: color::MUTED,
                    weak_bg_fill: color::MUTED,
                    bg_stroke: egui::Stroke::new(1.0, color::MUTED),
                    rounding: egui::Rounding::same(4.0),
                    fg_stroke: egui::Stroke::new(1.0, color::TEXT),
                    expansion: 1.0,
                },
                active: egui::style::WidgetVisuals {
                    bg_fill: color::SUBTLE,
                    weak_bg_fill: color::SUBTLE,
                    bg_stroke: egui::Stroke::new(1.0, color::SUBTLE),
                    rounding: egui::Rounding::same(4.0),
                    fg_stroke: egui::Stroke::new(1.0, color::TEXT),
                    expansion: 1.0,
                },
                open: egui::style::WidgetVisuals {
                    bg_fill: color::SUBTLE,
                    weak_bg_fill: color::SUBTLE,
                    bg_stroke: egui::Stroke::new(1.0, color::MUTED),
                    rounding: egui::Rounding::same(4.0),
                    fg_stroke: egui::Stroke::new(1.0, color::TEXT),
                    expansion: 1.0,
                },
            },
            selection: egui::style::Selection {
                bg_fill: color::PINE,
                stroke: egui::Stroke::new(1.0, color::TEXT),
            },
            ..egui::Visuals::default()
        };
        cc.egui_ctx.set_visuals(visuals);

        Self {
            board: Board::new(32, 32, 50),
            bombs: 50,
            height: 32,
            width: 32,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("Time for the Sweepster");
                    ui.add(
                        Slider::new(&mut self.bombs, 50..=(self.height * self.width / 2))
                            .text("Bombs"),
                    );
                    ui.add(Slider::new(&mut self.height, 16..=128).text("Height"));
                    ui.add(Slider::new(&mut self.width, 16..=128).text("Width"));
                    if ui.button("Gen board").clicked() {
                        self.board = Board::new(self.height, self.width, self.bombs);
                        ctx.request_repaint();
                    }
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("Assists");
                    ui.toggle_value(&mut self.board.auto_flag, "Auto plant flags");
                })
            });

            ScrollArea::vertical().show(ui, |ui| {
                Grid::new("board_grid")
                    .spacing(egui::vec2(0.0, 0.0))
                    .max_col_width(CELL_SIZE)
                    .min_col_width(CELL_SIZE)
                    .min_row_height(CELL_SIZE)
                    .show(ui, |ui| {
                        for h in 0..self.board.rows() {
                            for w in 0..self.board.columns() {
                                let c = (h, w);
                                let cell = cell_ui(ui, &mut self.board, c);
                                if cell.clicked() {
                                    info!("Clicked {c:?}");
                                    self.board.expose(c);
                                }
                                if cell.secondary_clicked() {
                                    info!("Flagged {c:?}");
                                    self.board.toggle_bomb(c);
                                }
                            }
                            ui.end_row();
                        }
                    });
            });
        });
    }
}

fn cell_ui(ui: &mut egui::Ui, board: &mut Board, c: (usize, usize)) -> egui::Response {
    let desired_size = ui.spacing().interact_size.y * egui::vec2(CELL_SIZE, CELL_SIZE);
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    if response.clicked() {
        response.mark_changed();
    }

    let color = match board.get_cell(c).state {
        crate::board::CellState::Covered => color::OVERLAY,
        crate::board::CellState::Empty => Color32::TRANSPARENT,
        crate::board::CellState::Flagged => color::PINE,
        crate::board::CellState::Detonated => color::LOVE,
    };
    if ui.is_rect_visible(rect) {
        let visuals = ui.style().interact(&response);
        ui.painter().rect(rect, 0.0, color, visuals.bg_stroke);

        if board.get_cell(c).is_empty() && board.get_cell(c).value != 0 {
            let visuals = ui.style().noninteractive();
            let text_color = if board.is_cell_satsfied(c) {
                color::MUTED
            } else {
                visuals.text_color()
            };
            let cell = board.get_cell(c);
            let galley = WidgetText::from(format!("{}", cell.value)).into_galley(
                ui,
                None,
                rect.size().y,
                TextStyle::Heading,
            );
            let text_pos = ui.layout().align_size_within_rect(galley.size(), rect).min;

            ui.painter().galley(text_pos, galley, text_color);
        }
    }

    response
}

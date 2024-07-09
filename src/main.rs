use board::Board;

mod color;
mod app;
mod board;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();
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
        // cc.egui_ctx.set_visuals(visuals);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };
    eframe::run_native(
        "Sweepster",
        options,
        Box::new(|cc| Ok(Box::new(app::App::new(cc)))),
    )?;
    Ok(())
}

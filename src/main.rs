use board::Board;

mod color;
mod app;
mod board;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

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

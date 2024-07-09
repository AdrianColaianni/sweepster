use app::App;
use board::Board;

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
        Box::new(|_| Ok(Box::<app::App>::default())),
    )?;
    Ok(())
}

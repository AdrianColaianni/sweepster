use app::App;
use board::Board;

mod app;
mod board;

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Sweepster",
        options,
        Box::new(|_| Ok(Box::<app::App>::new(App::new(Board::new(32, 32, 50))))),
    )?;
    Ok(())
}

use anyhow::Result;
use eframe::NativeOptions;
use egui::ViewportBuilder;

mod interface;
use interface::gui::SonusApp;

fn main() -> Result<()> {
    let options = NativeOptions {
        viewport: ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Sonus",
        options,
        Box::new(|cc| Box::new(SonusApp::new(cc))),
    ).map_err(|err| anyhow::anyhow!("Failed to start Sonus: {}", err))?;

    Ok(())
}
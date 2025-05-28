use anyhow::Result;
use eframe::{App, Frame, NativeOptions, egui};
use env_logger;
use eframe::icon_data::from_png_bytes;

use sonus::interface::gui::SonusApp;

struct SplashApp {
    start_time: std::time::Instant,
}

impl SplashApp {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
        }
    }
}

impl App for SplashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(ui.available_height() * 0.3);
                ui.label(egui::RichText::new("Sonus").heading().size(32.0));
                ui.add_space(20.0);
                ui.spinner().on_hover_text("Loading...");
                ui.add_space(10.0);
                ui.label("Initializing audio engine...");
                ui.add_space(ui.available_height() * 0.3);
            });
        });
        if self.start_time.elapsed().as_secs() >= 9 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let icon_bytes: &'static [u8] = include_bytes!("assets/icons/logo_x64.png");

    let splash_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([270.0, 400.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_icon(from_png_bytes(icon_bytes).unwrap())
            .with_title("Sonus - Loading"),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    eframe::run_native(
        "Sonus - Splash",
        splash_options,
        Box::new(|_| Box::new(SplashApp::new())),
    )?;

    let main_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 720.0])
            .with_resizable(true)
            .with_decorations(true)
            .with_icon(from_png_bytes(icon_bytes).unwrap())
            .with_title("Sonus"),
        default_theme: eframe::Theme::Dark,
        ..Default::default()
    };

    eframe::run_native(
        "Sonus",
        main_options,
        Box::new(|cc| Box::new(SonusApp::new(cc))),
    )
}

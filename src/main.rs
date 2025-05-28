use anyhow::Result;
use eframe::icon_data::from_png_bytes;
use eframe::{App, Frame, NativeOptions, egui};
use env_logger;

use sonus::interface::gui::SonusApp;

struct SplashApp {
    start_time: std::time::Instant,
    logo_texture: Option<egui::TextureHandle>,
}

impl SplashApp {
    fn new() -> Self {
        Self {
            start_time: std::time::Instant::now(),
            logo_texture: None,
        }
    }
}

impl App for SplashApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        if let Some(cmd) = egui::ViewportCommand::center_on_screen(ctx) {
            ctx.send_viewport_cmd(cmd);
        }

        const TEXT_LOGO_BYTES: &[u8] = include_bytes!("assets/icons/text_logo.png");
        if self.logo_texture.is_none() {
            if let Ok(image) = image::load_from_memory(TEXT_LOGO_BYTES) {
                let image = image.to_rgba8();
                let size = [image.width() as usize, image.height() as usize];
                let pixels = image.into_raw();
                let texture = ctx.load_texture(
                    "logo",
                    egui::ColorImage::from_rgba_unmultiplied(size, &pixels),
                    egui::TextureOptions::default(),
                );
                self.logo_texture = Some(texture);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered_justified(|ui| {
                ui.add_space(ui.available_height() * 0.3);
                if let Some(texture) = &self.logo_texture {
                    ui.add(
                        egui::Image::from_texture(texture)
                            .max_size(ui.available_size() * 0.8),
                    );
                }
                ui.add_space(20.0);
                ui.spinner();
                ui.add_space(10.0);
                ui.label("Initializing audio engine...");
                ui.add_space(ui.available_height() * 0.3);
            });
        });
        if self.start_time.elapsed().as_secs() >= 90 {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }
    }
}

fn main() -> Result<(), eframe::Error> {
    env_logger::init();

    let icon_bytes: &'static [u8] = include_bytes!("assets/icons/logo_x64.png");

    let splash_options = NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([320.0, 400.0])
            .with_resizable(false)
            .with_decorations(false)
            .with_drag_and_drop(true)
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

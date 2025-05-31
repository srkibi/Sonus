use eframe::{egui};
use egui::{Color32};

struct Sonus {
    progress: f32,
}

impl Default for Sonus {
    fn default() -> Self {
        Self { progress: 0.3 }
    }
}

impl eframe::App for Sonus {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(
                egui::Layout::bottom_up(egui::Align::BOTTOM),
                    |ui| {
                        ui.add(
                            egui::ProgressBar::new(self.progress)
                            .fill(Color32::from_rgb(255, 255, 255))
                            .desired_width(350.0)
                            .desired_height(10.0)
                            .corner_radius(0.0),
                        )
                    }
            )
        });
    }
}

fn main() -> eframe::Result<()> {
    let icon_bytes = include_bytes!("assets/icons/logo_x64.png");
    let image = image::load_from_memory(icon_bytes).expect("Failed to load icon");
    let image = image.to_rgba8();
    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    let options = eframe::NativeOptions {
        viewport: {
            // Create IconData from the loaded image
            let icon_data = egui::IconData {
                rgba: rgba.clone(),
                width: width as u32,
                height: height as u32,
            };
            egui::ViewportBuilder::default()
                .with_title("Sonus")
                .with_icon(icon_data)
                .with_inner_size([350.0, 400.0])
                .with_title_shown(false)
                .with_resizable(false)
                .with_decorations(false)
                .with_taskbar(false)
        },
        ..Default::default()
    };

    eframe::run_native(
        "Sonus",
        options,
        Box::new(|_cc| {
            Ok(Box::<Sonus>::default())
        }),
    )
    .map_err(|e| e.into())
}

use eframe::{App, CreationContext, Frame};
use egui::{CentralPanel, Context, TopBottomPanel};
use crate::interface::settings::SettingsDialog;

pub struct SonusApp {
    settings_dialog: SettingsDialog,
}

impl SonusApp {
    pub fn new(_cc: &CreationContext) -> Self {
        Self {
            settings_dialog: SettingsDialog::default(),
        }
    }
}

impl App for SonusApp {
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        TopBottomPanel::top("menu_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Project").clicked() {}
                    if ui.button("Open Project").clicked() {}
                    if ui.button("Save Project").clicked() {}
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });

                ui.menu_button("Edit", |ui| {
                    ui.add_enabled(false, egui::Button::new("Undo"));
                    ui.add_enabled(false, egui::Button::new("Redo"));
                    if ui.button("Settings").clicked() {
                        self.settings_dialog.open = true;
                    }
                });

                ui.menu_button("View", |ui| {
                    ui.add_enabled(false, egui::Button::new("Mixer"));
                    ui.add_enabled(false, egui::Button::new("Piano Roll"));
                });

                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {}
                });
            });
        });

        self.settings_dialog.show(ctx);

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|_ui| {
                
            });
        });
    }
}

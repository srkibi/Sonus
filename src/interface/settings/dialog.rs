use egui::{ComboBox, Layout, RichText, ScrollArea, Ui, Vec2, Window};
use futures_executor;
use super::{Settings, SettingsCategory};
use crate::{
    update::UpdateChecker,
    audio::{AudioSystem, device::AudioDevice},
};

pub struct SettingsDialog {
    pub open: bool,
    selected_category: SettingsCategory,
    settings: Settings,
    update_checker: UpdateChecker,
    audio_system: Option<AudioSystem>,
    input_devices: Vec<AudioDevice>,
    output_devices: Vec<AudioDevice>,
}

impl SettingsDialog {
    pub fn new() -> Self {
        let mut settings = Settings::load();
        let audio_system = AudioSystem::new().ok();

        let (input_devices, output_devices) = if let Some(audio_system) = &audio_system {
            audio_system.get_available_devices()
        } else {
            (Vec::new(), Vec::new())
        };

        if settings.selected_input_device.is_none() || settings.selected_output_device.is_none() {
            if let Some(audio_system) = &audio_system {
                let (default_input, default_output) = audio_system.get_default_devices();

                if settings.selected_input_device.is_none() {
                    if let Some(default_input) = default_input {
                        for (i, device) in input_devices.iter().enumerate() {
                            if device.name() == default_input.name() {
                                settings.selected_input_device = Some(i);
                                break;
                            }
                        }
                    }
                }

                if settings.selected_output_device.is_none() {
                    if let Some(default_output) = default_output {
                        for (i, device) in output_devices.iter().enumerate() {
                            if device.name() == default_output.name() {
                                settings.selected_output_device = Some(i);
                                break;
                            }
                        }
                    }
                }
            }
        }

        Self {
            open: false,
            selected_category: SettingsCategory::Updates,
            settings,
            update_checker: UpdateChecker::new(),
            audio_system,
            input_devices,
            output_devices,
        }
    }
}

impl Default for SettingsDialog {
    fn default() -> Self {
        Self::new()
    }
}

impl SettingsDialog {
    pub fn show(&mut self, ctx: &egui::Context) {
        if !self.open {
            return;
        }

        Window::new("Settings")
            .resizable(true)
            .collapsible(false)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(RichText::new("Settings").heading());
                        ui.with_layout(Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("❌").clicked() {
                                self.open = false;
                            }
                        });
                    });

                    ui.separator();

                    ui.horizontal(|ui| {
                        ui.allocate_ui_with_layout(
                            Vec2::new(60.0, ui.available_height()),
                            egui::Layout::top_down(egui::Align::Min),
                            |ui| {
                                self.show_categories(ui);
                            },
                        ); // Sidebar

                        ui.separator();

                        ui.vertical(|ui| {
                            self.show_settings_panel(ui);
                        })
                        .response.rect.set_width(ui.available_width());
                    });
                });
            });
    }

    fn show_categories(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().id_source("scroll_categories").show(ui, |ui| {
            ui.vertical_centered(|ui| {
                if ui.selectable_label(self.selected_category == SettingsCategory::Updates, "Updates").clicked() {
                    self.selected_category = SettingsCategory::Updates;
                }
                if ui.selectable_label(self.selected_category == SettingsCategory::Audio, "Audio").clicked() {
                    self.selected_category = SettingsCategory::Audio;
                }
            });
        });
    }

    fn show_settings_panel(&mut self, ui: &mut Ui) {
        ScrollArea::vertical().id_source("scroll_settings_panel").show(ui, |ui| {
            match self.selected_category {
                SettingsCategory::Audio => self.show_audio_panel(ui),
                SettingsCategory::Updates => self.show_updates_panel(ui),
            }
        });
    }

    fn show_updates_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Updates");
            ui.checkbox(&mut self.settings.auto_update, "Enable automatic updates");

            if ui.button("Check for Updates").clicked() {
                let update_checker = self.update_checker.clone();
                match futures_executor::block_on(update_checker.check_for_updates()) {
                    Ok(status) => {
                        if status.update_available {
                            self.settings.update_status = Some(format!(
                                "Update available! Current version: {}, Latest version: {}",
                                status.current_version,
                                status.latest_version.unwrap()
                            ));
                        } else {
                            self.settings.update_status = Some("You're running the latest version!".to_string());
                        }
                    }
                    Err(e) => {
                        self.settings.update_status = Some(format!("Failed to check for updates: {}", e));
                    }
                }
            }

            if let Some(status) = &self.settings.update_status {
                ui.label(status);
            }
        });
    }

    fn show_audio_panel(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Audio Settings");

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    if self.audio_system.is_some() {
                        ui.label("Audio Engine: Active");
                    } else {
                        ui.label("Audio Engine: Failed to initialize");
                        if ui.button("Retry").clicked() {
                            self.audio_system = AudioSystem::new().ok();
                            if let Some(audio_system) = &self.audio_system {
                                let (input_devices, output_devices) = audio_system.get_available_devices();
                                self.input_devices = input_devices;
                                self.output_devices = output_devices;
                            }
                        }
                        return;
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Input Device:");
                    ComboBox::from_id_source("input_device_selector_audio")
                        .selected_text(match &self.settings.selected_input_device {
                            Some(idx) => {
                                if *idx < self.input_devices.len() {
                                    self.input_devices[*idx].name()
                                } else {
                                    "Select Input Device"
                                }
                            }
                            None => "No Input Device Selected",
                        })
                        .show_ui(ui, |ui| {
                            for (i, device) in self.input_devices.iter().enumerate() {
                                if ui.selectable_label(self.settings.selected_input_device == Some(i), device.name()).clicked() {
                                    self.settings.selected_input_device = Some(i);
                                }
                            }
                        });
                });
            });

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Output Device:");
                    ComboBox::from_id_source("output_device_selector_audio")
                        .selected_text(match &self.settings.selected_output_device {
                            Some(idx) => {
                                if *idx < self.output_devices.len() {
                                    self.output_devices[*idx].name()
                                } else {
                                    "Select Output Device"
                                }
                            }
                            None => "No Output Device Selected",
                        })
                        .show_ui(ui, |ui| {
                            for (i, device) in self.output_devices.iter().enumerate() {
                                if ui.selectable_label(self.settings.selected_output_device == Some(i), device.name()).clicked() {
                                    self.settings.selected_output_device = Some(i);
                                }
                            }
                        });
                });
            });

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Buffer Size:");
                    for size in [64, 128, 256, 512, 1024] {
                        if ui.selectable_label(self.settings.buffer_size == size, size.to_string()).clicked() {
                            self.settings.buffer_size = size;
                        }
                    }
                });
            });

            ui.group(|ui| {
                ui.vertical_centered(|ui| {
                    ui.label("Sample Rate:");
                    for rate in [44100, 48000, 96000] {
                        if ui.selectable_label(self.settings.sample_rate == rate, format!("{} Hz", rate)).clicked() {
                            self.settings.sample_rate = rate;
                        }
                    }
                });
            });

            ui.add_space(10.0);
            if ui.button("Apply Settings").clicked() {
                if let Err(err) = self.settings.save() {
                    log::error!("Failed to save settings: {}", err);
                    ui.label("Failed to save settings");
                } else {
                    ui.label("Settings saved successfully. Restart may be required for some changes to take effect.");
                }
            }
        });
    }
}

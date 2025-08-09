use eframe::egui;
use cpal::{traits::{DeviceTrait, HostTrait}};

#[derive(Default)]
pub struct MyApp {
    input_devices: Vec<String>,
    selected_device: Option<String>,
    show_device_list: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Audio Input Selector");

            // Button to trigger device listing
            if ui.button("List Input Devices").clicked() {
                self.input_devices.clear(); // Reset list
                let host = cpal::default_host();
                if let Ok(inputs) = host.input_devices() {
                    self.input_devices = inputs.map(|d| d.name().unwrap_or_default()).collect();
                    self.show_device_list = true;
                }
            }

            // Show buttons for each device
            if self.show_device_list {
                ui.label("Available Devices:");
                for name in &self.input_devices {
                    if ui.button(name).clicked() {
                        self.selected_device = Some(name.clone());
                        self.show_device_list = false; // hide after selection
                    }
                }
            }

            // Show current selection
            if let Some(selected) = &self.selected_device {
                ui.separator();
                ui.label(format!("Selected device: {}", selected));
            }
        });
    }
}


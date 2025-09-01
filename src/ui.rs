use eframe::egui;
use std::sync::Arc;

use crate::{audio_stream::AudioStream, effects::AtomicEffects};

pub struct MyApp {
    volume_value: u32,
    atomic_effects: Arc<AtomicEffects>,
    audio_streamer: AudioStream
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui: &mut egui::Ui| {

            // volume left (obviously)
            egui::SidePanel::left("Volume panel").show_inside(ui, |ui| {
                ui.heading("Volume control");
                let volume_slider = ui.add(egui::Slider::new(&mut self.volume_value, 0..=100));
                if volume_slider.dragged() {
                    self.atomic_effects.set_volume(self.volume_value);
                }
            });

        });
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let ae = Arc::new(AtomicEffects::new());
        Self {
            volume_value: 0,
            atomic_effects: ae.clone(),
            audio_streamer: AudioStream::with_effects(ae.clone())
        }
    }
}
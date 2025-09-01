mod audio_stream;
mod ui;
mod effects;

use crate::audio_stream::AudioStream;
use crate::effects::AtomicEffects;
use std::sync::Arc;
use crate::ui::MyApp;
use eframe::egui;

fn main() {

    let effects = Arc::new(AtomicEffects::new());
    let mut streamer = AudioStream::with_effects(effects.clone());

    //streamer.select_devices();

    println!("{streamer}");

    //streamer.dispatch();

    // std::thread::park(); // uncomment if no ui

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "Rust Guitar", 
        options, 
        Box::new(|_cc| Ok(Box::<MyApp>::default())), 
    );

}


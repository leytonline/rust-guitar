mod audio_stream;
mod ui;

use crate::audio_stream::AudioStream;
use crate::ui::MyApp;
use eframe::egui;

fn main() {

    let mut streamer = AudioStream::new();
    let _ = streamer;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    let w = egui::Window::new("window")
        .default_width(600.0)
        .default_height(400.0)
        .vscroll(false)
        .open(&mut true);
    
    let _ = eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| {
            Ok(Box::<MyApp>::default())
        }),
    );
}


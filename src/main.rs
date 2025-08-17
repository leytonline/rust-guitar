mod audio_stream;
mod ui;

use crate::audio_stream::AudioStream;
use crate::ui::MyApp;
use eframe::egui;

fn main() {

    let mut streamer = AudioStream::new();

    streamer.select_devices();

    println!("{streamer}");

    streamer.dispatch();

    std::thread::park();

}


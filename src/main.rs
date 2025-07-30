mod audio_stream;

use crate::audio_stream::AudioStream;

fn main() {
    let mut streamer: AudioStream = AudioStream::new();
    streamer.select_devices();
    println!("{streamer}");
    match streamer.dispatch() {
        Ok(_) => println!("Beginning stream..."),
        Err(msg) => panic!("Dispatch error: {msg}"),
    }

    std::thread::park();
}


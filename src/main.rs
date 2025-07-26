use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use std::sync::{Arc, Mutex};
use std::io::{self};

fn main() {
    let host = cpal::default_host();

    let devices: Vec<_> = host.input_devices().unwrap().collect();
    println!("Choose input device from list:");
    devices.iter().enumerate().for_each(|(i, d)| {
        println!("{}. {}", i, d.name().unwrap());
    });

    let mut input: String = String::new();
    print!("Select input device: ");
    io::stdin().read_line(&mut input).unwrap();
    let idx: usize = input.trim().parse().unwrap();
    let selected_input = devices.get(idx).unwrap();

    // Pick default devices
    let input_device = selected_input;
    let output_device = host
        .default_output_device()
        .expect("No default output device");

    println!("Using input: {}", input_device.name().unwrap());
    println!("Using output: {}", output_device.name().unwrap());

    // Match sample formats
    let input_config = input_device.default_input_config().unwrap();
    let output_config = output_device.default_output_config().unwrap();

    let config = input_config.config(); 

    assert_eq!(input_config.sample_format(), cpal::SampleFormat::F32);
    assert_eq!(output_config.sample_format(), cpal::SampleFormat::F32);

    let buffer = Arc::new(Mutex::new(vec![0.0f32; config.channels as usize * 512]));

    let input_buffer = buffer.clone();
    let input_stream = input_device
        .build_input_stream(
            &config,
            move |data: &[f32], _| {
                let mut shared = input_buffer.lock().unwrap();
                for (i, sample) in data.iter().enumerate() {
                    shared[i] = *sample;
                }
            },
            err_fn,
            None,
        )
        .unwrap();

    let output_buffer = buffer.clone();
    let output_stream = output_device
        .build_output_stream(
            &config,
            move |data: &mut [f32], _| {
                let shared = output_buffer.lock().unwrap();
                for (i, sample) in data.iter_mut().enumerate() {
                    *sample = shared[i];
                }
            },
            err_fn,
            None,
        )
        .unwrap();

    input_stream.play().unwrap();
    output_stream.play().unwrap();

    std::thread::park();
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("Stream error: {}", err);
}

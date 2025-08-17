use cpal::{traits::{DeviceTrait, HostTrait, StreamTrait}};
use std::io::{self};
use std::sync::{Arc, Mutex};
use std::{fmt};

pub struct AudioStream {
    host: cpal::Host,
    input_device: Option<cpal::Device>,
    output_device: Option<cpal::Device>,
    input_stream: Option<cpal::Stream>,
    output_stream: Option<cpal::Stream>,
    input_buffer: Option<Arc<Mutex<Vec<f32>>>>
}

impl AudioStream {
    pub fn new() -> Self {
        Self {
            host: cpal::default_host(),
            input_device: None,
            output_device: None,
            input_stream: None,
            output_stream: None, 
            input_buffer: None,
        }
    }

    pub fn dispatch(&mut self) -> Result<(), String> {
        if self.input_device.is_none() {
            return Err(String::from("no input device"));
        }

        if self.output_device.is_none() {
            return Err(String::from("no output device"));
        }

        let input_config = self.input_device.as_ref().unwrap().default_input_config().unwrap();
        let output_config = self.output_device.as_ref().unwrap().default_output_config().unwrap();

        let mut config = input_config.config(); 

        assert_eq!(input_config.sample_format(), cpal::SampleFormat::F32);
        assert_eq!(output_config.sample_format(), cpal::SampleFormat::F32);

        // arbitrarily large buffer, won't need to change but not a huge fan
        let buffer = Arc::new(Mutex::new(vec![0.0f32; 4096]));
        self.input_buffer = Some(buffer.clone());

        let in_shared = self.input_buffer.as_ref().unwrap().clone();
        let out_shared = self.input_buffer.as_ref().unwrap().clone();

        self.input_stream = Some(self.input_device.as_ref().unwrap()
            .build_input_stream(
                &config,
                move |data: &[f32], _| {
                    let mut shared = in_shared.lock().unwrap();
                    // let length = data.len();
                    // println!("{length}");
                    for (i, sample) in data.iter().enumerate() {
                        let mut preclamped = *sample;
                        preclamped *= 5.0;
                        if preclamped > 0. {
                            preclamped = 1.0;
                        }
                        else {
                            preclamped = -1.0;
                        }
                        preclamped *= 0.2;
                        shared[i] = preclamped;
                    }
                },
                err_fn,
                None,
            )
            .unwrap());

        self.output_stream = Some(self.output_device.as_ref().unwrap()
            .build_output_stream(
                &config,
                move |data: &mut [f32], _| {
                    let shared = out_shared.lock().unwrap();
                    // let length = data.len();
                    // println!("{length}");
                    for (i, sample) in data.iter_mut().enumerate() {
                        *sample = shared[i];
                    }
                },
                err_fn,
                None,
            )
            .unwrap());

        
        self.input_stream.as_ref().unwrap().play().unwrap();
        self.output_stream.as_ref().unwrap().play().unwrap();

        Ok(())            
    }

    pub fn select_devices(&mut self) {
        // get input dev
        println!("Choose input device from below:");
        self.list_input_devices();
        print!("Enter choice: ");
        let mut choice = String::new();
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let idx: usize = choice.trim().parse().unwrap_or(0); // default 0 on nonsensical input (first dev, hope it exists)
        let inputs: Vec<cpal::Device> = self.host.input_devices().unwrap().collect();
        self.input_device = Some(inputs.get(idx).unwrap().clone());

        // get output dev
        println!("Choose output device from below:");
        self.list_output_devices();
        println!("Enter choice (-1 for no output): ");
        choice = String::new();
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let odx: i32 = choice.trim().parse().unwrap_or(-1);

        if odx == -1 {
            return;
        }

        let outputs: Vec<cpal::Device> = self.host.output_devices().unwrap().collect();
        self.output_device = Some(outputs.get(odx as usize).unwrap().clone());

        let ic = self.input_device.as_ref().unwrap().default_input_config().unwrap();
        let oc = self.output_device.as_ref().unwrap().default_output_config().unwrap();

        println!("{ic:?}\n");
        println!("{oc:?}");


    }

    fn list_input_devices(&self) {
        self.host.input_devices().unwrap().enumerate().for_each(|(i, d)| {
            println!("{i}. {}", d.name().unwrap());
        });
    }

    fn list_output_devices(&self) {
        self.host.output_devices().unwrap().enumerate().for_each(|(i, d)| {
            println!("{i}. {}", d.name().unwrap());
        });
    }
}

impl fmt::Display for AudioStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AudioStream(")?;
        match &self.input_device {
            Some(device) => match device.name() {
                Ok(name) => write!(f, "{name}")?,
                Err(_) => write!(f, "nameless input")?,
            },
            None => write!(f, "no input")?,
        };

        write!{f, ", "}?;
        match &self.output_device {
            Some(device) => match device.name() {
                Ok(name) => write!(f, "{name}")?,
                Err(_) => write!(f, "nameless output")?,
            },
            None => write!(f, "no output")?,
        };

        write!(f, ")")?;
        Ok(())
    }
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("Stream error: {}", err);
}

use cpal::{traits::HostTrait, traits::DeviceTrait};
use std::io::{self};
use std::fmt;

pub struct AudioStream {
    host: cpal::Host,
    input_device: Option<cpal::Device>,
    output_device: Option<cpal::Device>,
}

impl AudioStream {
    pub fn new() -> Self {
        Self {
            host: cpal::default_host(),
            input_device: None,
            output_device: None,
        }
    }

    pub fn select_devices(&mut self) {

        // get input dev
        self.list_input_devices();
        print!("Enter choice: ");
        let mut choice = String::new();
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let idx: usize = choice.trim().parse().unwrap();
        let inputs: Vec<cpal::Device> = self.host.input_devices().unwrap().collect();
        self.input_device = Some(inputs.get(idx).unwrap().clone());


        // get output dev
        self.list_output_devices();
        println!("Enter choice (-1 for no output): ");
        choice = String::new();
        io::Write::flush(&mut io::stdout()).unwrap();
        io::stdin().read_line(&mut choice).unwrap();
        let odx: i32 = choice.trim().parse().unwrap();

        if odx == -1 {
            return;
        }

        let outputs: Vec<cpal::Device> = self.host.output_devices().unwrap().collect();
        self.output_device = Some(outputs.get(odx as usize).unwrap().clone());
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
use std::fs::File;
use std::path::{Path, PathBuf};
use std::error::Error;

use cpal::traits::{DeviceTrait, HostTrait};
use cpal::Host;
use rodio::Device as RodioDevice;

use crate::sounds::signals::Signal;
use crate::experiment::experiment::{AudioSettings, Experiment, RunSettings};

impl Experiment {
    pub fn new() -> Experiment {
        let available_hosts = cpal::available_hosts();
        let output_path = PathBuf::from("./output/experiment_results");

        // Check if the file exists, if not create it
        if !Path::new(&output_path).exists() {
            File::create(&output_path).expect("Failed to create file");
        }

        Experiment {
            audio_settings: AudioSettings::new(),
            signal_vec: Vec::new(),
            output_path,
            run_settings: RunSettings {
                start_silence_time: 0.5,
                end_silence_time: 0.5,
                silence_between_signals: 0.1,
            },
        }
    }

    pub fn display_host_options(&self) {
        println!("Available hosts:");
        for device_option in &self.audio_settings.all_output_devices {
            println!("{}", device_option);
        }
    }

    pub fn set_device(&mut self, device_name: &str) {
        let host = cpal::default_host();
        let device = host.output_devices().unwrap()
            .find(|device| device.name().unwrap() == device_name)
            .expect("Device not found");

        self.audio_settings.output_device = Some(device);
    }
    pub fn add_signal(&mut self, signal: Signal) {
        self.signal_vec.push(signal);
    }

    pub fn set_start_silence_time(&mut self, time: f64) {
        self.run_settings.start_silence_time = time;
    }

    pub fn set_end_silence_time(&mut self, time: f64) {
        self.run_settings.end_silence_time = time;
    }

    pub fn set_silence_between_signals(&mut self, time: f64) {
        self.run_settings.silence_between_signals = time;
    }
}




impl AudioSettings {
    pub fn new() -> Self {
        // Get the default output device with rodio
        let output_device = rodio::default_output_device();

        // Get a list of all output devices with cpal
        let host = Host::default();
        let all_output_devices: Vec<String> = host.output_devices().unwrap()
            .map(|device| device.name().unwrap())
            .collect();

        Self {
            output_device,
            all_output_devices,
        }
    }
}
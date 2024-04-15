// Standard library imports
use std::fs::File;
use std::path::{Path, PathBuf};

// External crate imports
use cpal::traits::{DeviceTrait, HostTrait};

// Internal module imports
use crate::experiments::experiment::{AudioSettings, Experiment, RunSettings};
use crate::sounds::signals::Signal;

impl Experiment {
    pub fn new() -> Experiment {
 
        let output_path = PathBuf::from("./experiment_results");

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
        for device_option in &self.audio_settings.all_hosts {
            println!("{device_option:?}");
        }
    }

    pub fn display_device_options(&self) {
        println!("Available output devices:");
        for device_option in &self.audio_settings.all_devices {
            println!("{device_option:?}");
        }
    }

    pub fn set_device(&mut self, device_name: &str) {
        let host = cpal::default_host();
        let device = host.output_devices().unwrap()
            .find(|device| device.name().unwrap() == device_name)
            .expect("Device not found");

        self.audio_settings.selected_device = Some(device);
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
        // Get the default host with cpal
        let default_host = cpal::default_host();

        // Retrieve a list of all available hosts
        let hosts = cpal::available_hosts();

        // Collect names of all available audio hosts into a Vec<String>
        let all_hosts: Vec<String> = hosts.iter().map(|host_id| host_id.name().to_string()).collect();

        // Get the default output device with cpal
        let selected_device = default_host.default_output_device();

        // Retrieve a list of all available output devices
        let all_devices: Vec<String> = default_host.output_devices().unwrap().map(|device| device.name().unwrap()).collect();

        Self {
            all_hosts,
            selected_host: default_host,
            all_devices,
            selected_device,
        }
    }
}
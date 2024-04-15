use std::path::PathBuf;

use crate::sounds::signals::Signal;

pub struct Experiment {
    pub audio_settings: AudioSettings,
    pub signal_vec: Vec<Signal>,
    pub output_path: PathBuf,
    pub run_settings: RunSettings,
}

pub struct RunSettings {
    pub start_silence_time: f64,
    pub end_silence_time: f64,
    pub silence_between_signals: f64,
}

pub struct AudioSettings {
    pub output_device: Option<cpal::Device>,
    pub all_output_devices: Vec<String>,
}
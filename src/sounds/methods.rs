
use std::array;
use std::time::Duration;

use hound;

use ndarray::Array1;

use rodio::{Sink, Source};

use crate::experiment::experiment::Experiment; 
use crate::sounds::signals::Signal;

const PI: f64 = std::f64::consts::PI;

impl Signal {
    pub fn new_from_sine(frequency: f64, duration: f64, sample_rate: f64) -> Signal {
        let num_samples = (duration * sample_rate) as usize;
        let mut data = vec![0i16; num_samples];
        for (i, sample) in data.iter_mut().enumerate() {
            let sine_value = (2.0 * PI * frequency * i as f64 / sample_rate).sin();
            *sample = (sine_value * i16::MAX as f64) as i16;
        }
    
        let data = normalize(&mut data);
    
        let description = format!("Signal from sine wave: frequency = {frequency} Hz, duration = {duration} s, sample rate = {sample_rate} Hz");
    
        Signal {
            sample_rate,
            data,
            duration,
            description,
        }
    }

    fn new_from_wave(filename: &str) -> Signal {
        let mut reader = hound::WavReader::open(filename).unwrap();
        let mut samples: Vec<i16> = reader.samples().map(|x| x.unwrap()).collect();
        let data = normalize(&mut samples);

        Signal {
            sample_rate: reader.spec().sample_rate as f64,
            data,
            duration: samples.len() as f64 / reader.spec().sample_rate as f64,
            description: format!("Signal from wave file: {filename}"),
        }
    }

    pub fn play(&self, experiment: &Experiment) {
        let device = experiment.audio_settings.output_device.as_ref().expect("no output device available");

        let sink = Sink::try_new(device).unwrap();

        let source = rodio::buffer::SamplesBuffer::new(1, 44100, &self.data);

        sink.append(source);
        sink.sleep_until_end();
    }
}

fn normalize(data: &mut Vec<i16>) -> Vec<f32> {
    let max_value = data.iter().fold(0, |acc, &x| i16::max(acc, x.abs())) as f32;
    data.iter().map(|&sample| sample as f32 / max_value).collect()
}



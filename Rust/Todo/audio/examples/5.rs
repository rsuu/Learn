//! Records a WAV file (roughly 3 seconds long) using the default input device and config.
//!
//! The input data is recorded to "$CARGO_MANIFEST_DIR/recorded.wav".

extern crate anyhow;
extern crate cpal;
extern crate hound;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::sync::{Arc, Mutex};

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;
type WavR = Arc<Mutex<Option<hound::WavReader<BufReader<File>>>>>;

#[derive(Debug)]
struct Opt {
    device: String,
}

impl Opt {
    fn from_args() -> Self {
        let device = "lavrate".to_string();

        Opt { device }
    }
}

fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::from_args();
    let host = cpal::default_host();

    // Set up the input device and stream with the default input config.
    let device = if opt.device == "default" {
        host.default_input_device()
    } else {
        host.input_devices()?
            .find(|x| x.name().map(|y| y == opt.device).unwrap_or(false))
    }
    .expect("failed to find input device");

    println!("Input device: {}", device.name()?);

    let config = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Default input config: {:?}", config);

    // The WAV file we're recording to.
    //const PATH: &str = "./recorded.wav";
    //let spec = wav_spec_from_config(&config);
    //let writer = hound::WavWriter::create(PATH, spec)?;
    //let writer = Arc::new(Mutex::new(Some(writer)));
    //let writer_2 = writer.clone();
    //let rr = Arc::new(Mutex::new(Some(r)));

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into()),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into()),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into()),
    }
    .unwrap();

    Ok(())
}

pub fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig) -> Result<(), anyhow::Error>
where
    T: cpal::Sample,
{
    let mut r = hound::WavReader::open("./test.wav").unwrap();
    let rr = r.spec();

    let sample_rate = rr.sample_rate as f32;
    let channels = config.channels as usize;

    // Produce a sinusoid of maximum amplitude.
    let mut sample_clock = 2200 as f32;
    let mut next_value = move || {
        sample_clock = (sample_clock + 1.0) % sample_rate;
        (sample_clock * 440.0 * 2.0 * std::f32::consts::PI / sample_rate).sin()
    };

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            write_data(data, channels);
        },
        err_fn,
    )?;
    stream.play()?;

    std::thread::sleep(std::time::Duration::from_millis(2000));

    Ok(())
}

fn write_data<T>(output: &mut [T], channels: usize)
where
    T: cpal::Sample,
{
    let mut r = hound::WavReader::open("./test.wav").unwrap();
    let mut s: Vec<_> = r.samples::<i16>().map(|x| x.unwrap() as i16).collect();
    let mut i = 0;

    for frame in output.chunks_mut(channels) {
        for sample in frame.iter_mut() {
            let value: T = cpal::Sample::from::<_>(&s[i]);
            *sample = value;
        }
        i += 1;
    }
}

fn write_input_data2<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: cpal::Sample,
    U: cpal::Sample + hound::Sample,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = cpal::Sample::from(&sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

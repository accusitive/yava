#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::TemplateApp;
use cpal::{Device, traits::StreamTrait};

pub fn setup_audio() {
    println!("Setting up audio");
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("no input device available");
    use cpal::traits::{DeviceTrait, HostTrait};
    let mut supported_configs_range = device
        .supported_input_configs()
        .expect("error while querying configs");
    let supported_config = supported_configs_range
        .next()
        .expect("no supported config?!")
        .with_max_sample_rate();
    println!("About to build stream");
    let stream = device.build_input_stream(
        &supported_config.config(),
        move |data: &[u8], _: &cpal::InputCallbackInfo| {
            println!("Received {} bytes", data.len())
            // react to stream events and read or write stream data here.
        },
        move |err| {
            // react to errors here.
        },
        None, // None=blocking, Some(Duration)=timeout
    ).unwrap();
    stream.play().unwrap();
    println!("Playing stream");
    println!("Got audio devices. {:#?}", supported_config);
}

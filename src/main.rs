extern crate vst;

use std::env;
use std::error::Error;
use std::path::Path;
use std::process;
use std::sync::{Arc, Mutex};

use vst::host::{Host, HostBuffer, PluginLoader};
use vst::plugin::Plugin;

#[allow(dead_code)]
struct SampleHost;

impl Host for SampleHost {
    fn automate(&mut self, index: i32, value: f32) {
        println!("Parameter {} had it's value changed to {}", index, value);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: simple_host path/to/vst");
        process::exit(1);
    }

    let path = Path::new(&args[1]);

    let host = Arc::new(Mutex::new(SampleHost));

    println!("Loading {} ...", path.to_str().unwrap());

    let mut loader = PluginLoader::load(path, Arc::clone(&host))
        .unwrap_or_else(|e| panic!("Failed to load plugin: {}", e.description()));

    let mut instance = loader.instance().unwrap();

    let info = instance.get_info();

    println!(
        "Loaded '{}':\n\t\
         Vendor: {}\n\t\
         Presets: {}\n\t\
         Parameters: {}\n\t\
         VST ID: {}\n\t\
         Version: {}\n\t\
         Initial Delay: {} samples",
        info.name,
        info.vendor,
        info.presets,
        info.parameters,
        info.unique_id,
        info.version,
        info.initial_delay
    );

    instance.init();
    let mut host_buffer: HostBuffer<f32> = HostBuffer::new(2, 2);
    let inputs = vec![vec![0.0; 1000]; 2];
    let mut outputs = vec![vec![0.0; 1000]; 2];
    let mut audio_buffer = host_buffer.bind(&inputs, &mut outputs);

    loop {
        instance.process(&mut audio_buffer);
    }

    println!("Initialized instance!");

    println!("Closing instance...");
}

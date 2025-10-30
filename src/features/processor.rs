use crate::{config::structure::FeaturesConfig, helpers::data::Request};

use std::{
    io::Write,
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

fn process(request: Request, config: FeaturesConfig) {
    if config.log {
        println!("New request: {}", request);
        std::io::stdout().flush().unwrap();
    }

    if config.time {
        println!("  Time taken: {}", request.time);
        std::io::stdout().flush().unwrap();
    }
}

pub fn start_processor(rx: Receiver<Request>, config: FeaturesConfig) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(request) => process(request, config.clone()),
                Err(error) => eprintln!("Error: {error}"),
            }
        }
    })
}

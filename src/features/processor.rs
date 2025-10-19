use crate::{config::parser::FeaturesConfig, helpers::data::Request};

use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

fn process(request: Request, config: FeaturesConfig) {
    if config.log {
        println!("New request: {}", request);
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

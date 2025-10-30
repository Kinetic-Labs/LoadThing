use crate::helpers::error::{self, ERROR_9};
use crate::helpers::logger::TableLogger;
use crate::{config::structure::FeaturesConfig, helpers::data::Request};

use std::{
    sync::{Arc, Mutex, mpsc::Receiver},
    thread::{self, JoinHandle},
};

fn process(request: Request, config: FeaturesConfig, logger: Arc<Mutex<TableLogger>>) {
    if config.log {
        let mut logger = logger.lock().unwrap();

        let time_str = if request.time >= 60_000 {
            format!("{:.1}m", request.time as f64 / 60_000.0)
        } else if request.time >= 1_000 {
            format!("{:.2}s", request.time as f64 / 1_000.0)
        } else {
            format!("{}ms", request.time)
        };

        logger.add_row(vec![
            request.location.to_string(),
            request.path.to_string(),
            time_str,
        ]);
        logger.log();
    }
}

pub fn start_processor(rx: Receiver<Request>, config: FeaturesConfig) -> JoinHandle<()> {
    let logger = Arc::new(Mutex::new(TableLogger::new(vec![
        "Location      ".to_string(),
        "Path       ".to_string(),
        "Time   ".to_string(),
    ])));

    thread::spawn(move || {
        loop {
            match rx.recv() {
                Ok(request) => process(request, config.clone(), Arc::clone(&logger)),
                Err(error) => {
                    error::send_error(ERROR_9, format!("while recieving request data : {error}"))
                }
            }
        }
    })
}

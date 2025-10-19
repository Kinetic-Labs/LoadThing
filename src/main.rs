mod features;
mod helpers;
mod network;

use crate::features::processor;
use crate::helpers::data::Request;
use crate::network::proxy;
use std::io;
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

fn start_tasks(
    tx: Sender<Request>,
    rx: Receiver<Request>,
    listener: TcpListener,
) -> (JoinHandle<()>, JoinHandle<()>) {
    (
        processor::start_processor(rx),
        proxy::start_proxy_listener(listener, tx),
    )
}

fn main() -> io::Result<()> {
    let port: u16 = 9595;
    let address: String = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&address) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("Error biding adress: {error}");

            return Err(error);
        }
    };

    let (tx, rx) = mpsc::channel::<Request>();
    let (processor_handle, proxy_handle) = start_tasks(tx, rx, listener);

    println!(
        "LoadThing server running on {}",
        helpers::misc::format_hostname(helpers::misc::Protocol::Http, address)
    );

    match processor_handle.join() {
        Ok(_) => {}
        Err(_) => eprintln!("Failed to join processor handle"),
    }

    match proxy_handle.join() {
        Ok(_) => {}
        Err(_) => eprintln!("Failed to join proxy handle"),
    }

    Ok(())
}

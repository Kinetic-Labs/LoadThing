mod data;
mod processor;
mod proxy;

use std::io;
use std::net::TcpListener;
use std::sync::mpsc;

fn main() -> io::Result<()> {
    let port: u16 = 9595;
    let address: String = format!("127.0.0.1:{}", port);
    let listener = match TcpListener::bind(&address) {
        Ok(val) => val,
        Err(error) => {
            eprintln!("error biding adress: {error}");

            return Err(error);
        }
    };

    let (tx, rx) = mpsc::channel::<data::Request>();

    let processor_handle = processor::start_processor(rx);
    let proxy_handle = proxy::start_proxy_listener(listener, tx);

    println!("LoadThing server running on {address}");

    let _ = processor_handle.join();
    let _ = proxy_handle.join();

    Ok(())
}

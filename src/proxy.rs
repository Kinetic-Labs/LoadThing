use crate::data::{self, Request};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

fn handle_client(mut client: TcpStream, tx: Sender<data::Request>) {
    let mut buffer = [0; 8192];

    let n = match client.read(&mut buffer) {
        Ok(n) if n > 0 => n,
        _ => return,
    };

    let request = String::from_utf8_lossy(&buffer[..n]);
    let mut host = "";
    let mut port = 80;
    let mut path = "/";

    for line in request.lines() {
        if line.starts_with("Host: ") {
            host = line[6..].trim();
            if let Some(colon_pos) = host.find(':') {
                port = host[colon_pos + 1..].parse().unwrap_or(80);
                host = &host[..colon_pos];
            }
        }
        if line.starts_with("GET ") || line.starts_with("POST ") {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                path = parts[1];
            }
        }
    }

    if host.is_empty() {
        eprintln!("No host found in request");
        return;
    }

    match tx.send(Request {
        location: host.to_string(),
        target: host.to_string(),
        path: path.to_string(),
    }) {
        Ok(_) => {}
        Err(error) => eprintln!("error sending data: {}", error),
    }

    let target_addr = format!("{}:{}", host, port);
    let mut server = match TcpStream::connect(&target_addr) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("Failed to connect to {}: {}", target_addr, error);
            let error_response = "HTTP/1.1 502 Bad Gateway\r\n\r\n";
            let _ = client.write_all(error_response.as_bytes());
            return;
        }
    };

    if let Err(error) = server.write_all(&buffer[..n]) {
        eprintln!("Failed to write to server: {}", error);
        return;
    }

    let mut response_buffer = [0; 8192];
    loop {
        match server.read(&mut response_buffer) {
            Ok(0) => break,
            Ok(n) => {
                if let Err(e) = client.write_all(&response_buffer[..n]) {
                    eprintln!("Failed to write to client: {}", e);
                    break;
                }
            }
            Err(error) => {
                eprintln!("Failed to read from server: {}", error);
                break;
            }
        }
    }
}

pub fn start_proxy_listener(listener: TcpListener, tx: Sender<data::Request>) -> JoinHandle<()> {
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    handle_client(stream, tx.clone());
                }
                Err(error) => {
                    eprintln!("Connection failed: {}", error);
                }
            }
        }
    })
}

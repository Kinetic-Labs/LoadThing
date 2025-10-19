use crate::config::parser::ProxyConfig;
use crate::helpers::data::{self, Request};
use native_tls::TlsConnector;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::Sender;
use std::thread::{self, JoinHandle};

fn rewrite_http_request(buffer: &[u8], clean_host: &str) -> Vec<u8> {
    let request = String::from_utf8_lossy(buffer);
    let lines: Vec<&str> = request.lines().collect();

    if lines.is_empty() {
        return buffer.to_vec();
    }

    let mut result = String::new();

    let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
    if first_line_parts.len() >= 3 {
        let method = first_line_parts[0];
        let url = first_line_parts[1];
        let version = first_line_parts[2];

        let path = if url.starts_with("http://") || url.starts_with("https://") {
            url.split("://")
                .nth(1)
                .and_then(|s| s.split_once('/'))
                .map(|(_, path)| format!("/{}", path))
                .unwrap_or_else(|| "/".to_string())
        } else {
            url.to_string()
        };

        result.push_str(&format!("{} {} {}\r\n", method, path, version));
    } else {
        result.push_str(lines[0]);
        result.push_str("\r\n");
    }

    let mut has_host = false;
    for line in lines.iter().skip(1) {
        if line.to_lowercase().starts_with("host:") {
            result.push_str(&format!("Host: {}\r\n", clean_host));
            has_host = true;
        } else {
            result.push_str(line);
            result.push_str("\r\n");
        }
    }

    if !has_host {
        result.push_str(&format!("Host: {}\r\n", clean_host));
    }

    result.push_str("\r\n");
    result.into_bytes()
}

fn handle_client(mut client: TcpStream, tx: Sender<data::Request>, config: ProxyConfig) {
    let mut buffer = [0; 8192];

    let Ok(n) = client.read(&mut buffer) else {
        return;
    };

    if n == 0 {
        return;
    }

    let host = config.target;
    let port = config.port;
    let path = config.path;
    let use_tls = host.starts_with("https://");

    let clean_host = host
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_end_matches('/');

    match tx.send(Request {
        location: host.to_string(),
        target: host.to_string(),
        path: path.to_string(),
    }) {
        Ok(_) => {}
        Err(error) => eprintln!("Error sending data: {}", error),
    }

    let target_addr = format!("{}:{}", clean_host, port);
    let mut server = match TcpStream::connect(&target_addr) {
        Ok(s) => s,
        Err(error) => {
            eprintln!("Failed to connect to {}: {}", target_addr, error);
            let error_response = "HTTP/1.1 502 Bad Gateway\r\n\r\n";
            let _ = client.write_all(error_response.as_bytes());
            return;
        }
    };

    let rewritten_request = rewrite_http_request(&buffer[..n], clean_host);

    if use_tls {
        let connector = match TlsConnector::new() {
            Ok(c) => c,
            Err(error) => {
                eprintln!("Failed to create TLS connector: {}", error);
                let error_response = "HTTP/1.1 502 Bad Gateway\r\n\r\n";
                let _ = client.write_all(error_response.as_bytes());
                return;
            }
        };

        let mut tls_stream = match connector.connect(clean_host, server) {
            Ok(s) => s,
            Err(error) => {
                eprintln!("Failed to establish TLS connection: {}", error);
                let error_response = "HTTP/1.1 502 Bad Gateway\r\n\r\n";
                let _ = client.write_all(error_response.as_bytes());
                return;
            }
        };

        if let Err(error) = tls_stream.write_all(&rewritten_request) {
            eprintln!("Failed to write to TLS server: {}", error);
            return;
        }

        let mut response_buffer = [0; 8192];

        loop {
            match tls_stream.read(&mut response_buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if let Err(error) = client.write_all(&response_buffer[..n]) {
                        eprintln!("Failed to write to client: {}", error);
                        break;
                    }
                }
                Err(error) => {
                    eprintln!("Failed to read from TLS server: {}", error);
                    break;
                }
            }
        }
    } else {
        if let Err(error) = server.write_all(&rewritten_request) {
            eprintln!("Failed to write to server: {}", error);
            return;
        }

        let mut response_buffer = [0; 8192];
        loop {
            match server.read(&mut response_buffer) {
                Ok(0) => break,
                Ok(n) => {
                    if let Err(error) = client.write_all(&response_buffer[..n]) {
                        eprintln!("Failed to write to client: {}", error);
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
}

pub fn start_proxy_listener(
    listener: TcpListener,
    tx: Sender<data::Request>,
    config: ProxyConfig,
) -> JoinHandle<()> {
    thread::spawn(move || {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let tx = tx.clone();
                    let config = config.clone();

                    thread::spawn(move || {
                        handle_client(stream, tx, config);
                    });
                }
                Err(error) => {
                    eprintln!("Connection failed: {}", error);
                }
            }
        }
    })
}

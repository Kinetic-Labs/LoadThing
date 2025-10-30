mod config;
mod features;
mod helpers;
mod network;

use crate::config::structure::Config;
use crate::features::processor;
use crate::helpers::data::Request;
use crate::helpers::error::{ERROR_2, ERROR_10};
use crate::helpers::{ansi, error};
use crate::network::proxy;
use std::io;
use std::net::TcpListener;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread::JoinHandle;

const ASCII_ART: &str = r#"
       LoadThing
   ,--,           ,----,
,---.'|         ,/   .`|
|   | :       ,`   .'  :
:   : |     ;    ;     /
|   ' :   .'___,/    ,'
;   ; '   |    :     |
'   | |__ ;    |.';  ;
|   | :.'|`----'  |  |
'   :    ;    '   :  ;
|   |  ./     |   |  '
;   : ;       '   :  |
|   ,/        ;   |.'
'---'         '---'
  By Kinetic Labs
"#;

fn start_tasks(
    tx: Sender<Request>,
    rx: Receiver<Request>,
    listener: TcpListener,
    config: Config,
) -> (JoinHandle<()>, JoinHandle<()>) {
    (
        processor::start_processor(rx, config.features_config),
        proxy::start_proxy_listener(listener, tx, config.proxy_config),
    )
}

fn main() -> io::Result<()> {
    println!("{}{}{}", ansi::MAGENTA, ASCII_ART, ansi::RESET);

    let config: Config = config::parser::parse_config()?;
    let port: u16 = config.web_config.port;
    let hostname: String = config.clone().web_config.hostname;
    let address: String = format!("{hostname}:{port}");
    let listener = match TcpListener::bind(&address) {
        Ok(val) => val,
        Err(error) => {
            error::send_error(ERROR_2, String::from("while binding port"));

            return Err(error);
        }
    };

    let (tx, rx) = mpsc::channel::<Request>();
    let (processor_handle, proxy_handle) = start_tasks(tx, rx, listener, config);

    println!(
        "{}LoadThing server running on {}{}{}\n",
        ansi::GREEN,
        ansi::ORANGE,
        helpers::misc::format_hostname(helpers::misc::Protocol::Http, address),
        ansi::RESET,
    );

    match proxy_handle.join() {
        Ok(_) => {}
        Err(_) => error::send_error(ERROR_10, String::from("while joining proxy handle")),
    }

    match processor_handle.join() {
        Ok(_) => {}
        Err(_) => error::send_error(ERROR_10, String::from("while joining processor handle")),
    }

    Ok(())
}

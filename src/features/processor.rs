use crate::helpers::data;

use std::{
    sync::mpsc::Receiver,
    thread::{self, JoinHandle},
};

pub fn start_processor(rx: Receiver<data::Request>) -> JoinHandle<()> {
    thread::spawn(move || {
        println!("{}", rx.recv().unwrap());
    })
}

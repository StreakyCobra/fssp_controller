use driver::command::{Command, GCode};
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::{thread, time};

pub mod command;

pub fn connect_driver(address: &str) -> Option<mpsc::Sender<Command>> {
    let (tx, rx) = mpsc::channel();
    if let Some(stream) = TcpStream::connect(address).ok() {
        thread::spawn(move || emit(stream, rx));

        return Some(tx);
    };

    return None;
}

fn emit(mut stream: TcpStream, rx: mpsc::Receiver<Command>) {
    let wait_duration = time::Duration::from_millis(100);
    stream.set_nodelay(true).unwrap();
    loop {
        for received in rx.try_iter() {
            println!("{:?}", received);
            stream
                .write(format!("{}\n", received.to_gcode()).as_bytes())
                .unwrap();
            stream.flush().unwrap();
        }
        thread::sleep(wait_duration);
    }
}

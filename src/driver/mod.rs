use driver::command::{Command, GCode};
use std::io::prelude::*;
use std::net::TcpStream;
use std::sync::mpsc;
use std::{thread, time};

pub mod command;

const WAIT_DURATION_MS: u64 = 100;

pub fn connect_driver(address: &str) -> mpsc::Sender<Command> {
    let (tx, rx) = mpsc::channel();

    if let Some(stream) = TcpStream::connect(address).ok() {
        thread::spawn(move || emit(stream, rx));
    } else {
        thread::spawn(move || dummy(rx));
    };

    return tx;
}

fn emit(mut stream: TcpStream, rx: mpsc::Receiver<Command>) {
    let wait_duration = time::Duration::from_millis(WAIT_DURATION_MS);
    stream.set_nodelay(true).unwrap();
    loop {
        for received in rx.try_iter() {
            let code = received.to_gcode();
            if code.len() > 0 {
                stream.write(format!("{}\n", code).as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        }
        thread::sleep(wait_duration);
    }
}

fn dummy(rx: mpsc::Receiver<Command>) {
    let wait_duration = time::Duration::from_millis(WAIT_DURATION_MS);
    loop {
        for received in rx.try_iter() {
            println!("{:?}\r", received);
        }
        thread::sleep(wait_duration);
    }
}

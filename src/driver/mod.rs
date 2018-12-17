use driver::command::Command;
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
    loop {
        let one_sec = time::Duration::from_secs(1);
        loop {
            for received in rx.try_iter() {
                println!("{:?}", received);
                stream
                    .write(format!("{:?}\n", received).as_bytes())
                    .unwrap();
            }
            thread::sleep(one_sec);
        }
    }
}

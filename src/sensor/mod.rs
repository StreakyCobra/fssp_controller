use sensor::event::Event;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;
use std::sync::mpsc;
use std::{thread, time};

pub mod event;

pub fn connect_sensor(address: &str) -> Option<mpsc::Receiver<Event>> {
    let (tx, rx) = mpsc::channel();
    if let Some(stream) = TcpStream::connect(address).ok() {
        thread::spawn(move || emit(stream, tx));

        return Some(rx);
    };

    return None;
}

fn emit(stream: TcpStream, tx: mpsc::Sender<Event>) {
    let one_sec = time::Duration::from_secs(1);
    let mut buf = BufReader::new(stream);
    let mut line: String = String::new();
    loop {
        while let Ok(count) = buf.read_line(&mut line) {
            if count == 0 {
                break;
            }
            tx.send(Event::Content {
                string: line.clone(),
            })
            .unwrap();
            line.clear();
        }
        thread::sleep(one_sec);
    }
}

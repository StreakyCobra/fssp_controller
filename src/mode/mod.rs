use driver::command::Command;
use gilrs::Button;
use sensor::event::Event;
use std::sync::mpsc;
use std::{thread, time};

pub enum Mode {
    Manual,
    Calibration,
    Simulation,
}

impl Mode {
    pub fn next(&self) -> Mode {
        match self {
            Mode::Manual => Mode::Calibration,
            Mode::Calibration => Mode::Simulation,
            Mode::Simulation => Mode::Manual,
        }
    }
}

pub fn master_loop(
    controls: mpsc::Receiver<gilrs::Button>,
    commands: Option<mpsc::Sender<Command>>,
    events: Option<mpsc::Receiver<Event>>,
) {
    let mut current: Mode = Mode::Manual;
    let wait_duration = time::Duration::from_millis(100);
    loop {
        for button in controls.try_iter() {
            println!("{:?}", button);
            if let Button::Mode = button {
                current = current.next();
            }
        }
        if let Some(rx) = &events {
            for event in rx.try_iter() {
                println!("{:?}", event)
            }
        }
        if let Some(tx) = &commands {
            tx.send(Command::MoveTo {
                x: Some(10),
                y: None,
                z: None,
                f: None,
            })
            .unwrap();
        }
        thread::sleep(wait_duration);
    }
}

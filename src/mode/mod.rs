mod manual;

use driver::command::Command;
use gilrs::Button;
use sensor::event::Event;
use std::sync::mpsc;
use std::{thread, time};

#[derive(Debug)]
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
    controls: mpsc::Receiver<Button>,
    commands: Option<mpsc::Sender<Command>>,
    events: Option<mpsc::Receiver<Event>>,
) {
    let mut mode: Mode = Mode::Manual;
    let wait_duration = time::Duration::from_millis(100);
    loop {
        for button in controls.try_iter() {
            if let Button::Mode = button {
                next_mode(&mut mode);
            } else {
                match mode {
                    Mode::Manual => {
                        manual::handle(button, &commands);
                    }
                    Mode::Calibration => println!("Not supported yet"),
                    Mode::Simulation => println!("Not supported yet"),
                };
            }
        }
        if let Some(rx) = &events {
            for event in rx.try_iter() {
                println!("{:?}", event)
            }
        }
        thread::sleep(wait_duration);
    }
}

fn next_mode(mode: &mut Mode) {
    *mode = mode.next();
    println!("{:?}", mode);
}

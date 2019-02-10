mod manual;

use controller::control::Control;
use driver::command::Command;
use gilrs;
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
    controls: mpsc::Receiver<Control>,
    commands: Option<mpsc::Sender<Command>>,
    events: Option<mpsc::Receiver<Event>>,
) {
    let mut mode: Mode = Mode::Manual;
    let wait_duration = time::Duration::from_millis(100);
    loop {
        handle_controls(&controls, &mut mode, &commands);
        handle_events(&events, &mut mode, &commands);
        thread::sleep(wait_duration);
    }
}

fn handle_controls(
    controls: &mpsc::Receiver<Control>,
    mode: &mut Mode,
    commands: &Option<mpsc::Sender<Command>>,
) {
    for control in controls.try_iter() {
        // Check if it is the Mode button to change mode here
        if let Control::Joystick {
            event:
                gilrs::Event {
                    id: _,
                    event: gilrs::EventType::ButtonReleased { 0: button, 1: _ },
                    time: _,
                },
        } = control
        {
            if button == gilrs::Button::Mode {
                next_mode(mode);
                continue;
            }
        }

        // Dispatch the controls to the active mode
        match mode {
            Mode::Manual => {
                manual::handle(control, &commands);
            }
            Mode::Calibration => println!("Not supported yet"),
            Mode::Simulation => println!("Not supported yet"),
        };
    }
}

fn handle_events(
    events: &Option<mpsc::Receiver<Event>>,
    _mode: &mut Mode,
    _commands: &Option<mpsc::Sender<Command>>,
) {
    if let Some(rx) = &events {
        for event in rx.try_iter() {
            println!("{:?}", event)
        }
    }
}

fn next_mode(mode: &mut Mode) {
    *mode = mode.next();
    println!("{:?}", mode);
}

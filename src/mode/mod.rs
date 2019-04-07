mod manual;

use controller::control::Control;
use driver::command::Command;
use gilrs;
use sensor::event::Event;
use std::sync::mpsc;
use std::{thread, time};
use ncurses;

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
    commands: mpsc::Sender<Command>,
    events: mpsc::Receiver<Event>,
) {
    let mut mode: Mode = Mode::Manual;
    let wait_duration = time::Duration::from_millis(100);
    loop {
        if !handle_controls(&controls, &mut mode, &commands) {
            break;
        };
        handle_events(&events, &mut mode, &commands);
        thread::sleep(wait_duration);
    }
}

fn handle_controls(
    controls: &mpsc::Receiver<Control>,
    mode: &mut Mode,
    commands: &mpsc::Sender<Command>,
) -> bool {
    for control in controls.try_iter() {
        // Handle quit trigger
        if is_quit_trigger(&control) {
            return false;
        }

        // Handle mode change trigger
        if is_mode_trigger(&control) {
            next_mode(mode);
            continue;
        }

        // Dispatch the controls to the active mode
        match mode {
            Mode::Manual => {
                manual::handle(control, &commands);
            }
            Mode::Calibration => {
                println!("Not supported yet\r");
            }
            Mode::Simulation => {
                println!("Not supported yet\r");
            }
        };
    }
    return true;
}

fn handle_events(
    events: &mpsc::Receiver<Event>,
    _mode: &mut Mode,
    _commands: &mpsc::Sender<Command>,
) {
    for event in events.try_iter() {
        println!("{:?}\r", event)
    }
}

fn is_mode_trigger(control: &Control) -> bool {
    match *control {
        Control::Joystick {
            event:
                gilrs::Event {
                    id: _,
                    event: gilrs::EventType::ButtonReleased { 0: button, 1: _ },
                    time: _,
                },
        } => return button == gilrs::Button::Mode,
        Control::Keyboard { keycode } => return keycode == 'm' as i32,
        _ => return false,
    }
}

fn is_quit_trigger(control: &Control) -> bool {
    match *control {
        Control::Joystick {
            event:
                gilrs::Event {
                    id: _,
                    event: gilrs::EventType::ButtonReleased { 0: button, 1: _ },
                    time: _,
                },
        } => return button == gilrs::Button::Start,
        Control::Keyboard { keycode } => return keycode == 'q' as i32,
        _ => return false,
    }
}

fn next_mode(mode: &mut Mode) {
    *mode = mode.next();
    println!("{:?}\r", mode);
}

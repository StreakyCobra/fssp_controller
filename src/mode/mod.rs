mod manual;
mod calibration;
mod simulation;

use controller::control::Control;
use driver::command::Command;
use gilrs;
use mode::manual::Manual;
use sensor::event::Event;
use std::sync::mpsc;
use std::{thread, time};

trait Mode {
    fn init(driver: &mpsc::Sender<Command>) -> Self
    where
        Self: Sized;
    fn name(&self) -> String;
    fn next_mode(&self) -> Box<Mode>;
    fn handle(&self, control: Control, driver: &mpsc::Sender<Command>);
}

pub fn master_loop(
    controller: mpsc::Receiver<Control>,
    driver: mpsc::Sender<Command>,
    sensor: mpsc::Receiver<Event>,
) {
    let mut mode: Box<Mode> = Box::new(Manual::init(&driver));
    println!(":: Welcome to FSSP\r");
    println!(":: Mode: {}\r", mode.name());
    let wait_duration = time::Duration::from_millis(100);
    loop {
        if !handle_controls(&controller, &mut mode, &driver) {
            break;
        };
        handle_events(&sensor, &mut mode, &driver);
        thread::sleep(wait_duration);
    }
}

fn handle_controls(
    controller: &mpsc::Receiver<Control>,
    mode: &mut Box<Mode>,
    driver: &mpsc::Sender<Command>,
) -> bool {
    for control in controller.try_iter() {
        // Handle quit trigger
        if is_quit_trigger(&control) {
            return false;
        }

        // Handle mode change trigger
        if is_mode_trigger(&control) {
            next_mode(mode);
            continue;
        }

        mode.handle(control, &driver);
    }
    return true;
}

fn next_mode(mode: &mut Box<Mode>) {
    *mode = mode.next_mode();
    println!(":: Mode: {}\r", mode.name());
}

fn handle_events(
    events: &mpsc::Receiver<Event>,
    _mode: &mut Box<Mode>,
    _driver: &mpsc::Sender<Command>,
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

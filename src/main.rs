extern crate gilrs;

pub mod control;

use control::{Control, GCode};
use gilrs::{Button, Event, EventType, Gilrs};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }

    loop {
        while let Some(Event { id, event, time }) = gilrs.next_event() {
            let control = match event {
                EventType::ButtonReleased { 0: button, 1: code } => match button {
                    Button::DPadDown => Some(Control::MoveTo {
                        x: None,
                        y: Some(-10),
                        z: None,
                        f: None,
                    }),
                    Button::DPadLeft => Some(Control::MoveTo {
                        x: Some(-10),
                        y: None,
                        z: None,
                        f: None,
                    }),
                    Button::DPadRight => Some(Control::MoveTo {
                        x: Some(10),
                        y: None,
                        z: None,
                        f: None,
                    }),
                    Button::DPadUp => Some(Control::MoveTo {
                        x: None,
                        y: Some(10),
                        z: None,
                        f: None,
                    }),
                    _ => None,
                },
                _ => None,
            };
            match control {
                None => (),
                Some(control) => println!("{}", control.to_gcode()),
            }
        }
    }
}

extern crate gilrs;

pub mod control;

use control::{Control, GCode};
use gilrs::{Button, Event, EventType, Gilrs};

fn main() {
    let mut gilrs = Gilrs::new().unwrap();

    if let Some((_id, gamepad)) = gilrs.gamepads().nth(0) {
        println!("{}", gamepad.os_name())
    }

    loop {
        while let Some(Event {
            id: _,
            event,
            time: _,
        }) = gilrs.next_event()
        {
            let control = match event {
                EventType::ButtonReleased { 0: button, 1: _ } => match button {
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

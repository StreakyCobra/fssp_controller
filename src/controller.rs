use gilrs::{Button, Event, EventType, Gilrs};

use driver::{Command, GCode};

pub fn joystick_test() {
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
            if let Some(control) = match event {
                EventType::ButtonReleased { 0: button, 1: _ } => match button {
                    Button::DPadDown => Some(Command::MoveTo {
                        x: None,
                        y: Some(-10),
                        z: None,
                        f: None,
                    }),
                    Button::DPadLeft => Some(Command::MoveTo {
                        x: Some(-10),
                        y: None,
                        z: None,
                        f: None,
                    }),
                    Button::DPadRight => Some(Command::MoveTo {
                        x: Some(10),
                        y: None,
                        z: None,
                        f: None,
                    }),
                    Button::DPadUp => Some(Command::MoveTo {
                        x: None,
                        y: Some(10),
                        z: None,
                        f: None,
                    }),
                    _ => None,
                },
                _ => None,
            } {
                println!("{}", control.to_gcode())
            }
        }
    }
}

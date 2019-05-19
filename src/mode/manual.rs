use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use std::sync::mpsc;

use mode::Mode;

pub struct Manual {}

impl Mode for Manual {
    fn init() -> Self {
        Manual {}
    }

    fn name(&self) -> String {
        String::from("Manual")
    }

    fn next_mode(&self) -> Box<Mode> {
        Box::new(Manual::init())
    }

    fn handle(&self, control: Control, driver: &mpsc::Sender<Command>) {
        match control {
            Control::Joystick {
                event:
                    gilrs::Event {
                        id: _,
                        event,
                        time: _,
                    },
            } => {
                if let gilrs::EventType::ButtonReleased { 0: button, 1: _ } = event {
                    self.handle_button(button, &driver)
                } else if let gilrs::EventType::AxisChanged {
                    0: axis,
                    1: value,
                    2: _,
                } = event
                {
                    self.handle_axis(axis, value, &driver)
                }
            }
            Control::Keyboard { keycode } => println!("Key press: {}\r", keycode as u8 as char),
        }
    }
}

impl Manual {
    fn handle_button(&self, button: Button, tx: &mpsc::Sender<Command>) {
        match button {
            Button::DPadDown => tx
                .send(Command::MoveTo {
                    x: None,
                    y: Some(-10),
                    z: None,
                    f: None,
                })
                .unwrap(),
            Button::DPadUp => tx
                .send(Command::MoveTo {
                    x: None,
                    y: Some(10),
                    z: None,
                    f: None,
                })
                .unwrap(),
            Button::DPadLeft => tx
                .send(Command::MoveTo {
                    x: Some(-10),
                    y: None,
                    z: None,
                    f: None,
                })
                .unwrap(),
            Button::DPadRight => tx
                .send(Command::MoveTo {
                    x: Some(10),
                    y: None,
                    z: None,
                    f: None,
                })
                .unwrap(),
            Button::North => tx
                .send(Command::MoveTo {
                    x: None,
                    y: None,
                    z: Some(10),
                    f: None,
                })
                .unwrap(),
            Button::South => tx
                .send(Command::MoveTo {
                    x: None,
                    y: None,
                    z: Some(-10),
                    f: None,
                })
                .unwrap(),
            _ => (),
        }
    }

    fn handle_axis(&self, axis: gilrs::Axis, value: f32, tx: &mpsc::Sender<Command>) {
        match axis {
            gilrs::Axis::LeftStickX => tx
                .send(Command::MoveTo {
                    x: Some((value * 100.) as i32),
                    y: None,
                    z: None,
                    f: None,
                })
                .unwrap(),
            gilrs::Axis::LeftStickY => tx
                .send(Command::MoveTo {
                    x: None,
                    y: Some((value * 100.) as i32),
                    z: None,
                    f: None,
                })
                .unwrap(),
            gilrs::Axis::RightStickY => tx
                .send(Command::MoveTo {
                    x: None,
                    y: None,
                    z: Some((value * 100.) as i32),
                    f: None,
                })
                .unwrap(),
            _ => (),
        }
    }
}

use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use mode::manual::Manual;
use mode::Mode;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Calibration {
    driver: mpsc::Sender<Command>,
}

impl Mode for Calibration {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetAbsolute).unwrap();
        Calibration {
            driver: driver.clone(),
        }
    }

    fn name(&self) -> String {
        String::from("Simulation")
    }

    fn next_mode(&self) -> Box<Mode> {
        Box::new(Manual::init(&self.driver))
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
            Control::Keyboard { keycode } => self.handle_key(keycode, &driver),
        }
    }
}

impl Calibration {
    fn handle_button(&self, button: Button, tx: &mpsc::Sender<Command>) {
        println!("Button press: {:?}\r", button);
        tx.send(Command::NoOp).unwrap();
    }

    fn handle_axis(&self, axis: gilrs::Axis, value: f32, tx: &mpsc::Sender<Command>) {
        println!("Axis changed: {:?} {}\r", axis, value);
        tx.send(Command::NoOp).unwrap();
    }

    fn handle_key(&self, keycode: i32, tx: &mpsc::Sender<Command>) {
        println!("Key press: {}\r", keycode as u8 as char);
        tx.send(Command::NoOp).unwrap();
    }
}

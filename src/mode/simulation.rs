use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use mode::manual::Manual;
use mode::Mode;
use std::sync::mpsc;

#[derive(Debug)]
pub struct Simulation {
    driver: mpsc::Sender<Command>,
}

impl Mode for Simulation {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetAbsolute).unwrap();
        Simulation {
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
            Control::Keyboard { keycode } => println!("Key press: {}\r", keycode as u8 as char),
        }
    }
}

impl Simulation {
    fn handle_button(&self, button: Button, tx: &mpsc::Sender<Command>) {}
    fn handle_axis(&self, axis: gilrs::Axis, value: f32, tx: &mpsc::Sender<Command>) {}
}

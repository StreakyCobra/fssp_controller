use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use mode::simulation::Simulation;
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

    fn start(&mut self) {}

    fn stop(&mut self) {}

    fn name(&self) -> String {
        String::from("Calibration")
    }

    fn next_mode(&self) -> Box<Mode> {
        Box::new(Simulation::init(&self.driver))
    }

    fn handle(&mut self, control: Control) {
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
                    self.handle_button(button)
                } else if let gilrs::EventType::AxisChanged {
                    0: axis,
                    1: value,
                    2: _,
                } = event
                {
                    self.handle_axis(axis, value)
                }
            }
            Control::Keyboard { keycode } => self.handle_key(keycode),
        }
    }
}

impl Calibration {
    fn handle_button(&self, button: Button) {
        println!("Button press: {:?}\r", button);
    }

    fn handle_axis(&self, axis: gilrs::Axis, value: f32) {
        println!("Axis changed: {:?} {}\r", axis, value);
    }

    fn handle_key(&self, keycode: i32) {
        println!("Key press: {}\r", keycode as u8 as char);
    }
}

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

    fn handle(&self, control: Control, driver: &mpsc::Sender<Command>) {}
}

impl Calibration {
    fn handle_button(&self, button: Button, tx: &mpsc::Sender<Command>) {}

    fn handle_axis(&self, axis: gilrs::Axis, value: f32, tx: &mpsc::Sender<Command>) {}
}

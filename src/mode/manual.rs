use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use mode::calibration::Calibration;
use mode::Mode;
use na::Vector3;
use std::sync::mpsc;
use std::thread;
use std::time;

type Num = i32;

const WAIT_DURATION_MS: u64 = 1000;

#[derive(Debug)]
pub struct Manual {
    driver: mpsc::Sender<Command>,
    velocity: Vector3<Num>,
    thread: mpsc::Sender<Option<Vector3<Num>>>,
}

fn emit(
    rx: mpsc::Receiver<Option<Vector3<Num>>>,
    driver: mpsc::Sender<Command>,
    velocity: Vector3<Num>,
) {
    let wait_duration = time::Duration::from_millis(WAIT_DURATION_MS);
    let mut velocity = velocity;
    let mut command;
    'emitter: loop {
        for received in rx.try_iter() {
            match received {
                None => break 'emitter,
                Some(vec) => {
                    // println!("{:?}", vec);
                    velocity = vec;
                }
            }
        }
        // println!("{:?}", velocity);
        command = Command::MoveTo {
            x: Some(velocity[0]),
            y: Some(velocity[1]),
            z: Some(velocity[2]),
            f: None,
        };
        if Vector3::new(0, 0, 0) != velocity {
            driver.send(command).unwrap();
        }
        thread::sleep(wait_duration);
    }
}

impl Mode for Manual {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetRelative).unwrap();
        let (tx, rx) = mpsc::channel();
        let state = Manual {
            driver: driver.clone(),
            velocity: Vector3::new(0, 0, 0),
            thread: tx,
        };
        let driver = driver.clone();
        let velocity = state.velocity.clone();
        thread::spawn(move || emit(rx, driver, velocity));
        return state;
    }

    fn stop(&mut self) {
        self.thread.send(None).unwrap();
    }

    fn name(&self) -> String {
        String::from("Manual")
    }

    fn next_mode(&self) -> Box<Mode> {
        Box::new(Calibration::init(&self.driver))
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

impl Manual {
    fn handle_button(&mut self, button: Button) {
        println!("Button press: {:?}\r", button);
    }

    fn handle_axis(&mut self, axis: gilrs::Axis, value: f32) {
        match axis {
            gilrs::Axis::LeftStickX => {
                self.velocity.x = (value * 10.) as i32;
            }
            gilrs::Axis::LeftStickY => {
                self.velocity.y = (value * 10.) as i32;
            }
            gilrs::Axis::RightStickY => {
                self.velocity.z = (value * 10.) as i32;
            }
            _ => (),
        }
        // println!("{:?}", self.velocity);
        self.thread.send(Some(self.velocity)).unwrap();
    }

    fn handle_key(&mut self, keycode: i32) {
        println!("Key press: {}\r", keycode as u8 as char);
    }
}

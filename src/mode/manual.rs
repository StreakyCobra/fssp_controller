use controller::control::Control;
use driver::command::Command;
use driver::command::Num;
use gilrs;
use gilrs::Button;
use mode::calibration::Calibration;
use mode::Mode;
use na::Vector3;
use std::sync::mpsc;
use std::thread;
use std::time;
use std::cmp::{min, max};

const WAIT_DURATION_MS: u64 = 100;

#[derive(Debug)]
pub struct Manual {
    driver: mpsc::Sender<Command>,
    vector: Vector3<Num>,
    speed: Num,
    thread: mpsc::Sender<Option<Vector3<Num>>>,
}

fn emit(
    rx: mpsc::Receiver<Option<Vector3<Num>>>,
    driver: mpsc::Sender<Command>,
    vector: Vector3<Num>,
) {
    let wait_duration = time::Duration::from_millis(WAIT_DURATION_MS);
    let mut vector = vector;
    let mut command;
    'emitter: loop {
        for received in rx.try_iter() {
            match received {
                None => break 'emitter,
                Some(vec) => {
                    vector = vec;
                }
            }
        }
        command = Command::MoveTo {
            x: Some(vector[0]),
            y: Some(vector[1]),
            z: Some(vector[2]),
            f: None,
        };
        driver.send(command).unwrap();
        thread::sleep(wait_duration);
    }
}

impl Mode for Manual {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetRelative).unwrap();
        let (tx, rx) = mpsc::channel();
        let state = Manual {
            driver: driver.clone(),
            vector: Vector3::new(0, 0, 0),
            speed: 6000,
            thread: tx,
        };
        let driver = driver.clone();
        let vector = state.vector.clone();
        thread::spawn(move || emit(rx, driver, vector));
        return state;
    }

    fn start(&mut self) {
        self.print_speed();
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
    fn print_speed(&mut self) {
        println!(":: Speed = {} mm/min\r", self.speed)
    }

    fn handle_button(&mut self, button: Button) {
        println!("Button press: {:?}\r", button);
    }

    fn handle_axis(&mut self, axis: gilrs::Axis, value: f32) {
        match axis {
            gilrs::Axis::LeftStickX => {
                self.vector.x = (value * 10.) as Num;
            }
            gilrs::Axis::LeftStickY => {
                self.vector.y = (value * 10.) as Num;
            }
            gilrs::Axis::RightStickY => {
                self.vector.z = (value * 10.) as Num;
            }
            _ => (),
        }
        self.thread.send(Some(self.vector)).unwrap();
    }

    fn handle_key(&mut self, keycode: i32) {
        match keycode as u8 as char {
            'w' => {
                self.speed = min(10000, self.speed + 100);
                self.print_speed();
            },
            's' => {
                self.speed = max(0, self.speed - 100);
                self.print_speed();
            },
            'a' => {
                self.speed = max(0, self.speed / 2);
                self.print_speed();
            },
            'd' => {
                self.speed = min(10000, self.speed * 2);
                self.print_speed();
            },
            _ => ()
        }
    }
}

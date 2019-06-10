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

const FREQUENCY: f32 = 1.0;
const MAX_SPEED: f32 = 60_000.0;
const MIN_SPEED: f32 = FREQUENCY * 60.0;

#[derive(Debug, Clone)]
struct Target {
    vector: Vector3<Num>,
    speed: Num,
}

#[derive(Debug)]
pub struct Manual {
    driver: mpsc::Sender<Command>,
    vector: Vector3<f32>,
    speed: f32,
    thread: mpsc::Sender<Option<Target>>,
}

fn emit(
    rx: mpsc::Receiver<Option<Target>>,
    driver: mpsc::Sender<Command>,
    target: Target,
) {
    let wait_time: u64 = (1000.0 / FREQUENCY as f64) as u64;
    let wait_duration = time::Duration::from_millis(wait_time);
    let mut target = target;
    let mut command;
    'emitter: loop {
        for received in rx.try_iter() {
            match received {
                None => break 'emitter,
                Some(t) => {
                    target = t;
                }
            }
        }
        command = Command::MoveTo {
            x: Some(target.vector.x),
            y: Some(target.vector.y),
            z: Some(target.vector.z),
            f: Some(target.speed),
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
            vector: Vector3::new(0., 0., 0.),
            speed: 6000.0,
            thread: tx,
        };
        let driver = driver.clone();
        let target = Target {
            vector: Vector3::new(0, 0, 0),
            speed: state.speed as Num,
        };
        thread::spawn(move || emit(rx, driver, target));
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
        println!(":: Speed = {} mm/min\r", self.speed as Num)
    }

    fn update_target(&mut self) {
        let target = Target {
            vector: Vector3::new(
                (self.vector.x * self.speed as f32 / (FREQUENCY * 60.0)) as Num,
                (self.vector.y * self.speed as f32 / (FREQUENCY * 60.0)) as Num,
                (self.vector.z * self.speed as f32 / (FREQUENCY * 60.0)) as Num,
            ),
            speed: self.speed as Num,
        };
        self.thread.send(Some(target)).unwrap();
    }

    fn handle_button(&mut self, button: Button) {
        println!("Button press: {:?}\r", button);
    }

    fn handle_axis(&mut self, axis: gilrs::Axis, value: f32) {
        match axis {
            gilrs::Axis::LeftStickX => {
                self.vector.x = value;
            }
            gilrs::Axis::LeftStickY => {
                self.vector.y = value;
            }
            gilrs::Axis::RightStickY => {
                self.vector.z = value;
            }
            _ => (),
        }
        self.update_target();
    }

    fn handle_key(&mut self, keycode: i32) {
        match keycode as u8 as char {
            'w' => {
                self.speed = self.speed + 100.0;
                if self.speed > MAX_SPEED { self.speed = MAX_SPEED }
                self.print_speed();
            },
            's' => {
                self.speed = self.speed - 100.0;
                if self.speed < MIN_SPEED { self.speed = MIN_SPEED }
                self.print_speed();
            },
            'a' => {
                self.speed = self.speed / 2.0;
                if self.speed < MIN_SPEED { self.speed = MIN_SPEED }
                self.print_speed();
            },
            'd' => {
                self.speed = self.speed * 2.0;
                if self.speed > MAX_SPEED { self.speed = MAX_SPEED }
                self.print_speed();
            },
            _ => ()
        }
        self.update_target();
    }
}

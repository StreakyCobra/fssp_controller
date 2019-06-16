use controller::control::Control;
use driver::command::Command;
use driver::command::Num;
use gilrs;
use gilrs::Button;
use mode::calibration::Calibration;
use mode::Mode;
use na::{Vector3, Vector2};
use std::sync::mpsc;
use std::thread;
use std::time;

const FREQUENCY: f32 = 10.0;
const MAX_TRANSLATION_SPEED: f32 = 60_000.0;
const MIN_TRANSLATION_SPEED: f32 = FREQUENCY * 60.0;
const MAX_ROTATION_SPEED: f32 = 2_700.0;
const MIN_ROTATION_SPEED: f32 = FREQUENCY * 60.0;

#[derive(Debug, Clone)]
struct Target {
    translation: Vector3<Num>,
    translation_speed: Num,
    rotation: Vector2<Num>,
    rotation_speed: Num,
}

#[derive(Debug)]
struct Axis<T> {
    x: T,
    y: T,
    z: T,
    u: T,
    v: T,
}

#[derive(Debug)]
struct Speed<T> {
    translational: T,
    rotational: T,
}

#[derive(Debug)]
pub struct Manual {
    driver: mpsc::Sender<Command>,
    thread: mpsc::Sender<Option<Target>>,
    axis: Axis<f32>,
    speed: Speed<f32>,
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
            x: Some(target.translation.x),
            y: Some(target.translation.y),
            z: Some(target.translation.z),
            f: Some(target.translation_speed),
        };
        driver.send(command).unwrap();
        command = Command::RotateTo {
            u: Some(target.rotation.x),
            v: Some(target.rotation.y),
            f: Some(target.rotation_speed),
        };
        driver.send(command).unwrap();
        thread::sleep(wait_duration);
    }
}

impl Mode for Manual {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetAbsolute).unwrap();
        let (tx, rx) = mpsc::channel();
        let state = Manual {
            driver: driver.clone(),
            thread: tx,
            axis: Axis {
                x: 0.,
                y: 0.,
                z: 0.,
                u: 0.,
                v: 0.,
            },
            speed: Speed {
                translational: 6000.,
                rotational: 2000.,
            }
        };
        let driver = driver.clone();
        let target = Target {
            translation: Vector3::new(0, 0, 0),
            translation_speed: state.speed.translational as Num,
            rotation: Vector2::new(0, 0),
            rotation_speed: state.speed.rotational as Num,
        };
        thread::spawn(move || emit(rx, driver, target));
        return state;
    }

    fn start(&mut self) {
        self.print_state();
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
                if let gilrs::EventType::ButtonChanged { 0: button, 1: value, 2: _ } = event {
                    self.handle_button(button, value)
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
    fn print_state(&mut self) {
        println!(":: Translation speed = {} mm/min\r", self.speed.translational as Num);
        println!(":: Rotation speed = {} deg/min\r", self.speed.rotational as Num);
        println!("----------\r");
    }

    fn update_target(&mut self) {
        let target = Target {
            translation: Vector3::new(
                (self.axis.x * self.speed.translational as f32 / (FREQUENCY * 60.0)) as Num,
                (self.axis.y * self.speed.translational as f32 / (FREQUENCY * 60.0)) as Num,
                (self.axis.z * self.speed.translational as f32 / (FREQUENCY * 60.0)) as Num,
            ),
            translation_speed: self.speed.translational as Num,
            rotation: Vector2::new(
                (self.axis.u * self.speed.rotational as f32 / (FREQUENCY * 60.0)) as Num,
                (self.axis.v * self.speed.rotational as f32 / (FREQUENCY * 60.0)) as Num,
            ),
            rotation_speed: self.speed.rotational as Num,
        };
        self.thread.send(Some(target)).unwrap();
    }

    fn handle_button(&mut self, button: Button, value: f32) {
        let value = value.powf(3.);
        match button {
            gilrs::Button::LeftTrigger2 => {
                self.axis.z = -value;
            },
            gilrs::Button::RightTrigger2 => {
                self.axis.z = value;
            }
            _ => (),
        }
        self.update_target();
    }

    fn handle_axis(&mut self, axis: gilrs::Axis, value: f32) {
        let value = value.powf(3.);
        match axis {
            gilrs::Axis::LeftStickX => {
                self.axis.x = value;
            }
            gilrs::Axis::LeftStickY => {
                self.axis.y = value;
            }
            gilrs::Axis::RightStickX => {
                self.axis.u = value;
            }
            gilrs::Axis::RightStickY => {
                self.axis.v = value;
            }
            _ => (),
        }
        self.update_target();
    }

    fn handle_key(&mut self, keycode: i32) {
        match keycode as u8 as char {
            'w' => {
                update_speed(&mut self.speed.translational, |x| x * 2., MIN_TRANSLATION_SPEED, MAX_TRANSLATION_SPEED);
            },
            's' => {
                update_speed(&mut self.speed.translational, |x| x / 2., MIN_TRANSLATION_SPEED, MAX_TRANSLATION_SPEED);
            },
            'a' => {
                update_speed(&mut self.speed.translational, |x| x - 100., MIN_TRANSLATION_SPEED, MAX_TRANSLATION_SPEED);
            },
            'd' => {
                update_speed(&mut self.speed.translational, |x| x + 100., MIN_TRANSLATION_SPEED, MAX_TRANSLATION_SPEED);
            },
            'i' => {
                update_speed(&mut self.speed.rotational, |x| x * 2., MIN_ROTATION_SPEED, MAX_ROTATION_SPEED);
            },
            'k' => {
                update_speed(&mut self.speed.rotational, |x| x / 2., MIN_ROTATION_SPEED, MAX_ROTATION_SPEED);
            },
            'j' => {
                update_speed(&mut self.speed.rotational, |x| x - 100., MIN_ROTATION_SPEED, MAX_ROTATION_SPEED);
            },
            'l' => {
                update_speed(&mut self.speed.rotational, |x| x + 100., MIN_ROTATION_SPEED, MAX_ROTATION_SPEED);
            },
            _ => ()
        }
        self.print_state();
        self.update_target();
    }
}

fn update_speed<F>(speed: &mut f32, func: F, min: f32, max: f32) -> f32
    where F: Fn(f32) -> f32 {
    let mut result = func(*speed);
    if result > max { result = max }
    if result < min { result = min }
    *speed = result;
    result
}

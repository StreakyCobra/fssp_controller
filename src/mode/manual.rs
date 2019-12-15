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
    axis: Axis<f32>,
    speed: Speed<f32>,
}

enum Event {
    Target(Target),
    Quit,
}

#[derive(Debug, Clone)]
struct Axis<T> {
    x: T,
    y: T,
    z: T,
    u: T,
    v: T,
}

#[derive(Debug, Clone)]
struct Speed<T> {
    translational: T,
    rotational: T,
}

#[derive(Debug)]
pub struct Manual {
    driver: mpsc::Sender<Command>,
    thread: mpsc::Sender<Event>,
    axis: Axis<f32>,
    speed: Speed<f32>,
}

fn integrate(
    rx: mpsc::Receiver<Event>,
    driver: mpsc::Sender<Command>,
    target: Target,
) {
    let wait_time: u64 = (1000.0 / FREQUENCY as f64) as u64;
    let wait_duration = time::Duration::from_millis(wait_time);
    let mut target = target;
    let mut position = Vector3::new(1350., 1800., 400.);
    let mut rotation = Vector2::new(0., 0.);
    let mut command;
    'emitter: loop {
        for received in rx.try_iter() {
            match received {
                Event::Quit => break 'emitter,
                Event::Target(t) => {
                    target = t;
                }
            }
        }

        position.x += target.axis.x * target.speed.translational as f32 / (FREQUENCY * 60.0);
        position.y += target.axis.y * target.speed.translational as f32 / (FREQUENCY * 60.0);
        position.z += target.axis.z * target.speed.translational as f32 / (FREQUENCY * 60.0);
        rotation.x += target.axis.u * target.speed.rotational as f32 / (FREQUENCY * 60.0);
        rotation.y += target.axis.v * target.speed.rotational as f32 / (FREQUENCY * 60.0);
    
        command = Command::MoveTo {
            x: Some(position.x as Num),
            y: Some(position.y as Num),
            z: Some(position.z as Num),
            f: Some(target.speed.translational as Num),
        };
        driver.send(command).unwrap();
        command = Command::RotateTo {
            u: Some(rotation.x as Num),
            v: Some(rotation.y as Num),
            f: Some(target.speed.rotational as Num),
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
            axis: state.axis.clone(),
            speed: state.speed.clone(),
        };
        thread::spawn(move || integrate(rx, driver, target));
        return state;
    }

    fn start(&mut self) {
        self.print_state();
    }

    fn stop(&mut self) {
        self.thread.send(Event::Quit).unwrap();
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
            axis: self.axis.clone(),
            speed: self.speed.clone(),
        };
        self.thread.send(Event::Target(target)).unwrap();
    }

    fn handle_button(&mut self, button: Button, value: f32) {
        let value = response_curve(value);
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
        let value = response_curve(value);
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

fn response_curve(value: f32) -> f32 {
    let mut value = value;
    if value.abs() < 0.05 { 
        value = 0.
    }
    value = value.powf(3.);
    return value
}
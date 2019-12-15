use controller::control::Control;
use driver::command::Num;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use mode::simulation::Simulation;
use mode::Mode;
use std::sync::mpsc;
use std::time;
use std::thread;
use std::cmp::{min, max};

const NUM_MOTORS: usize = 4;
const FREQUENCY: f32 = 10.0;
const MAX_SPEED: f32 = 60_000.0;
const MIN_SPEED: f32 = FREQUENCY * 60.0;

#[derive(Debug, Clone, Copy)]
struct Target {
    motor: usize,
    length: f32,
    speed: f32,
}

#[derive(Debug)]
pub struct Calibration {
    driver: mpsc::Sender<Command>,
    thread: mpsc::Sender<Option<Target>>,
    target: Target
}

fn integrate(
    rx: mpsc::Receiver<Option<Target>>,
    driver: mpsc::Sender<Command>,
    target: Target,
) {
    let wait_time: u64 = (1000.0 / FREQUENCY as f64) as u64;
    let wait_duration = time::Duration::from_millis(wait_time);
    let mut positions: [f32; NUM_MOTORS] = [0.; NUM_MOTORS];
    let mut targets: [Target; NUM_MOTORS] = [target; NUM_MOTORS];
    let mut target = targets[0];
    let mut command;
    'emitter: loop {

        for received in rx.try_iter() {
            match received {
                None => break 'emitter,
                Some(t) => {
                    targets[t.motor as usize] = t;
                    target = targets[t.motor];
                }
            }
        }

        positions[target.motor as usize] += target.length * target.speed / (FREQUENCY * 60.0);
    
        command = Command::MoveMotorTo {
            m: target.motor as Num,
            l: positions[target.motor] as Num,
            f: Some(target.speed as Num),
        };
        driver.send(command).unwrap();
        thread::sleep(wait_duration);
    }
}


impl Mode for Calibration {
    fn init(driver: &mpsc::Sender<Command>) -> Self {
        driver.send(Command::SetAbsolute).unwrap();
        let (tx, rx) = mpsc::channel();
        let target = Target {
            motor: 0,
            length: 0.,
            speed: 10_000.,
        };
        let state = Calibration {
            driver: driver.clone(),
            thread: tx,
            target: target,
        };
        let driver = driver.clone();
        thread::spawn(move || integrate(rx, driver, target));
        return state;
    }

    fn start(&mut self) {
        self.print_state();
    }

    fn stop(&mut self) {
        self.thread.send(None).unwrap();
    }

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
    fn print_state(&mut self) {
        println!(":: Motor = {} \r", self.target.motor);
        println!(":: Speed = {} mm/min\r", self.target.speed as Num);
        println!("----------\r");
    }

    fn update_target(&mut self) {
        self.thread.send(Some(self.target.clone())).unwrap();
    }


    fn handle_button(&mut self, button: Button) {
        match button {
            Button::DPadUp => {
                self.target.length = 0.;
                self.target.motor = min(self.target.motor + 1, NUM_MOTORS - 1);
            }
            Button::DPadDown => {
                self.target.length = 0.;
                self.target.motor = if self.target.motor > 0 {self.target.motor - 1} else {0}
            }
            _ => ()
        }
        self.print_state();
        self.update_target();
    }

    fn handle_axis(&mut self, axis: gilrs::Axis, value: f32) {
        let value = response_curve(value);
        match axis {
            gilrs::Axis::LeftStickY => {
                self.target.length = value;
            }
            _ => (),
        }
        self.update_target();
    }

    fn handle_key(&mut self, keycode: i32) {
        match keycode as u8 as char {
            'w' => {
                update_speed(&mut self.target.speed, |x| x * 2., MIN_SPEED, MAX_SPEED);
            },
            's' => {
                update_speed(&mut self.target.speed, |x| x / 2., MIN_SPEED, MAX_SPEED);
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
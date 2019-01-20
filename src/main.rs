extern crate gilrs;
extern crate nalgebra as na;

mod controller;
mod driver;
mod physics;
mod sensor;
mod simulation;

use controller::connect_controller;
use driver::command::Command;
use driver::connect_driver;
use physics::State;
use sensor::connect_sensor;
use std::{thread, time};

#[derive(Debug)]
pub struct Lander {
    pub state: State,
}

fn main() {
    let controls = connect_controller();
    let commands = connect_driver("localhost:16000");
    let events = connect_sensor("localhost:16001");
    let one_sec = time::Duration::from_secs(1);
    loop {
        for command in controls.try_iter() {
            println!("{:?}", command);
            if let Some(tx) = &commands {
                tx.send(Command::NoOp).unwrap();
            }
        }
        if let Some(rx) = &events {
            for event in rx.try_iter() {
                println!("{:?}", event);
            }
        }
        thread::sleep(one_sec);
    }
}

extern crate gilrs;
extern crate nalgebra as na;

mod controller;
mod driver;
mod physics;
mod simulation;

use controller::start_controller;
use driver::command::Command;
use driver::connect_driver;
use physics::State;
use std::{thread, time};

#[derive(Debug)]
pub struct Lander {
    pub state: State,
}

fn main() {
    let controls = start_controller();
    let commands = connect_driver("localhost:1234");
    let one_sec = time::Duration::from_secs(1);
    loop {
        for received in controls.try_iter() {
            println!("{:?}", received);
            if let Some(tx) = &commands {
                tx.send(Command::NoOp).unwrap();
            }
        }
        thread::sleep(one_sec);
    }
}

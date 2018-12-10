extern crate gilrs;
extern crate nalgebra as na;

mod controller;
mod driver;
mod physics;
mod simulation;

use controller::start_controller;
use physics::State;
use std::{thread, time};

#[derive(Debug)]
pub struct Lander {
    pub state: State,
}

fn main() {
    let rx = start_controller();
    let one_sec = time::Duration::from_secs(1);
    loop {
        for received in rx.try_iter() {
            println!("{:?}", received);
        }
        thread::sleep(one_sec);
    }
}

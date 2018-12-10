extern crate gilrs;
extern crate nalgebra as na;

mod controller;
mod driver;
mod physics;
mod simulation;

use controller::joystick_test;
use physics::State;
use simulation::lander_test;

#[derive(Debug)]
pub struct Lander {
    pub state: State,
}

fn main() {
    lander_test();
    joystick_test();
}

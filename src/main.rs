extern crate gilrs;
extern crate nalgebra as na;

mod controller;
mod driver;
mod mode;
mod physics;
mod sensor;
mod simulation;

use controller::connect_controller;
use driver::connect_driver;
use mode::master_loop;
use sensor::connect_sensor;

fn main() {
    let controls = connect_controller();
    let commands = connect_driver("geneKranz.local:16000");
    let events = connect_sensor("geneKranz.local:16001");

    master_loop(controls, commands, events);
}

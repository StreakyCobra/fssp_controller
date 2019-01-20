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
    // let commands = connect_driver("172.28.252.97:3000");
    // let commands = connect_driver("172.28.252.12:3000");
    let commands = connect_driver("192.168.2.68:3000");
    // let commands = connect_driver("localhost:16000");
    let events = connect_sensor("localhost:16001");

    master_loop(controls, commands, events);
}

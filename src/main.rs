extern crate gilrs;

use gilrs::Gilrs;

fn main() {
    let gilrs = Gilrs::new().unwrap();

    for (_id, gamepad) in gilrs.gamepads() {
        println!("{} is {:?}", gamepad.name(), gamepad.power_info());
    }
}

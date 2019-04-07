extern crate gilrs;
extern crate nalgebra as na;
extern crate ncurses;

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
    let controller = connect_controller();
    let driver = connect_driver("localhost:16000");
    let sensor = connect_sensor("localhost:16001");

    init_ncurses();
    master_loop(controller, driver, sensor);
    close_ncurses();
}

fn init_ncurses() {
    let windows = ncurses::initscr();
    ncurses::nodelay(windows, true);
    ncurses::noecho();
    ncurses::refresh();
}

fn close_ncurses() {
    ncurses::endwin();
}

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
    let controls = connect_controller();
    let commands = connect_driver("geneKranz.local:16000");
    let events = connect_sensor("geneKranz.local:16001");

    init_ncurses();
    master_loop(controls, commands, events);
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

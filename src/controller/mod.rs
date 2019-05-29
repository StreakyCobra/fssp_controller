use controller::control::Control;
use gilrs::Gilrs;
use ncurses;
use std::sync::mpsc;
use std::{thread, time};

pub mod control;

const WAIT_DURATION_MS: u64 = 1;

pub fn connect_controller() -> mpsc::Receiver<Control> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || listen(tx));

    return rx;
}

fn listen(tx: mpsc::Sender<Control>) {
    let wait_duration = time::Duration::from_millis(WAIT_DURATION_MS);
    let mut gilrs = Gilrs::new().unwrap();
    loop {
        if let Some(event) = gilrs.next_event() {
            tx.send(Control::Joystick { event: event }).unwrap();
        }
        let ch = ncurses::getch();
        if ch != -1 {
            tx.send(Control::Keyboard { keycode: ch }).unwrap();
        }
        thread::sleep(wait_duration);
    }
}

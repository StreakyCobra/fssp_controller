use controller::control::Control;
use gilrs::Gilrs;
use std::sync::mpsc;
use std::thread;

pub mod control;

pub fn connect_controller() -> mpsc::Receiver<Control> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || listen(tx));

    return rx;
}

fn listen(tx: mpsc::Sender<Control>) {
    let mut gilrs = Gilrs::new().unwrap();
    loop {
        if let Some(event) = gilrs.next_event() {
            tx.send(Control::Joystick { event: event }).unwrap();
        }
    }
}

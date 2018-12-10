use gilrs::{Event, EventType, Gilrs};

use std::sync::mpsc;
use std::thread;

pub fn start_controller() -> mpsc::Receiver<gilrs::Button> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || listen(tx));

    return rx;
}

fn listen(tx: mpsc::Sender<gilrs::Button>) {
    let mut gilrs = Gilrs::new().unwrap();
    loop {
        if let Some(Event {
            id: _,
            event,
            time: _,
        }) = gilrs.next_event()
        {
            if let EventType::ButtonReleased { 0: button, 1: _ } = event {
                tx.send(button).unwrap();
            }
        }
    }
}

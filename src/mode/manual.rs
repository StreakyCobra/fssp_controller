use controller::control::Control;
use driver::command::Command;
use gilrs;
use gilrs::Button;
use std::sync::mpsc;

pub fn handle(control: Control, commands: &Option<mpsc::Sender<Command>>) {
    if let Some(tx) = commands {
        let Control::Joystick {
            event:
                gilrs::Event {
                    id: _,
                    event: event,
                    time: _,
                },
        } = control;
        if let gilrs::EventType::ButtonReleased { 0: button, 1: _ } = event {
            handle_button(button, &tx)
        }
    }
}

fn handle_button(button: Button, tx: &mpsc::Sender<Command>) {
    match button {
        Button::DPadDown => tx
            .send(Command::MoveTo {
                x: None,
                y: Some(-10),
                z: None,
                f: None,
            })
            .unwrap(),
        Button::DPadUp => tx
            .send(Command::MoveTo {
                x: None,
                y: Some(10),
                z: None,
                f: None,
            })
            .unwrap(),
        Button::DPadLeft => tx
            .send(Command::MoveTo {
                x: Some(-10),
                y: None,
                z: None,
                f: None,
            })
            .unwrap(),
        Button::DPadRight => tx
            .send(Command::MoveTo {
                x: Some(10),
                y: None,
                z: None,
                f: None,
            })
            .unwrap(),
        Button::North => tx
            .send(Command::MoveTo {
                x: None,
                y: None,
                z: Some(10),
                f: None,
            })
            .unwrap(),
        Button::South => tx
            .send(Command::MoveTo {
                x: None,
                y: None,
                z: Some(-10),
                f: None,
            })
            .unwrap(),
        _ => (),
    }
}

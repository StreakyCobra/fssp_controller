use driver::command::Command;
use gilrs::Button;
use std::sync::mpsc;

pub fn handle(button: Button, commands: &Option<mpsc::Sender<Command>>) {
    if let Some(tx) = commands {
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
}

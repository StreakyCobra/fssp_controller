use gilrs::Event;

#[allow(dead_code)]
#[derive(Debug)]
pub enum Control {
    Joystick { event: Event },
    Keyboard { keycode: i32 },
}

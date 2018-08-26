#[derive(Clone, Debug)]
pub enum Control {
    Absolute,
    MoveTo {
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
        f: Option<i32>,
    },
    NoOp,
    Relative,
    SetPosition {
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
        e: Option<i32>,
    },
}

pub trait GCode {
    fn to_gcode(&self) -> String;
}

impl GCode for Control {
    fn to_gcode(&self) -> String {
        match self {
            Control::Absolute => String::from("G90;"),
            Control::MoveTo { x, y, z, f } => {
                let mut params = String::new();
                match x {
                    None => (),
                    Some(val) => params.push_str(&format!("x{} ", val)),
                }
                match y {
                    None => (),
                    Some(val) => params.push_str(&format!("y{} ", val)),
                }
                match z {
                    None => (),
                    Some(val) => params.push_str(&format!("z{} ", val)),
                }
                match f {
                    None => format!("G0 {};", params),
                    Some(val) => format!("G1 {}f{} ;", params, val),
                }
            }
            Control::NoOp => format!(""),
            Control::Relative => String::from("G91;"),
            Control::SetPosition { x, y, z, e } => {
                let mut params = String::new();
                match x {
                    None => (),
                    Some(val) => params.push_str(&format!("x{} ", val)),
                }
                match y {
                    None => (),
                    Some(val) => params.push_str(&format!("y{} ", val)),
                }
                match z {
                    None => (),
                    Some(val) => params.push_str(&format!("z{} ", val)),
                }
                match e {
                    None => (),
                    Some(val) => params.push_str(&format!("e {}", val)),
                }
                format!("G92 {};", params)
            }
        }
    }
}

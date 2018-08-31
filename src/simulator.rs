#[derive(Clone, Debug)]
pub enum Command {
    MoveTo {
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
        f: Option<i32>,
    },
    MoveToHome,
    NoOp,
    Pause {
        p: Option<i32>,
        s: Option<i32>,
    },
    SetAbsolute,
    SetAttachPosition {
        n: i32,
        x: i32,
        y: i32,
        z: i32,
    },
    SetPosition {
        x: Option<i32>,
        y: Option<i32>,
        z: Option<i32>,
        e: Option<i32>,
    },
    SetRelative,
    Shutdown,
}

pub trait GCode {
    fn to_gcode(&self) -> String;
}

impl GCode for Command {
    fn to_gcode(&self) -> String {
        match *self {
            Command::MoveTo { x, y, z, f } => {
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
                    None => format!("G0 {}", params),
                    Some(val) => format!("G1 {}f{}", params, val),
                }
            },
            Command::MoveToHome => format!("G28"),
            Command::NoOp => format!(""),
            Command::Pause { s, p } => {
                let mut params = String::new();
                match s {
                    None => (),
                    Some(val) => params.push_str(&format!("s{}", val)),
                }
                match p {
                    None => (),
                    Some(val) => params.push_str(&format!("p{}", val)),
                }
                format!("G4 {}", params)
            },
            Command::SetAbsolute => String::from("G90"),
            Command::SetAttachPosition { n, x, y, z } => {
                let mut params = String::new();
                params.push_str(&format!("x{} ", x));
                params.push_str(&format!("y{} ", y));
                params.push_str(&format!("z{} ", z));
                let code = match n {
                    1 => "M131",
                    2 => "M132",
                    3 => "M133",
                    _ => panic!("Unsupported attach point number")
                };
                format!("{} {}", code, params)
            },
            Command::SetPosition { x, y, z, e } => {
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
                    Some(val) => params.push_str(&format!("e{}", val)),
                }
                format!("G92 {}", params)
            },
            Command::SetRelative => String::from("G91"),
            Command::Shutdown => String::from("M00"),
        }
    }
}

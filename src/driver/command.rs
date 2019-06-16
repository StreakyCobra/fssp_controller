pub type Num = i32;

#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum Command {
    MoveTo {
        x: Option<Num>,
        y: Option<Num>,
        z: Option<Num>,
        f: Option<Num>,
    },
    RotateTo {
        u: Option<Num>,
        v: Option<Num>,
        f: Option<Num>,
    },
    MoveToHome,
    NoOp,
    Pause {
        p: Option<Num>,
        s: Option<Num>,
    },
    SetAbsolute,
    SetAttachPosition {
        n: Num,
        x: Num,
        y: Num,
        z: Num,
    },
    SetPosition {
        x: Option<Num>,
        y: Option<Num>,
        z: Option<Num>,
        e: Option<Num>,
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
                    Some(val) => params.push_str(&format!("X{} ", val)),
                }
                match y {
                    None => (),
                    Some(val) => params.push_str(&format!("Y{} ", val)),
                }
                match z {
                    None => (),
                    Some(val) => params.push_str(&format!("Z{} ", val)),
                }
                match f {
                    None => format!("G0 {}", params),
                    Some(val) => format!("G1 {}F{}", params, val),
                }
            }
            Command::RotateTo { u, v, f } => {
                let mut params = String::new();
                match u {
                    None => (),
                    Some(val) => params.push_str(&format!("U{} ", val)),
                }
                match v {
                    None => (),
                    Some(val) => params.push_str(&format!("V{} ", val)),
                }
                match f {
                    None => format!("G0 {}", params),
                    Some(val) => format!("G1 {}F{}", params, val),
                }
            }
            Command::MoveToHome => format!("G28"),
            Command::NoOp => format!(""),
            Command::Pause { s, p } => {
                let mut params = String::new();
                match s {
                    None => (),
                    Some(val) => params.push_str(&format!("S{}", val)),
                }
                match p {
                    None => (),
                    Some(val) => params.push_str(&format!("P{}", val)),
                }
                format!("G4 {}", params)
            }
            Command::SetAbsolute => String::from("G90"),
            Command::SetAttachPosition { n, x, y, z } => {
                let mut params = String::new();
                params.push_str(&format!("X{} ", x));
                params.push_str(&format!("Y{} ", y));
                params.push_str(&format!("Z{} ", z));
                let code = match n {
                    1 => "M131",
                    2 => "M132",
                    3 => "M133",
                    _ => panic!("Unsupported attach point number"),
                };
                format!("{} {}", code, params)
            }
            Command::SetPosition { x, y, z, e } => {
                let mut params = String::new();
                match x {
                    None => (),
                    Some(val) => params.push_str(&format!("X{} ", val)),
                }
                match y {
                    None => (),
                    Some(val) => params.push_str(&format!("Y{} ", val)),
                }
                match z {
                    None => (),
                    Some(val) => params.push_str(&format!("Z{} ", val)),
                }
                match e {
                    None => (),
                    Some(val) => params.push_str(&format!("E{}", val)),
                }
                format!("G92 {}", params)
            }
            Command::SetRelative => String::from("G91"),
            Command::Shutdown => String::from("M00"),
        }
    }
}

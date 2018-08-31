use na::Vector3;

#[derive(Debug)]
pub struct Lander {
    mass: i32,
    position: Vector3<i32>,
    velocity: Vector3<i32>,
}

impl Lander {
    pub fn new() -> Lander {
        Lander {
            mass: 0,
            position: Vector3::new(0, 0, 0),
            velocity: Vector3::new(0, 0, 0),
        }
    }
}
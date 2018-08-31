use na::Vector3;

type Num = f64;

#[derive(Debug)]
pub struct State {
    position: Vector3<Num>,
    velocity: Vector3<Num>,
}

impl State {
    pub fn new() -> State {
        State {
            position: Vector3::new(0., 0., 0.),
            velocity: Vector3::new(0., 0., 0.),
        }
    }
}

pub fn integrate(state: &mut State, force: &Vector3<Num>, mass: Num, dt: Num) {
    state.position = state.position + state.velocity * dt;
    state.velocity = state.velocity + force / mass * dt;
}

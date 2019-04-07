use physics;
use physics::State;

use na::Vector3;

#[derive(Debug)]
pub struct Lander {
    pub state: State,
}

pub fn lander_test() {
    let mut lander = Lander {
        state: State::new(),
    };
    let dt = 0.01;
    let mut t = 0.;
    while t <= 10. {
        physics::integrate(&mut lander.state, &Vector3::new(10., 10., 0.), 1., dt);
        t += dt;
        println!("{:} {:?}\r", t, lander);
    }
}

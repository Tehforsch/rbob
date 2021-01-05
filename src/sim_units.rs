use uom::si::f64::{Length, Mass, MassDensity, Time, Velocity};

#[derive(Debug, Clone)]
pub struct SimUnits {
    pub length: Length,
    pub velocity: Velocity,
    pub mass: Mass,
    pub mass_density: MassDensity,
    pub time: Time,
}

impl SimUnits {
    pub fn new(length: Length, velocity: Velocity, mass: Mass) -> SimUnits {
        let mass_density = mass / (length * length * length);
        let time = length / velocity;
        SimUnits {
            length,
            velocity,
            mass,
            mass_density,
            time,
        }
    }
}

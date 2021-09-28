use bevy_ecs::prelude::Bundle;

use crate::physics::Kinematics;

#[derive(Bundle, Default)]
pub struct RocketBundle {
    pub kinematics: Kinematics,
    pub rocket: Rocket,
}

pub struct RocketEntity(pub bevy_ecs::entity::Entity);

pub struct Rocket {
    pub fuel_capacity: f32,
    pub current_fuel_mass: f32,
    pub non_fuel_mass: f32,
    /// how fast the fuel burns
    pub fuel_burn_rate: f32,
    /// how much force per fuel unit
    pub fuel_thrust_factor: f32,
}

impl Default for Rocket {
    fn default() -> Self {
        Rocket {
            fuel_capacity: 900.0,
            current_fuel_mass: 900.0,
            non_fuel_mass: 500.0,
            fuel_burn_rate: 200.0,
            fuel_thrust_factor: 2500.0,
        }
    }
}

impl Rocket {
    pub fn total_mass(&self) -> f32 {
        self.current_fuel_mass + self.non_fuel_mass
    }
}

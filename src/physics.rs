use crate::rocket::Rocket;
use bevy::prelude::*;
use macroquad::prelude::Vec2;

pub struct DT(pub f32);

pub struct Mass(pub f32);

#[derive(Default)]
pub struct Kinematics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

pub fn integration_sys(mut query: Query<&mut Kinematics>, dt: Res<DT>) {
    for mut kinematics in query.iter_mut() {
        let dt = dt.0;
        let vel = kinematics.vel;
        let accel = kinematics.acc;

        kinematics.pos += vel * dt + 0.5 * accel * dt * dt;
        kinematics.vel += accel * dt;
    }
}

pub fn rocket_thrust_sys(mut query: Query<(&mut Kinematics, &mut Rocket)>, dt: Res<DT>) {
    let rockets = query
        .iter_mut()
        .filter(|(_, rocket)| rocket.current_fuel_mass > 0.0);
    for (mut kinematics, mut rocket) in rockets {
        let mass = rocket.total_mass();
        let fuel_burned = rocket.fuel_burn_rate * dt.0;
        let thrust_force = fuel_burned * rocket.fuel_thrust_factor;
        kinematics.acc -= Vec2::new(0.0, thrust_force / mass * crate::THRUST_MULTIPLIER);

        rocket.current_fuel_mass -= fuel_burned;
        rocket.current_fuel_mass = rocket.current_fuel_mass.max(0.0);
    }
}

pub fn rocket_gravity_sys(mut query: Query<(&mut Kinematics, &Rocket)>) {
    for (mut kinematics, _) in query.iter_mut() {
        kinematics.acc.y += crate::GRAVITY;
    }
}

pub fn reset_accel_sys(mut query: Query<&mut Kinematics>) {
    for mut kinematics in query.iter_mut() {
        kinematics.acc = Vec2::new(0.0, 0.0)
    }
}

use crate::rocket::Rocket;
use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::Vec2;

use crate::planet::CelestialBody;
use crate::rocket::RocketCrashed;
use crate::trajectory::Trajectory;

pub struct DT(pub f32);
pub struct Steps(pub usize);
pub struct Mass(pub f32);

#[derive(Default, Copy, Clone)]
pub struct Kinematics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

pub fn integration_sys(mut query: Query<&mut Kinematics, Without<Trajectory>>, dt: Res<DT>) {
    for mut kinematics in query.iter_mut() {
        let dt = dt.0;
        let vel = kinematics.vel;
        let accel = kinematics.acc;

        kinematics.pos += vel * dt + 0.5 * accel * dt * dt;
        kinematics.vel += accel * dt;
    }
}

pub fn rocket_thrust_sys(mut query: Query<(&mut Kinematics, &mut Rocket), Without<Trajectory>>, dt: Res<DT>) {
    let rockets = query
        .iter_mut()
        .filter(|(_, rocket)| rocket.current_fuel_mass > 0.0 && rocket.thrust);
    for (mut kinematics, mut rocket) in rockets {
        let mass = rocket.total_mass();
        let fuel_burned = rocket.fuel_burn_rate * dt.0;
        let thrust_force = fuel_burned * rocket.fuel_thrust_factor;
        let thrust_accel = thrust_force / mass * crate::THRUST_MULTIPLIER;
        kinematics.acc -= thrust_accel * Vec2::new(rocket.angle.sin(), rocket.angle.cos());

        rocket.current_fuel_mass -= fuel_burned;
        rocket.current_fuel_mass = rocket.current_fuel_mass.max(0.0);
    }
}

pub fn calculate_planet_interaction(
    (rocket_kinematics, rocket): (&Kinematics, &Rocket),
    (planet, planet_kinematics): (&CelestialBody, &Kinematics),
) -> (Vec2, f32) {
    use crate::GRAVITY as G;
    let damping_eqn = |x: f32| 0.5 + x.sqrt() / 2.0;

    let r = rocket_kinematics.pos - planet_kinematics.pos;
    assert!(r.length() > planet.radius);

    let _m1 = rocket.total_mass();
    let m2 = planet.mass;

    let a_g = G * m2 / r.length_squared();

    let atmosphere_proportion = r.length() / planet.atmosphere_radius;
    let g_accel = a_g * r.normalize();

    let damping = if atmosphere_proportion < 1.0 {
        damping_eqn(atmosphere_proportion)
    } else {
        1.0
    };

    (g_accel, damping)
}

pub fn rocket_planet_interaction_sys(
    mut query_set: QuerySet<(
        Query<(&Kinematics, &Rocket), Without<Trajectory>>,
        Query<(&mut Kinematics, &Rocket), Without<Trajectory>>,
        Query<(&CelestialBody, &Kinematics)>,
    )>,
    dt: Res<DT>,
    mut rocket_crashed: ResMut<RocketCrashed>,
) {
    let dt = dt.0;

    let rocket_immut_query = query_set.q0();
    let planet_query = query_set.q2();

    let mut rocket_accels: Vec<Vec2> = vec![];
    let mut rocket_dampings: Vec<f32> = vec![];

    for planet_info @ (planet, planet_kinematics) in planet_query.iter() {
        for rocket_info @ (rocket_kinematics, _) in rocket_immut_query.iter() {
            let r = rocket_kinematics.pos - planet_kinematics.pos;
            if r.length() > planet.radius {
                let (accel, damping) = calculate_planet_interaction(rocket_info, planet_info);
                rocket_accels.push(accel);
                rocket_dampings.push(damping);
            } else {
                rocket_crashed.0 = true;
            }
        }
    }

    let rocket_mut_query = query_set.q1_mut();
    for (((mut rocket_kinematics, _), g_accel), atm_damping) in rocket_mut_query
        .iter_mut()
        .zip(rocket_accels.iter())
        .zip(rocket_dampings.iter())
    {
        rocket_kinematics.acc -= *g_accel;
        rocket_kinematics.vel *= (*atm_damping).powf(dt);
    }
}

pub fn reset_accel_sys(mut query: Query<&mut Kinematics>) {
    for mut kinematics in query.iter_mut() {
        kinematics.acc = Vec2::new(0.0, 0.0)
    }
}

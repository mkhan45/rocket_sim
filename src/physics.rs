use crate::rocket::Rocket;
use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::Vec2;

use crate::planet::CelestialBody;
use crate::rocket::RocketCrashed;
use crate::trajectory::Trajectory;

pub mod offset;

pub struct DT(pub f32);
pub struct Steps(pub usize);
pub struct Mass(pub f32);

#[derive(Default, Copy, Clone)]
pub struct Kinematics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

macro_rules! generate_integration_systems {
    ($filter:ident, $name:ident) => {
        pub fn $name(mut query: Query<&mut Kinematics, $filter<Trajectory>>, dt: Res<DT>) {
            for mut kinematics in query.iter_mut() {
                let dt = dt.0;
                let vel = kinematics.vel;
                let accel = kinematics.acc;

                kinematics.pos += vel * dt + 0.5 * accel * dt * dt;
                kinematics.vel += accel * dt;
            }
        }
    };
}

generate_integration_systems!(Without, integration_sys);
generate_integration_systems!(With, trajectory_integration_sys);

// TODO: Rocket accel/gravity wrong direction
macro_rules! generate_rocket_thrust_systems {
    ($filter:ident, $name:ident) => {
        pub fn $name(
            mut query: Query<(&mut Kinematics, &mut Rocket), $filter<Trajectory>>,
            dt: Res<DT>,
        ) {
            let rockets = query
                .iter_mut()
                .filter(|(_, rocket)| rocket.current_fuel_mass > 0.0 && rocket.thrust > 0.0);
            for (mut kinematics, mut rocket) in rockets {
                let mass = rocket.total_mass();
                let fuel_burned = rocket.fuel_burn_rate * dt.0 * rocket.thrust;
                let thrust_force = fuel_burned * rocket.fuel_thrust_factor;
                let thrust_accel = thrust_force / mass * crate::THRUST_MULTIPLIER;
                kinematics.acc += thrust_accel * Vec2::new(rocket.angle.sin(), rocket.angle.cos());

                rocket.current_fuel_mass -= fuel_burned;
                rocket.current_fuel_mass = rocket.current_fuel_mass.max(0.0);
            }
        }
    };
}

generate_rocket_thrust_systems!(Without, rocket_thrust_sys);
generate_rocket_thrust_systems!(With, trajectory_rocket_thrust_sys);

macro_rules! generate_planet_interaction_systems {
    ($filter:ident, $name:ident) => {
        pub fn $name(
            mut query_set: QuerySet<(
                Query<(&Kinematics, &Rocket), $filter<Trajectory>>,
                Query<(&mut Kinematics, &Rocket), $filter<Trajectory>>,
                Query<(&CelestialBody, &Kinematics)>,
            )>,
            dt: Res<DT>,
        ) {
            let dt = dt.0;

            let rocket_immut_query = query_set.q0();
            let planet_query = query_set.q2();

            let mut rocket_accels: Vec<Vec2> = vec![];
            let mut rocket_dampings: Vec<f32> = vec![];

            for planet_info @ (planet, planet_kinematics) in planet_query.iter() {
                for (i, rocket_info @ (rocket_kinematics, _)) in
                    rocket_immut_query.iter().enumerate()
                {
                    rocket_accels.push(Vec2::new(0.0, 0.0));
                    rocket_dampings.push(1.0);
                    let r = rocket_kinematics.pos - planet_kinematics.pos;
                    if r.length() > planet.radius {
                        let (accel, damping) =
                            calculate_planet_interaction(rocket_info, planet_info);
                        rocket_accels[i] += accel;
                        rocket_dampings[i] *= damping;
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
    };
}

generate_planet_interaction_systems!(Without, rocket_planet_interaction_sys);
generate_planet_interaction_systems!(With, trajectory_planet_interaction_sys);

pub fn rocket_crash_sys(
    rocket_query: Query<&Kinematics, (With<Rocket>, Without<Trajectory>)>,
    planet_query: Query<(&Kinematics, &CelestialBody)>,
    mut rocket_crashed: ResMut<RocketCrashed>,
) {
    for rocket_kinematics in rocket_query.iter() {
        for (planet_kinematics, planet) in planet_query.iter() {
            if (rocket_kinematics.pos - planet_kinematics.pos).length() < planet.radius {
                rocket_crashed.0 = true;
                break;
            }
        }
    }
}

macro_rules! generate_reset_accel_systems {
    ($filter:ident, $name:ident) => {
        pub fn $name(mut query: Query<&mut Kinematics, $filter<Trajectory>>) {
            for mut kinematics in query.iter_mut() {
                kinematics.acc = Vec2::new(0.0, 0.0)
            }
        }
    };
}

generate_reset_accel_systems!(Without, reset_accel_sys);
generate_reset_accel_systems!(With, trajectory_reset_accel_sys);

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

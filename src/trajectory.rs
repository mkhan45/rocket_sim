use std::collections::VecDeque;

use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

use crate::physics::{Kinematics, calculate_planet_interaction, DT};
use crate::planet::CelestialBody;
use crate::rocket::{Rocket, RocketEntity};

pub struct Trajectory{
    pub points: VecDeque<Vec2>,
    pub max_len: usize,
    pub valid: bool,
}

impl Trajectory {
    pub fn new(max_len: usize) -> Self {
        Trajectory {
            points: VecDeque::with_capacity(max_len),
            max_len,
            valid: false,
        }
    }
}

pub fn trajectory_calculation_sys(
    query_set: QuerySet<(
        Query<(&mut Kinematics, &mut Trajectory, &Rocket)>,
        Query<(&CelestialBody, &Kinematics)>,
        Query<(&Kinematics, &Rocket)>,
    )>,
    dt: Res<DT>,
    rocket_entity: Res<RocketEntity>,
) {
    let dt = dt.0;
    let rocket_mut_query = query_set.q0();
    let planet_query = query_set.q1();
    let main_rocket_query = query_set.q2();

    let (main_rocket_kinematics, main_rocket) = main_rocket_query.get(rocket_entity.0).unwrap();

    unsafe {
        for (mut kinematics, mut trajectory, rocket) in rocket_mut_query.iter_unsafe() {
            trajectory.valid = !main_rocket.thrust;
            if !trajectory.valid {
                *kinematics = *main_rocket_kinematics;
                trajectory.points.clear();
            }

            while trajectory.points.len() >= trajectory.max_len {
                trajectory.points.pop_front();
            }

            let mut rocket_accels = vec![];
            let mut rocket_dampings = vec![];
            let mut iterations = 0;
            let max_iterations = 250;
            while trajectory.points.len() < trajectory.max_len && iterations < max_iterations {
                iterations += 1;
                for planet_info@(planet, planet_kinematics) in planet_query.iter() {
                    if (kinematics.pos - planet_kinematics.pos).length() > planet.radius {
                        let (accel, damping) = calculate_planet_interaction((&kinematics, rocket), planet_info);
                        rocket_accels.push(accel);
                        rocket_dampings.push(damping);
                    }
                }

                for (accel, damping) in rocket_accels.iter().zip(rocket_dampings.iter()) {
                    kinematics.acc -= *accel;
                    kinematics.vel *= damping.powf(dt / max_iterations as f32);
                }

                let vel = kinematics.vel;
                let accel = kinematics.acc;

                kinematics.pos += vel * dt + 0.5 * accel * dt * dt;
                kinematics.vel += accel * dt;
                kinematics.acc = Vec2::new(0.0, 0.0);

                trajectory.points.push_back(kinematics.pos);
            }
        }
    }
}

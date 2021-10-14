use std::collections::VecDeque;

use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

use crate::main_state::MainState;
use crate::physics::{Kinematics, Steps, DT};
use crate::rocket::{Rocket, RocketEntity};

pub struct Trajectory {
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

pub fn _inspect_trajectory_pos_sys(query: Query<&Kinematics, With<Trajectory>>) {
    for kinematics in query.iter() {
        dbg!(kinematics.pos);
    }
}

impl MainState {
    pub fn add_trajectory_points(&mut self) {
        self.world.insert_resource(DT(1.0 / 1.0));
        let steps = self.world.get_resource::<Steps>().unwrap().0;
        let (main_rocket_kinematics, main_rocket) = {
            let rocket_entity = self.world.get_resource::<RocketEntity>().unwrap().0;
            let mut kinematics_query = self.world.query::<&Kinematics>();
            let mut rocket_query = self.world.query::<&Rocket>();
            (
                *kinematics_query.get(&self.world, rocket_entity).unwrap(),
                *rocket_query.get(&self.world, rocket_entity).unwrap(),
            )
        };

        {
            let mut trajectory_query = self
                .world
                .query::<(&mut Trajectory, &mut Kinematics, &mut Rocket)>();
            for (mut trajectory, mut kinematics, mut rocket) in
                trajectory_query.iter_mut(&mut self.world)
            {
                if rocket.thrust > 0.0 {
                    trajectory.valid = false;
                }
                if !trajectory.valid {
                    trajectory.valid = true;
                    trajectory.points.clear();
                    *kinematics = main_rocket_kinematics;
                    *rocket = main_rocket;
                } else {
                    trajectory.points.pop_front();
                }
            }
        }

        let mut trajectory_query = self.world.query::<(&mut Trajectory, &Kinematics)>();
        let start_time = get_time();
        let mut trajectory_filled = false;
        while get_time() - start_time < 0.005 {
            if trajectory_filled {
                // self.world.insert_resource(DT(1.0 / 60.0));
                break;
            }
            self.trajectory_schedule.run(&mut self.world);
            for (mut trajectory, kinematics) in trajectory_query.iter_mut(&mut self.world) {
                if trajectory.points.len() == trajectory.max_len - 1 {
                    trajectory_filled = true;
                    break;
                }
                if trajectory.points.len() < trajectory.max_len - 1 {
                    trajectory.points.push_back(kinematics.pos);
                }
            }
        }
        self.world.insert_resource(DT(1.0 / 60.0));
    }
}

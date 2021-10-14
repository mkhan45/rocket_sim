use std::collections::VecDeque;

use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

use crate::main_state::MainState;
use crate::physics::{Kinematics, Steps, DT};
use crate::rocket::{Rocket, RocketEntity};

#[derive(Copy, Clone)]
pub struct TrajectorySyncClock {
    pub tick: usize,
    pub needed_ticks: usize,
}

impl Default for TrajectorySyncClock {
    fn default() -> Self {
        TrajectorySyncClock {
            tick: 0,
            needed_ticks: 30,
        }
    }
}

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
        let mut clock = self.world.get_resource::<TrajectorySyncClock>().unwrap().clone();
        self.world.insert_resource(DT(1.0 / 60.0 * clock.needed_ticks as f32));

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

        let mut trajectory_len_diff = {
            let mut trajectory_query = self
                .world
                .query::<(&mut Trajectory, &mut Kinematics, &mut Rocket)>();
            let (mut trajectory, mut kinematics, mut rocket) =
                trajectory_query.iter_mut(&mut self.world).next().unwrap();

            if rocket.thrust > 0.0 {
                trajectory.valid = false;
            } else {
                trajectory.valid = true;
            }

            if !trajectory.valid {
                trajectory.points.clear();
                *kinematics = main_rocket_kinematics;
                *rocket = main_rocket;
            } else if clock.tick * steps >= clock.needed_ticks {
                clock.tick = 0;
                trajectory.points.pop_front();
            }

            clock.tick += 1;
            trajectory.max_len - trajectory.points.len()
        };
        self.world.insert_resource(clock);

        let mut trajectory_query = self.world.query::<(&mut Trajectory, &Kinematics)>();
        let start_time = get_time();
        if trajectory_len_diff == 1 {
            self.world.insert_resource(DT(steps as f32 * 1.0 / 60.0));
        }
        while get_time() - start_time < 0.005 && trajectory_len_diff > 0 {
            for _ in 0..5 {
                self.trajectory_schedule.run(&mut self.world);
            }
            let (mut trajectory, kinematics) =
                trajectory_query.iter_mut(&mut self.world).next().unwrap();
            trajectory.points.push_back(kinematics.pos);
            trajectory_len_diff -= 1;
        }
        self.world.insert_resource(DT(1.0 / 60.0));
    }
}

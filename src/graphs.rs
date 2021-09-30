use crate::physics::Kinematics;
use crate::rocket::Rocket;
use bevy_ecs::prelude::*;

use std::collections::VecDeque;

pub struct SpeedGraph(pub VecDeque<f32>);

const MAX_POINTS: usize = 60 * 30;

pub fn rocket_graph_sys(mut query: Query<(&Rocket, &Kinematics, &mut SpeedGraph)>) {
    for (_, kinematics, mut speed_graph) in query.iter_mut() {
        speed_graph.0.push_back(kinematics.vel.length());

        while speed_graph.0.len() > MAX_POINTS {
            speed_graph.0.pop_front();
        }
    }
}

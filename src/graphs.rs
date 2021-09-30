use bevy_ecs::prelude::*;
use crate::physics::Kinematics;
use crate::rocket::Rocket;

pub struct SpeedGraph(pub Vec<f32>);

pub fn rocket_graph_sys(mut query: Query<(&Rocket, &Kinematics, &mut SpeedGraph)>) {
    for (_, kinematics, mut speed_graph) in query.iter_mut() {
        speed_graph.0.push(-kinematics.vel.y);
    }
}

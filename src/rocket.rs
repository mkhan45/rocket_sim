use bevy::prelude::Bundle;

use crate::physics::Kinematics;

#[derive(Bundle, Default)]
pub struct RocketBundle {
    pub kinematics: Kinematics,
}

pub struct RocketEntity(pub bevy::ecs::entity::Entity);

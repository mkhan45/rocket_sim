use macroquad::prelude::*;
use bevy::prelude::Bundle;

use crate::physics::Kinematics;

#[derive(Bundle)]
pub struct RocketBundle {
    kinematics: Kinematics,
}

impl Default for RocketBundle {
    fn default() -> Self {
        RocketBundle {
            kinematics: Kinematics {
                pos: Vec2::new(screen_width() / 2.0, screen_height() * 0.8),
                vel: Vec2::new(0.0, 0.0),
                acc: Vec2::new(0.0, -1000.0),
            },
        }
    }
}

pub struct RocketEntity(pub bevy::ecs::entity::Entity);

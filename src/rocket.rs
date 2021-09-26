use bevy::math::Vec2;
use bevy::ecs::entity::Entity;
use macroquad::prelude::*;

pub struct Rocket {
    pub pos: Vec2,
    pub vel: Vec2,
}

impl Rocket {
    pub fn new() -> Self {
        Rocket {
            pos: Vec2::new(screen_width() / 2.0, screen_height() * 0.8),
            vel: Vec2::new(0.0, -100.0),
        }
    }
}

use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::Vec2;

use crate::physics::Kinematics;
use crate::rocket::RocketEntity;

pub struct UniverseOffset {
    pub offset: Vec2,
    pub max_rocket_radius: f32,
}

impl Default for UniverseOffset {
    fn default() -> Self {
        UniverseOffset {
            offset: Vec2::new(0.0, 0.0),
            max_rocket_radius: 50_000.0,
        }
    }
}

pub fn update_offset_sys(
    mut query_set: QuerySet<(Query<&mut Kinematics>, Query<&Kinematics>)>,
    rocket_entity: Res<RocketEntity>,
    mut offset: ResMut<UniverseOffset>,
) {
    let rocket_pos = query_set.q1().get(rocket_entity.0).unwrap().pos;
    if rocket_pos.length_squared() > offset.max_rocket_radius.powi(2) {
        offset.offset += rocket_pos;
        for mut kinematics in query_set.q0_mut().iter_mut() {
            kinematics.pos -= rocket_pos;
        }
    }
}

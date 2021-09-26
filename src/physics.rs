use bevy::prelude::*;
use macroquad::prelude::Vec2;

pub struct DT(pub f32);

pub struct Mass(pub f32);

pub struct Kinematics {
    pub pos: Vec2,
    pub vel: Vec2,
    pub acc: Vec2,
}

pub fn integration_sys(mut query: Query<&mut Kinematics>, dt: Res<DT>) {
    for mut kinematics in query.iter_mut() {
        let accel = kinematics.acc;
        kinematics.vel += accel * dt.0 * dt.0 * 0.5;

        let vel = kinematics.vel;
        kinematics.pos += vel * dt.0;
    }
}

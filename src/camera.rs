// use macroquad::prelude::{Vec2, Rect};

use bevy::prelude::*;
use macroquad::prelude::{set_camera, Camera2D, Rect};

pub struct CameraRes(pub Camera2D);
impl Default for CameraRes {
    fn default() -> Self {
        CameraRes(Camera2D::from_display_rect(Rect::new(
            -crate::SCREEN_WIDTH / 2.0,
            -crate::SCREEN_HEIGHT,
            crate::SCREEN_WIDTH * 2.0,
            crate::SCREEN_HEIGHT * 2.0,
        )))
    }
}

pub fn update_camera_sys(camera: Res<CameraRes>) {
    if camera.is_changed() {
        set_camera(&camera.0);
    }
}

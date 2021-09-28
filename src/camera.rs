// use macroquad::prelude::{Vec2, Rect};

use bevy::prelude::*;
use macroquad::prelude::{screen_height, screen_width, set_camera, Camera2D, Rect};

pub struct CameraRes {
    pub camera: Camera2D,
    pub screen_size: Vec2,
}

impl Default for CameraRes {
    fn default() -> Self {
        CameraRes {
            camera: Camera2D::from_display_rect(Rect::new(
                -crate::SCREEN_WIDTH / 2.0,
                -crate::SCREEN_HEIGHT,
                crate::SCREEN_WIDTH * 2.0,
                crate::SCREEN_HEIGHT * 2.0,
            )),
            screen_size: Vec2::new(screen_width(), screen_height()),
        }
    }
}

pub fn update_camera_sys(mut camera_res: ResMut<CameraRes>) {
    let screen_width_change = screen_width() / camera_res.screen_size.x;
    let screen_height_change = screen_height() / camera_res.screen_size.y;

    camera_res.screen_size.x = screen_width();
    camera_res.screen_size.y = screen_height();

    camera_res.camera.zoom.x /= screen_width_change;
    camera_res.camera.zoom.y /= screen_height_change;

    set_camera(&camera_res.camera);
}

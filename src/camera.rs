use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::{
    screen_height, screen_width, set_camera, Camera2D, Rect, Vec2,
};

pub struct CameraRes {
    pub camera: Camera2D,
    pub screen_size: Vec2,
}

impl Default for CameraRes {
    fn default() -> Self {
        let display_rect = Rect::new(
            -crate::SCREEN_WIDTH / 2.0,
            0.0,
            crate::SCREEN_WIDTH,
            crate::SCREEN_HEIGHT,
        );

        CameraRes {
            camera: Camera2D::from_display_rect(display_rect),
            screen_size: Vec2::new(screen_width(), screen_height()),
        }
    }
}

pub fn update_camera_sys(mut camera_res: ResMut<CameraRes>) {
    let screen_height_change = screen_height() / camera_res.screen_size.y;
    let aspect_ratio = camera_res.screen_size.x / camera_res.screen_size.y;

    camera_res.screen_size.x = screen_width();
    camera_res.screen_size.y = screen_height();

    camera_res.camera.zoom.y /= screen_height_change;
    camera_res.camera.zoom.x = camera_res.camera.zoom.y / aspect_ratio;

    set_camera(&camera_res.camera);
}

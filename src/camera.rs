use crate::physics::Kinematics;
use crate::rocket::RocketEntity;
use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

pub struct CameraRes {
    pub camera: Camera2D,
    pub screen_size: Vec2,
}

impl Default for CameraRes {
    fn default() -> Self {
        let display_rect = Rect::new(
            -crate::SCREEN_WIDTH / 2.0,
            -crate::SCREEN_HEIGHT,
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

pub fn camera_follow_sys(
    mut camera_res: ResMut<CameraRes>,
    rocket_entity: Res<RocketEntity>,
    kinematics: Query<&Kinematics>,
) {
    let rocket_entity = rocket_entity.0;
    let rocket_kinematics = kinematics.get(rocket_entity).unwrap();

    camera_res.camera.target = rocket_kinematics.pos;

    let t = get_time();
    let x_offset =
        (t.cos() + (t * 1.7).cos() + (t / 3.5).cos() + (t - 2.0).sin() + (t / 2.1).sin()) as f32;
    let y_offset =
        (t.sin() + (t * 1.5).sin() + (t / 3.1).sin() + (t + 1.0).cos() + (t / 1.5).cos()) as f32;
    camera_res.camera.target.x += x_offset * camera_res.screen_size.x / 250.0;
    camera_res.camera.target.y += y_offset * camera_res.screen_size.y / 250.0;
}

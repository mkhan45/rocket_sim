use egui_macroquad::macroquad::prelude::*;

use crate::camera::CameraRes;
use crate::physics::Kinematics;
use crate::planet::CelestialBody;

use bevy_ecs::prelude::*;

pub struct MapRes {
    pub position: Vec2,
    pub scale: Vec2,
    pub shown: bool,
}

impl Default for MapRes {
    fn default() -> Self {
        MapRes {
            position: Vec2::new(0.0, 0.0),
            scale: Vec2::new(1.0, 1.0),
            shown: false,
        }
    }
}

pub fn draw_map_sys(
    map_res: ResMut<MapRes>,
    camera_res: Res<CameraRes>,
    planet_query: Query<(&CelestialBody, &Kinematics)>,
) {
    if map_res.shown {
        let width = crate::SCREEN_WIDTH * 0.8;
        let height = crate::SCREEN_HEIGHT * 0.8;
        draw_rectangle(
            camera_res.camera.target.x - width / 2.0,
            camera_res.camera.target.y - height / 2.0,
            width,
            height,
            BLACK,
        );

        draw_rectangle_lines(
            camera_res.camera.target.x - width / 2.0,
            camera_res.camera.target.y - height / 2.0,
            width,
            height,
            1.0,
            RED,
        );

        // rocket
        draw_circle(
            camera_res.camera.target.x,
            camera_res.camera.target.y,
            crate::SCREEN_WIDTH / 150.0,
            RED,
        );

        for (_, kinematics) in planet_query.iter() {
            let camera_pos = camera_res.camera.target;
            let offset = camera_pos - kinematics.pos;
            let pos = offset / 1000.0 + camera_pos;

            if ((camera_pos.x - width / 2.0)..(camera_pos.x + height / 2.0)).contains(&pos.x)
                && ((camera_pos.y - height / 2.0)..(camera_pos.y + height / 2.0)).contains(&pos.y)
            {
                draw_circle(pos.x, pos.y, 1.0, WHITE);
            }
        }
    }
}

pub fn map_input_sys(mut map_res: ResMut<MapRes>) {
    if is_key_pressed(KeyCode::M) {
        map_res.shown = !map_res.shown;
    }
}

// pub fn update_trajectories_sys() {
// }

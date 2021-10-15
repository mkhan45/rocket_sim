use egui_macroquad::macroquad::prelude::*;

use crate::camera::CameraRes;
use crate::physics::Kinematics;
use crate::planet::CelestialBody;
use crate::texture::Textures;
use crate::trajectory::Trajectory;

use crate::rocket::{draw_rocket, Rocket, RocketEntity};

use bevy_ecs::prelude::*;

pub struct MapRes {
    pub position: Vec2,
    pub scale: f32,
    pub shown: bool,
}

impl Default for MapRes {
    fn default() -> Self {
        MapRes {
            position: Vec2::new(0.0, 0.0),
            scale: 1.0,
            shown: false,
        }
    }
}

pub fn draw_map_sys(
    map_res: ResMut<MapRes>,
    camera_res: Res<CameraRes>,
    planet_query: Query<(&CelestialBody, &Kinematics)>,
    trajectory_query: Query<&Trajectory>,
    textures: Res<Textures>,
    rocket_query: Query<&Rocket>,
    rocket_entity: Res<RocketEntity>,
) {
    let scale: f32 = 1.0 / 20_000.0 / map_res.scale;
    let camera_pos = camera_res.camera.target;

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
            0.01,
            RED,
        );

        // rocket
        if map_res.scale > 2.0 {
            // red dot
            draw_circle(
                camera_res.camera.target.x,
                camera_res.camera.target.y,
                crate::SCREEN_WIDTH / 250.0,
                RED,
            );
        } else {
            // rocket texture
            let rocket_size = crate::SCREEN_WIDTH / 125.0 / map_res.scale;
            let rocket = rocket_query.get(rocket_entity.0).unwrap();
            draw_rocket(
                &camera_res.camera.target,
                rocket.angle,
                rocket.thrust > 0.0 && rocket.current_fuel_mass > 0.0,
                &textures,
                rocket_size,
            );
        }

        let (x_bounds, y_bounds) = {
            let x_bounds = (camera_pos.x - width / 2.0)..(camera_pos.x + width / 2.0);
            let y_bounds = (camera_pos.y - height / 2.0)..(camera_pos.y + height / 2.0);
            (x_bounds, y_bounds)
        };

        let in_map = |point: &Vec2| x_bounds.contains(&point.x) && y_bounds.contains(&point.y);

        for (planet, kinematics) in planet_query.iter() {
            let mut offset = camera_pos - kinematics.pos;
            offset.y *= -1.0;
            let pos = offset * scale + camera_pos;

            let radius_vec = Vec2::splat(planet.radius * scale);
            if in_map(&(pos - radius_vec)) || in_map(&(pos + radius_vec)) {
                let size = planet.radius * 2.0 * scale;
                draw_texture_ex(
                    textures[planet.texture],
                    pos.x - size / 2.0,
                    pos.y - size / 2.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(Vec2::new(size, size)),
                        ..DrawTextureParams::default()
                    },
                );
            }
        }

        for trajectory in trajectory_query.iter() {
            let fst_iter = trajectory.points.iter();
            let snd_iter = trajectory.points.iter().skip(1);

            for (fst, snd) in fst_iter.zip(snd_iter) {
                let mut fst_offset = camera_pos - *fst;
                fst_offset.y *= -1.0;

                let mut snd_offset = camera_pos - *snd;
                snd_offset.y *= -1.0;

                let fst = fst_offset * scale + camera_pos;
                let snd = snd_offset * scale + camera_pos;

                if in_map(&fst) && in_map(&snd) {
                    draw_line(fst.x, fst.y, snd.x, snd.y, 0.005 / map_res.scale, GREEN);
                }
            }
        }
    }
}

pub fn map_input_sys(mut map_res: ResMut<MapRes>) {
    if is_key_pressed(KeyCode::M) {
        map_res.shown = !map_res.shown;
    }

    if is_key_down(KeyCode::Up) {
        map_res.scale = map_res.scale + 0.25;
    }

    if is_key_down(KeyCode::Down) {
        map_res.scale = (map_res.scale - 0.25).max(0.5);
    }
}

use crate::physics::Kinematics;
use crate::rocket::{Altitude, RocketEntity};
use crate::texture::{TextureName, Textures};
use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

pub struct CelestialBody {
    pub radius: f32,
    pub mass: f32,
    pub atmosphere_radius: f32,
    pub atmosphere_color: Color,
    pub texture: TextureName,
}

pub fn add_planets(world: &mut World) {
    world
        .spawn()
        .insert(CelestialBody {
            radius: 6000.0,
            atmosphere_radius: 6700.0,
            mass: 600_000.0,
            atmosphere_color: SKYBLUE,
            texture: TextureName::Earth,
        })
        .insert(Kinematics::default());
}

pub fn draw_atmosphere_sys(
    altitude_query: Query<&Altitude>,
    planet_query: Query<&CelestialBody>,
    rocket_entity: Res<RocketEntity>,
) {
    let rocket_altitude = altitude_query.get(rocket_entity.0).unwrap();
    let planet = planet_query.get(rocket_altitude.closest_planet).unwrap();

    if rocket_altitude.height < planet.atmosphere_radius {
        let base_color = planet.atmosphere_color.to_vec();
        let atmosphere_proportion =
            rocket_altitude.height / (planet.atmosphere_radius - planet.radius);
        let new_color = base_color * (1.0 - atmosphere_proportion);
        clear_background(Color::from_vec(new_color));
    } else {
        clear_background(BLACK);
    }
}

fn draw_planet(planet: &CelestialBody, kinematics: &Kinematics, textures: &Textures) {
    let size = planet.radius * 2.0;
    draw_texture_ex(
        textures[planet.texture],
        kinematics.pos.x - size / 2.0,
        kinematics.pos.y - size / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(size, size)),
            flip_y: true,
            ..DrawTextureParams::default()
        },
    );
}

pub fn draw_planet_sys(query: Query<(&CelestialBody, &Kinematics)>, textures: Res<Textures>) {
    for (planet, kinematics) in query.iter() {
        draw_planet(planet, kinematics, &textures);
    }
}

use crate::physics::Kinematics;
use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

pub struct CelestialBody {
    pub radius: f32,
    pub mass: f32,
    pub atmosphere_radius: f32,
    pub atmosphere_color: Color,
    // TODO
    // pub texture: idk,
}

pub fn add_planets(world: &mut World) {
    world
        .spawn()
        .insert(CelestialBody {
            radius: 6000.0,
            mass: 600_000.0,
            atmosphere_radius: 100_000.0,
            atmosphere_color: SKYBLUE,
        })
        .insert(Kinematics::default());
}

pub fn draw_planet(planet: &CelestialBody, kinematics: &Kinematics) {
    let atmosphere_color_vec = planet.atmosphere_color.to_vec();
    let atmosphere_color_incr = atmosphere_color_vec / 5.0;

    let atmosphere_radius_incr = (planet.atmosphere_radius - planet.radius) / 5.0;

    for i in 1..5 {
        draw_poly(
            kinematics.pos.x,
            kinematics.pos.y,
            250,
            planet.radius + atmosphere_radius_incr * i as f32,
            0.0,
            Color::from_vec(atmosphere_color_vec - atmosphere_color_incr * (i - 1) as f32),
        );
    }

    draw_poly(
        kinematics.pos.x,
        kinematics.pos.y,
        150,
        planet.radius,
        0.0,
        GREEN,
    );
}

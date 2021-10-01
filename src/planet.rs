use crate::physics::Kinematics;
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
            mass: 600_000.0,
            atmosphere_radius: 100_000.0,
            atmosphere_color: SKYBLUE,
            texture: TextureName::Earth,
        })
        .insert(Kinematics::default());
}

fn draw_planet(planet: &CelestialBody, kinematics: &Kinematics, textures: &Textures) {
    draw_radial_gradient(
        kinematics.pos.x,
        kinematics.pos.y,
        planet.radius,
        planet.atmosphere_radius,
        planet.atmosphere_color,
        BLACK,
    );

    let size = planet.radius * 2.0;
    draw_texture_ex(
        textures[planet.texture],
        kinematics.pos.x - size / 2.0,
        kinematics.pos.y - size / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(size, size)),
            ..DrawTextureParams::default()
        },
    );
}

pub fn draw_planet_sys(query: Query<(&CelestialBody, &Kinematics)>, textures: Res<Textures>) {
    for (planet, kinematics) in query.iter() {
        draw_planet(planet, kinematics, &textures);
    }
}

fn draw_radial_gradient(
    x: f32,
    y: f32,
    inner_radius: f32,
    outer_radius: f32,
    center: Color,
    edges: Color,
) {
    // Modified draw_poly()
    // https://github.com/not-fl3/macroquad/blob/432d383f35dbec9cd726acfa84d850c44d39e0c1/src/shapes.rs#L87-L111
    //
    // Advised by PSteinhaus

    let sides = 250;
    let rotation = 0.0f32;

    let mut vertices = Vec::<Vertex>::with_capacity(sides as usize + 2);
    let mut indices = Vec::<u16>::with_capacity(sides as usize * 3);

    let rot = rotation.to_radians();
    vertices.push(Vertex::new(x, y, 0., 0., 0., center));
    for i in 0..(sides * (inner_radius / outer_radius) as usize) {
        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let vertex = Vertex::new(
            x + inner_radius * rx,
            y + inner_radius * ry,
            0.,
            rx,
            ry,
            center,
        );

        vertices.push(vertex);

        if i != sides {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }

    for i in 0..sides + 1 {
        let rx = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).cos();
        let ry = (i as f32 / sides as f32 * std::f32::consts::PI * 2. + rot).sin();

        let vertex = Vertex::new(
            x + outer_radius * rx,
            y + outer_radius * ry,
            0.,
            rx,
            ry,
            edges,
        );

        vertices.push(vertex);

        if i != sides {
            indices.extend_from_slice(&[0, i as u16 + 1, i as u16 + 2]);
        }
    }

    unsafe {
        let gl = egui_macroquad::macroquad::window::get_internal_gl().quad_gl;
        gl.texture(None);
        gl.draw_mode(DrawMode::Triangles);
        gl.geometry(&vertices, &indices);
    }
}

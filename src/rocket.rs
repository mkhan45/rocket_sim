use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

use crate::physics::Kinematics;
use crate::planet::CelestialBody;
use crate::trajectory::Trajectory;

use crate::graphs::SpeedGraph;
pub struct RocketCrashed(pub bool);

use crate::texture::{TextureName, Textures};

#[derive(Bundle)]
pub struct RocketBundle {
    pub kinematics: Kinematics,
    pub rocket: Rocket,
    pub speed_graph: SpeedGraph,
    pub altitude: Altitude,
}

impl Default for RocketBundle {
    fn default() -> Self {
        RocketBundle {
            kinematics: Kinematics {
                pos: Vec2::new(0.0, -6015.0),
                ..Kinematics::default()
            },
            rocket: Rocket::default(),
            speed_graph: SpeedGraph(std::collections::VecDeque::new()),
            altitude: Altitude(std::f32::MAX),
        }
    }
}

pub struct RocketEntity(pub bevy_ecs::entity::Entity);

pub struct Rocket {
    pub fuel_capacity: f32,
    pub current_fuel_mass: f32,
    pub non_fuel_mass: f32,
    /// how fast the fuel burns
    pub fuel_burn_rate: f32,
    /// how much force per fuel unit
    pub fuel_thrust_factor: f32,
    pub angle: f32,
    // from 0 to 1
    pub thrust: f32,
}

impl Default for Rocket {
    fn default() -> Self {
        Rocket {
            fuel_capacity: 1750.0,
            current_fuel_mass: 1750.0,
            non_fuel_mass: 100.0,
            fuel_burn_rate: 10.0,
            fuel_thrust_factor: 2_150.0,
            angle: 0.0,
            thrust: 1.0,
        }
    }
}

impl Rocket {
    pub fn total_mass(&self) -> f32 {
        self.current_fuel_mass + self.non_fuel_mass
    }
}

pub struct Altitude(pub f32);

pub fn update_altitude_sys(
    mut rocket_query: Query<(&mut Altitude, &Kinematics, &Rocket)>,
    planet_query: Query<(&Kinematics, &CelestialBody)>,
) {
    for (mut altitude, rocket_kinematics, _) in rocket_query.iter_mut() {
        altitude.0 = std::f32::MAX;
        for (planet_kinematics, planet) in planet_query.iter() {
            altitude.0 = altitude
                .0
                .min((rocket_kinematics.pos - planet_kinematics.pos).length() - planet.radius);
        }
    }
}

pub fn draw_rocket(pos: &Vec2, angle: f32, thrust: bool, textures: &Textures, size: f32) {
    let (texture, height) = if thrust {
        (TextureName::RocketBoost, 2.4 * size)
    } else {
        (TextureName::Rocket, 2.0 * size)
    };

    draw_texture_ex(
        textures[texture],
        pos.x - size / 2.0,
        pos.y - height / 2.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(Vec2::new(size, height)),
            rotation: angle,
            ..DrawTextureParams::default()
        },
    );
}

pub fn draw_rocket_sys(
    query: Query<(&Rocket, &Kinematics), Without<Trajectory>>,
    textures: Res<Textures>,
) {
    for (rocket, kinematics) in query.iter() {
        draw_rocket(
            &kinematics.pos,
            rocket.angle,
            rocket.current_fuel_mass > 0.0 && rocket.thrust > 0.0,
            &textures,
            10.0
        );
    }
}

pub fn rocket_input_sys(mut query: Query<&mut Rocket>, dt: Res<crate::physics::DT>) {
    if is_key_down(KeyCode::A) {
        for mut rocket in query.iter_mut() {
            rocket.angle += 0.75 * dt.0;
        }
    }

    if is_key_down(KeyCode::D) {
        for mut rocket in query.iter_mut() {
            rocket.angle -= 0.75 * dt.0;
        }
    }

    if is_key_down(KeyCode::Space) || is_key_down(KeyCode::C) {
        for mut rocket in query.iter_mut() {
            rocket.thrust += 0.1 * dt.0;
            rocket.thrust = rocket.thrust.min(1.0);
        }
    }

    if is_key_down(KeyCode::Z) {
        for mut rocket in query.iter_mut() {
            rocket.thrust -= 0.1 * dt.0;
            rocket.thrust = rocket.thrust.max(0.0);
        }
    }

    if is_key_pressed(KeyCode::Q) {
        for mut rocket in query.iter_mut() {
            rocket.thrust = 0.0;
        }
    }

    if is_key_pressed(KeyCode::E) {
        for mut rocket in query.iter_mut() {
            rocket.thrust = 1.0;
        }
    }
}

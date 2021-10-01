use bevy_ecs::prelude::*;
use egui_macroquad::macroquad::prelude::*;

use crate::physics::Kinematics;

use crate::graphs::SpeedGraph;
pub struct RocketCrashed(pub bool);

#[derive(Bundle)]
pub struct RocketBundle {
    pub kinematics: Kinematics,
    pub rocket: Rocket,
    pub speed_graph: SpeedGraph,
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
}

impl Default for Rocket {
    fn default() -> Self {
        Rocket {
            fuel_capacity: 1500.0,
            current_fuel_mass: 1500.0,
            non_fuel_mass: 120.0,
            fuel_burn_rate: 150.0,
            fuel_thrust_factor: 4000.0,
        }
    }
}

impl Rocket {
    pub fn total_mass(&self) -> f32 {
        self.current_fuel_mass + self.non_fuel_mass
    }
}

pub fn draw_rocket(pos: &Vec2, thrust: bool) {
    // body
    draw_rectangle(pos.x, pos.y, 5.0, 10.0, WHITE);
    // window
    draw_circle(pos.x + 2.5, pos.y + 3.5, 2.0, SKYBLUE);

    // top
    draw_triangle(
        Vec2::new(pos.x + 6.5, pos.y),
        Vec2::new(pos.x - 1.5, pos.y),
        Vec2::new(pos.x + 2.5, pos.y - 5.0),
        RED,
    );

    // left fin
    draw_triangle(
        Vec2::new(pos.x + 5.0, pos.y + 10.0),
        Vec2::new(pos.x + 5.0, pos.y + 5.0),
        Vec2::new(pos.x + 7.0, pos.y + 10.0),
        RED,
    );

    // right fin
    draw_triangle(
        Vec2::new(pos.x, pos.y + 10.0),
        Vec2::new(pos.x, pos.y + 5.0),
        Vec2::new(pos.x - 2.0, pos.y + 10.0),
        RED,
    );

    if thrust {
        draw_triangle(
            Vec2::new(pos.x + 5.0, pos.y + 10.0),
            Vec2::new(pos.x, pos.y + 10.0),
            Vec2::new(pos.x + 2.5, pos.y + 15.0),
            ORANGE,
        )
    }
}

pub fn draw_rocket_sys(query: Query<(&Rocket, &Kinematics)>) {
    for (rocket, kinematics) in query.iter() {
        draw_rocket(&kinematics.pos, rocket.current_fuel_mass > 0.0);
    }
}

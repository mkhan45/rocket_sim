use macroquad::prelude::*;
use std::error::Error;
use bevy::ecs::world::World;
use bevy::ecs::schedule::{Schedule, SystemStage};
use bevy::ecs::system::{Res, ResMut};
use bevy::prelude::{IntoSystem, Stage};

pub mod rocket;
use rocket::Rocket;

pub const SCREEN_HEIGHT: f32 = 800.0;
pub const SCREEN_WIDTH: f32 = 800.0;

#[derive(Debug)]
pub enum GameError {}
impl std::fmt::Display for GameError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "GameError")
    }
}

impl Error for GameError {}

pub struct DT(pub f32);

pub struct MainState {
    world: World,
    schedule: Schedule,
}

fn rocket_physics_sys(mut rocket: ResMut<Rocket>, dt: Res<DT>) {
    let vel = rocket.vel;
    rocket.pos += vel * dt.0;
}

impl MainState {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut schedule = Schedule::default();
        schedule.add_stage("update", SystemStage::single_threaded().with_system(rocket_physics_sys.system()));
        world.insert_resource(Rocket::new());

        MainState {
            world,
            schedule,
        }
    }

    pub fn draw(&self) -> Result<(), GameError> {
        clear_background(BLACK);

        let Rocket { pos, .. } = self.world.get_resource::<Rocket>().unwrap();
        draw_rectangle(pos.x, pos.y, 10.0, 10.0, WHITE);

        Ok(())
    }

    pub fn update(&self) -> Result<(), GameError> {
        Ok(())
    }
}

#[macroquad::main("Rocket")]
async fn main() -> Result<(), GameError> {
    let mut main_state = MainState::new();

    loop {
        main_state.draw()?;
        main_state.update()?;

        main_state.world.insert_resource(DT(get_frame_time()));
        main_state.schedule.run(&mut main_state.world);
        next_frame().await
    }
}

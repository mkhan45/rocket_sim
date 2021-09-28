use macroquad::prelude::*;

pub mod error;
use error::GameError;

pub mod main_state;
pub mod physics;
pub mod rocket;

pub mod camera;

pub mod ui;

pub const SCREEN_WIDTH: f32 = 1000.0;
pub const SCREEN_HEIGHT: f32 = 1000.0;

pub const THRUST_MULTIPLIER: f32 = 75.0;
pub const GRAVITY: f32 = 200.0;

#[macroquad::main("Rocket")]
async fn main() -> Result<(), GameError> {
    let mut main_state = main_state::MainState::new();

    loop {
        main_state.draw()?;
        main_state.update()?;
        main_state.draw_ui()?;

        next_frame().await
    }
}

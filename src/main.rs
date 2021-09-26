use macroquad::prelude::*;

pub mod error;
use error::GameError;

pub mod rocket;
pub mod physics;
pub mod main_state;

pub const SCREEN_WIDTH: f32 = 1000.0;
pub const SCREEN_HEIGHT: f32 = 1000.0;

#[macroquad::main("Rocket")]
async fn main() -> Result<(), GameError> {
    let mut main_state = main_state::MainState::new();

    loop {
        main_state.draw()?;
        main_state.update()?;

        next_frame().await
    }
}

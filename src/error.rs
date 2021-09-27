use std::error::Error;

#[derive(Debug)]
pub enum GameError {}
impl std::fmt::Display for GameError {
    fn fmt(
        &self,
        formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        write!(formatter, "GameError")
    }
}

impl Error for GameError {}

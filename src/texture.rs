use egui_macroquad::macroquad::prelude::*;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
pub enum TextureName {
    Earth,
    Moon,
    Rocket,
    RocketBoost,
}

pub struct Textures(HashMap<TextureName, Texture2D>);
impl Default for Textures {
    fn default() -> Self {
        let mut map = HashMap::new();

        map.insert(
            TextureName::Earth,
            Texture2D::from_file_with_format(
                include_bytes!("../textures/earth.png"),
                Some(ImageFormat::Png),
            ),
        );

        map.insert(
            TextureName::Rocket,
            Texture2D::from_file_with_format(
                include_bytes!("../textures/rocket.png"),
                Some(ImageFormat::Png),
            ),
        );

        map.insert(
            TextureName::RocketBoost,
            Texture2D::from_file_with_format(
                include_bytes!("../textures/rocket_boost.png"),
                Some(ImageFormat::Png),
            ),
        );

        map.insert(
            TextureName::Moon,
            Texture2D::from_file_with_format(
                include_bytes!("../textures/moon.png"),
                Some(ImageFormat::Png),
            ),
        );

        Textures(map)
    }
}

impl std::ops::Index<TextureName> for Textures {
    type Output = Texture2D;

    fn index(&self, texture_name: TextureName) -> &Self::Output {
        self.0.get(&texture_name).unwrap()
    }
}

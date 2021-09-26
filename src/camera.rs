use macroquad::prelude::Vec2;

pub struct Camera {
    pub pos: Vec2,
    pub size: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            pos: Vec2::new(0.0, 0.0),
            size: Vec2::new(crate::SCREEN_WIDTH, crate::SCREEN_HEIGHT),
        }
    }
}

impl Camera {
    pub fn transform_point(point: &Vec2) -> Vec2 {
        todo!()
    }
}

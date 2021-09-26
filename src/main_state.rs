use crate::physics::Kinematics;

use bevy::ecs::schedule::{Schedule, SystemStage};
use bevy::ecs::world::World;
use bevy::prelude::{IntoSystem, Stage};

use macroquad::prelude::*;

use crate::rocket::{RocketBundle, RocketEntity};
use crate::physics::{self, DT};
use crate::GameError;

pub struct MainState {
    world: World,
    schedule: Schedule,
}

impl MainState {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "update",
            SystemStage::single_threaded().with_system(physics::integration_sys.system()),
        );


        let rocket = world.spawn().insert_bundle(
            RocketBundle {
                kinematics: Kinematics {
                    acc: Vec2::new(0.0, -5000.0),
                    pos: Vec2::new(500.0, 800.0),
                    ..Kinematics::default()
                },
                ..RocketBundle::default()
            }
        ).id();

        world.insert_resource(RocketEntity(rocket));

        MainState { world, schedule }
    }

    pub fn draw(&self) -> Result<(), GameError> {
        clear_background(BLACK);

        let RocketEntity(rocket_entity) = self.world.get_resource::<RocketEntity>().unwrap();
        let kinematics = self.world.get::<Kinematics>(*rocket_entity).unwrap();
        draw_rectangle(kinematics.pos.x / 1000.0 * screen_width(), kinematics.pos.y / 1000.0 * screen_height(), 10.0, 10.0, WHITE);

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        self.world.insert_resource(DT(get_frame_time()));
        self.schedule.run(&mut self.world);
        Ok(())
    }
}


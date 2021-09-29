use crate::physics::Kinematics;

use bevy_ecs::prelude::{IntoSystem, Stage};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, Schedule, SystemStage};
use bevy_ecs::world::World;

use egui_macroquad::macroquad::prelude::*;

use crate::physics::{self, DT};
use crate::rocket::{RocketBundle, RocketEntity};
use crate::GameError;

use crate::camera;

pub struct MainState {
    pub world: World,
    schedule: Schedule,
}

impl MainState {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut schedule = Schedule::default();
        schedule.add_stage(
            "physics",
            SystemStage::single_threaded()
                .with_system(physics::rocket_thrust_sys.system())
                .with_system(physics::rocket_gravity_sys.system()),
        );
        schedule.add_stage_after(
            "physics",
            "integrate",
            SystemStage::single_threaded()
                .with_system(physics::integration_sys.system().label("integrate"))
                .with_system(physics::reset_accel_sys.system().after("integrate")),
        );
        schedule.add_stage_after(
            "integrate",
            "camera",
            SystemStage::single_threaded()
                .with_system(camera::camera_follow_sys.system().label("follow"))
                .with_system(camera::update_camera_sys.system().after("follow")),
        );

        let rocket = world
            .spawn()
            .insert_bundle(RocketBundle {
                kinematics: Kinematics {
                    pos: Vec2::new(0.0, 0.0),
                    ..Kinematics::default()
                },
                ..RocketBundle::default()
            })
            .id();
        world.insert_resource(RocketEntity(rocket));

        world.insert_resource(crate::camera::CameraRes::default());

        MainState { world, schedule }
    }

    pub fn draw(&self) -> Result<(), GameError> {
        clear_background(BLACK);

        let RocketEntity(rocket_entity) = self.world.get_resource::<RocketEntity>().unwrap();
        let kinematics = self.world.get::<Kinematics>(*rocket_entity).unwrap();
        crate::rocket::draw_rocket(&kinematics.pos);

        Ok(())
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        self.world.insert_resource(DT(get_frame_time()));
        self.schedule.run(&mut self.world);
        Ok(())
    }
}

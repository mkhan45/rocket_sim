use bevy_ecs::prelude::{IntoSystem, Stage};
use bevy_ecs::schedule::{ParallelSystemDescriptorCoercion, Schedule, SystemStage};
use bevy_ecs::world::World;

use egui_macroquad::macroquad::prelude::*;

use crate::graphs;
use crate::physics::{self, DT};
use crate::rocket::{RocketBundle, RocketEntity};
use crate::GameError;

use crate::camera;

pub struct MainState {
    pub world: World,
    frame_schedule: Schedule,
    fixed_schedule: Schedule,
    draw_schedule: Schedule,
    leftover_time: f32,
}

impl MainState {
    pub fn new() -> Self {
        let mut world = World::new();

        let mut fixed_schedule = Schedule::default();
        fixed_schedule.add_stage(
            "physics",
            SystemStage::single_threaded()
                .with_system(physics::rocket_thrust_sys.system())
                .with_system(physics::rocket_planet_interaction_sys.system()),
        );
        fixed_schedule.add_stage_after(
            "physics",
            "integrate",
            SystemStage::single_threaded()
                .with_system(physics::integration_sys.system().label("integrate"))
                .with_system(physics::reset_accel_sys.system().after("integrate"))
                .with_system(graphs::rocket_graph_sys.system().after("integrate")),
        );

        let mut frame_schedule = Schedule::default();
        frame_schedule.add_stage(
            "camera",
            SystemStage::single_threaded()
                .with_system(camera::camera_follow_sys.system().label("follow"))
                .with_system(camera::update_camera_sys.system().after("follow")),
        );
        frame_schedule.add_stage(
            "input",
            SystemStage::single_threaded().with_system(crate::map::map_input_sys.system()),
        );

        let mut draw_schedule = Schedule::default();
        draw_schedule.add_stage(
            "draw",
            SystemStage::single_threaded()
                .with_system(crate::planet::draw_planet_sys.system().label("planets"))
                .with_system(
                    crate::rocket::draw_rocket_sys
                        .system()
                        .label("rocket")
                        .after("planets"),
                )
                .with_system(
                    crate::map::draw_map_sys
                        .system()
                        .label("map")
                        .after("rocket"),
                ),
        );

        let rocket = world.spawn().insert_bundle(RocketBundle::default()).id();
        world.insert_resource(RocketEntity(rocket));

        world.insert_resource(crate::camera::CameraRes::default());
        world.insert_resource(crate::physics::Steps(1));
        world.insert_resource(DT(1.0 / 60.0));
        world.insert_resource(crate::map::MapRes::default());
        world.insert_resource(crate::texture::Textures::default());

        crate::planet::add_planets(&mut world);

        MainState {
            world,
            frame_schedule,
            fixed_schedule,
            draw_schedule,
            leftover_time: 0.0,
        }
    }

    pub fn draw(&mut self) -> Result<(), GameError> {
        clear_background(BLACK);
        self.draw_schedule.run(&mut self.world);
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), GameError> {
        {
            let dt = self.world.get_resource::<DT>().unwrap().0;
            let steps = self
                .world
                .get_resource::<crate::physics::Steps>()
                .unwrap()
                .0;
            let target_dt = steps as f32 / 60.0 + self.leftover_time;
            let mut acc_time = 0.0;
            while acc_time < target_dt {
                self.fixed_schedule.run(&mut self.world);
                acc_time += dt;
            }
            self.leftover_time = acc_time - target_dt;
        }

        self.frame_schedule.run(&mut self.world);
        Ok(())
    }
}

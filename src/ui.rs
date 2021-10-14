use crate::error::GameError;
use crate::main_state::MainState;
use crate::physics::Kinematics;
use crate::rocket::Rocket;
use crate::rocket::RocketEntity;
use egui_macroquad::egui;

use bevy_ecs::entity::Entity;

use egui_macroquad::egui::Pos2;
use egui_macroquad::egui::Rect as EguiRect;
// use egui_macroquad::egui::Vec2 as EguiVec;
use egui_macroquad::macroquad::prelude::Rect;

use crate::graphs::SpeedGraph;
use crate::rocket::Altitude;

fn _to_egui_rect(rect: &Rect) -> EguiRect {
    EguiRect::from_two_pos(
        Pos2::new(rect.x, rect.y),
        Pos2::new(rect.x + rect.w, rect.y + rect.h),
    )
}

impl MainState {
    pub fn draw_ui(&mut self) -> Result<(), GameError> {
        egui_macroquad::ui(|egui_ctx| {
            use egui::{FontDefinitions, TextStyle};
            let mut fonts = FontDefinitions::default();
            fonts.family_and_size.get_mut(&TextStyle::Body).unwrap().1 = 24.0;
            egui_ctx.set_fonts(fonts);

            egui::Window::new("")
                .id(egui::Id::new("Main"))
                .show(egui_ctx, |ui| {
                    let RocketEntity(rocket_entity) =
                        self.world.get_resource::<RocketEntity>().unwrap();
                    self.fuel_bar(rocket_entity, ui);
                    self.throttle_bar(rocket_entity, ui);
                    self.rocket_info(rocket_entity, ui);

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    self.time_speed_slider(ui);

                    ui.add_space(5.0);
                    ui.separator();
                    ui.add_space(5.0);

                    self.draw_graphs(ui);
                });
        });

        egui_macroquad::draw();

        Ok(())
    }

    fn fuel_bar(&self, rocket_entity: &Entity, ui: &mut egui::Ui) {
        let rocket = self.world.get::<Rocket>(*rocket_entity).unwrap();

        let fuel_used_proportion = rocket.current_fuel_mass / rocket.fuel_capacity;

        ui.horizontal(|ui| {
            ui.label("Fuel Remaining:");
            ui.add(
                egui::ProgressBar::new(fuel_used_proportion)
                    .text(format!("Remaining: {:.2}", rocket.current_fuel_mass)),
            );
        });
    }

    fn throttle_bar(&self, rocket_entity: &Entity, ui: &mut egui::Ui) {
        let rocket = self.world.get::<Rocket>(*rocket_entity).unwrap();
        let thrust = rocket.thrust;

        ui.horizontal(|ui| {
            ui.label("Throttle: ");
            ui.add(egui::ProgressBar::new(thrust));
        });
    }

    fn rocket_info(&self, rocket_entity: &Entity, ui: &mut egui::Ui) {
        let kinematics = self.world.get::<Kinematics>(*rocket_entity).unwrap();
        let altitude = self.world.get::<Altitude>(*rocket_entity).unwrap();

        ui.label(format!(
            "Velocity: <{:.2}, {:.2}>",
            kinematics.vel.x * 1000.0,
            kinematics.vel.y * 1000.0
        ));

        ui.label(format!(
            "Position: <{:.2}, {:.2}>",
            kinematics.pos.x, kinematics.pos.y
        ));

        ui.label(format!("Altitude: {:.2}", altitude.height * 1000.0));
    }

    fn time_speed_slider(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut steps = self
                .world
                .get_resource_mut::<crate::physics::Steps>()
                .unwrap();

            ui.label("Sim Speed:");
            ui.add(egui::Slider::new(&mut steps.0, 1..=100));
        });
    }

    fn draw_graphs(&mut self, ui: &mut egui::Ui) {
        use egui::plot::{Line, Plot, Value, Values};

        let rocket_entity = self.world.get_resource::<RocketEntity>().unwrap().0;
        let mut speed_graphs = self.world.query::<&SpeedGraph>();
        let speed_graph = &speed_graphs.get(&self.world, rocket_entity).unwrap().0;
        ui.label("Speed:");
        ui.add_space(2.5);
        ui.add(
            Plot::new("velocity").line(Line::new(Values::from_values_iter(
                speed_graph
                    .iter()
                    .enumerate()
                    .map(|(i, y)| Value::new(i as f64, *y as f64)),
            ))),
        );
    }
}

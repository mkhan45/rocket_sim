use crate::error::GameError;
use crate::main_state::MainState;
use crate::physics::Kinematics;
use crate::rocket::Rocket;
use crate::rocket::RocketEntity;
use egui_macroquad::egui;

use bevy_ecs::entity::Entity;

use egui_macroquad::egui::Pos2;
use egui_macroquad::egui::Rect as EguiRect;
use egui_macroquad::egui::Vec2 as EguiVec;
use egui_macroquad::macroquad::prelude::{screen_height, screen_width, Rect};

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

            egui::Window::new("Rocket")
                .fixed_rect(EguiRect::from_min_size(
                    Pos2::new(0.0, 0.0),
                    EguiVec::new(screen_width() / 3.0, screen_height() / 20.0),
                ))
                .show(egui_ctx, |ui| {
                    let RocketEntity(rocket_entity) =
                        self.world.get_resource::<RocketEntity>().unwrap();
                    self.fuel_bar(rocket_entity, ui);
                    self.rocket_info(rocket_entity, ui);
                });

            egui::Window::new("Simulation")
                .default_rect(EguiRect::from_min_size(
                    Pos2::new(0.0, screen_height() / 10.0 + 50.0),
                    EguiVec::new(screen_width() / 3.0, screen_height() / 20.0),
                ))
                .show(egui_ctx, |ui| {
                    self.time_speed_slider(ui);
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

    fn rocket_info(&self, rocket_entity: &Entity, ui: &mut egui::Ui) {
        let kinematics = self.world.get::<Kinematics>(*rocket_entity).unwrap();

        ui.label(format!(
            "Velocity: <{:.2}, {:.2}>",
            kinematics.vel.x, -kinematics.vel.y
        ));

        ui.label(format!("Altitude: {:.2}", -kinematics.pos.y));
    }

    fn time_speed_slider(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            let mut steps = self
                .world
                .get_resource_mut::<crate::physics::Steps>()
                .unwrap();

            ui.label("Sim Speed:");
            ui.add(egui::Slider::new(&mut steps.0, 0..=10));
        });
    }
}

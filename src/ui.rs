use crate::rocket::Rocket;
use crate::rocket::RocketEntity;
use crate::error::GameError;
use egui_macroquad::egui;
use crate::main_state::MainState;

use bevy::ecs::entity::Entity;

impl MainState {
    pub fn draw_ui(&mut self) -> Result<(), GameError> {
        let RocketEntity(rocket_entity) = self.world.get_resource::<RocketEntity>().unwrap();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Fuel Remaining").show(egui_ctx, |ui| {
                self.fuel_bar(rocket_entity, ui);
            });
        });

        egui_macroquad::draw();

        Ok(())
    }

    fn fuel_bar(&self, rocket_entity: &Entity, ui: &mut egui::Ui) {
        let rocket = self.world.get::<Rocket>(*rocket_entity).unwrap();

        let fuel_used_proportion = rocket.current_fuel_mass / rocket.fuel_capacity;
        ui.add(egui::ProgressBar::new(fuel_used_proportion));
    }
}

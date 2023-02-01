use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};

pub fn open_debug_window(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("Debug").show(egui_context.ctx_mut(), |ui| {
        ui.label("Test that shit");
    });
}
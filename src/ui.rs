use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub fn ui_example_system(mut contexts: EguiContexts) {
    egui::Window::new("Hello").show(contexts.ctx_mut(), |ui| {
        ui.label("world");
    });
}
use bevy::prelude::*;
use bevy::app::{App, Plugin};
use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiPrimaryContextPass};
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        // Don't add the plugin for users, let them chose the default mode themselves
        // and just make sure they initialize EguiPlugin before yours.
        assert!(app.is_plugin_added::<EguiPlugin>());

        app.add_systems(EguiPrimaryContextPass, ui);
    }
}

pub fn ui(mut contexts: EguiContexts) -> Result {
    egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
        ui.label("world");
    });
    Ok(())
}
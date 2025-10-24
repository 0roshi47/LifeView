use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*
};
use bevy_egui::{EguiPlugin};
use grid::Grid;
use crate::grid::update_generation;
use crate::interface::UiPlugin;

mod interface;
mod cell;
mod grid;
mod rule;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin,)
        .add_plugins(EguiPlugin::default())
        .add_plugins(UiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, grid::spawn.after(setup))
        // .add_systems(EguiPrimaryContextPass, interface::init_ui)
        // .add_systems(EguiPrimaryContextPass, init_ui)
        .add_systems(Update, update_generation)
        .add_systems(Update, animate_materials)
        .run();
}

// pub fn init_ui(mut contexts: EguiContexts) -> Result {
//     egui::Window::new("Hello").show(contexts.ctx_mut()?, |ui| {
//         ui.label("world");
//     });
//     Ok(())
// }

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(100, 100, 5.0);
    commands.insert_resource(grid);
}

fn animate_materials (
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>
) {
    for (i, (_id, material)) in materials.iter_mut().enumerate() {
        if i < grid.cells.len()-1 {
            material.color = Color::linear_rgb(grid.cells.get(i).unwrap().state, 0.0, 1.0 - grid.cells.get(i).unwrap().state);
        }
    }
}


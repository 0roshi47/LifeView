use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, math::ops::{abs, sin}, prelude::*
};

use grid::Grid;
use cell::Cell;

use crate::grid::update_generation;

mod cell;
mod grid;
mod growth;

fn main() {
    App::new()
            .add_plugins((
            // The diagnostics plugins need to be added after DefaultPlugins as they use e.g. the time plugin for timestamps.
            DefaultPlugins,
            // Adds a system that prints diagnostics to the console.
            // The other diagnostics plugins can still be used without this if you want to use them in an ingame overlay for example.
            LogDiagnosticsPlugin::default(),
            // Adds frame time, FPS and frame count diagnostics.
            FrameTimeDiagnosticsPlugin::default(),
            // Adds an entity count diagnostic.
            bevy::diagnostic::EntityCountDiagnosticsPlugin,
            // Adds cpu and memory usage diagnostics for systems and the entire game process.
            bevy::diagnostic::SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Startup, grid::spawn.after(setup))
        .add_systems(Update, update_generation)
        .add_systems(Update, animate_materials)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(100, 100, 5.0);
    commands.insert_resource(grid);
}

fn animate_materials (
    // material_handles: Query<&MeshMaterial2d<ColorMaterial>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>
) {
    for (i, (_id, material)) in materials.iter_mut().enumerate() {
        if i < grid.cells.len()-1 {
            material.color = Color::linear_rgb(grid.cells.get(i).unwrap().state, 0.0, 1.0 - grid.cells.get(i).unwrap().state);
        }
    }
}


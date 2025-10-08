use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
};

use grid::Grid;

mod cell;
mod grid;

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
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(100, 100, 5.0);
    commands.insert_resource(grid);
}

// fn animate_materials(
//     material_handles: Query<&MeshMaterial2d<ColorMaterial>>,
//     time: Res<Time>,
//     mut materials: ResMut<Assets<ColorMaterial>>,
// ) {
//     for (_id, material) in materials.iter_mut() {
//         material.color = Color::linear_rgb(1.0, 0.0, 0.0);
//     }
// }


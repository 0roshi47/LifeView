use bevy::{
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d, Material2dPlugin},
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

use bevy_egui::{EguiPlugin};
use grid::Grid;
use crate::{cell::Cell, grid::update_generation};
use crate::interface::UiPlugin;

mod interface;
mod cell;
mod grid;
mod rule;

const SHADER_ASSET_PATH: &str = "shaders/cell.wgsl";

const BASE_CELL_SIZE: f32 = 10.0;
const BASE_CELL_WIDTH: usize = 50;
const BASE_CELL_HEIGHT: usize = 50;

const BASE_COLOR_ALIVE: Color = Color::linear_rgb(1.0, 1.0, 0.0);
const BASE_COLOR_DEATH: Color = Color::linear_rgb(0.0, 1.0, 1.0);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(LogDiagnosticsPlugin::default())
        .add_plugins(FrameTimeDiagnosticsPlugin::default())
        .add_plugins(bevy::diagnostic::SystemInformationDiagnosticsPlugin,)
        .add_plugins(EguiPlugin::default())
        .add_plugins(UiPlugin)
        .add_plugins(Material2dPlugin::<CustomMaterial>::default())
        .add_systems(Startup, setup)
        // .add_systems(Startup, spawn.after(setup))
        // .add_systems(FixedUpdate, update_generation)
        // .add_systems(FixedUpdate, animate_materials)
        .run();
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(BASE_CELL_WIDTH, BASE_CELL_HEIGHT, BASE_CELL_SIZE);
    let handle_mesh: Handle<Mesh> = meshes.add(Rectangle::new(BASE_CELL_SIZE, BASE_CELL_SIZE));
    for (i, cell) in grid.cells.iter().enumerate() {
        let x: f32 = (i%grid.width) as f32*grid.cell_size - (grid.width as f32*grid.cell_size)/2.0;
        let y: f32 = (i/grid.width) as f32*grid.cell_size - (grid.height as f32*grid.cell_size)/2.0;
        commands.spawn((
            Mesh2d(handle_mesh.clone()),
            MeshMaterial2d(materials.add(CustomMaterial {
                color: LinearRgba::new(cell.state, 0.0, 1.0-cell.state, 1.0)
            })),
            Transform::from_xyz(x, y, 0.0)
        ));
    }
    commands.insert_resource(grid);
}

// fn animate_materials (
//     mut materials: ResMut<Assets<ColorMaterial>>,
//     grid: Res<Grid>
// ) {
//     for (i, (_id, material)) in materials.iter_mut().enumerate() {
//         if i < grid.cells.len()-1 {
//             let c: &Cell = grid.cells.get(i).unwrap();
//             material.color = Color::linear_rgb(c.state, 0.0, 1.0 - c.state);
//         }
//     }
// }

// fn spawn (
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<CustomMaterial>>,
//     asset_server: Res<AssetServer>,
//     grid: Res<Grid>,
// ) {
//     let grid = grid.into_inner();
//     let handle_mesh = meshes.add(Rectangle::new(grid.cell_size, grid.cell_size));
//     for (i, _cell) in grid.cells.iter().enumerate() {
//         let x: f32 = (i%grid.width) as f32*grid.cell_size - (grid.width as f32*grid.cell_size)/2.0;
//         let y: f32 = (i/grid.width) as f32*grid.cell_size - (grid.height as f32*grid.cell_size)/2.0;
//         commands.spawn((
//             Mesh2d(handle_mesh.clone()),
//             MeshMaterial2d(materials.add(CustomMaterial {
//                 image: Some(asset_server.load("branding/icon.png")).unwrap(),
//             })),
//             Transform::from_xyz(x, y, 0.0),
//         ));
//     }
// }

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

/// The Material2d trait is very configurable, but comes with sensible defaults for all methods.
/// You only need to implement functions for features that need non-default behavior. See the Material2d api docs for details!
impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

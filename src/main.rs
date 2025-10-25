use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, prelude::*, sprite_render::Material2dPlugin,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{AlphaMode2d, Material2d},
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
        .add_systems(Startup, spawn.after(setup))
        // .add_systems(EguiPrimaryContextPass, interface::init_ui)
        // .add_systems(EguiPrimaryContextPass, init_ui)
        .add_systems(FixedUpdate, update_generation)
        .add_systems(FixedUpdate, animate_materials)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);
    let grid: Grid = Grid::new(70, 70, 7.5);
    commands.insert_resource(grid);
}

fn animate_materials (
    mut materials: ResMut<Assets<ColorMaterial>>,
    grid: Res<Grid>
) {
    for (i, (_id, material)) in materials.iter_mut().enumerate() {
        if i < grid.cells.len()-1 {
            let c: &Cell = grid.cells.get(i).unwrap();
            material.color = Color::linear_rgb(c.state, 0.0, 1.0 - c.state);
        }
    }
}

pub fn spawn (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    asset_server: Res<AssetServer>,
    grid: Res<Grid>,
) {
    let grid = grid.into_inner();
    let handle_mesh = meshes.add(Rectangle::new(grid.cell_size, grid.cell_size));
    for (i, _cell) in grid.cells.iter().enumerate() {
        let x: f32 = (i%grid.width) as f32*grid.cell_size - (grid.width as f32*grid.cell_size)/2.0;
        let y: f32 = (i/grid.width) as f32*grid.cell_size - (grid.height as f32*grid.cell_size)/2.0;
        commands.spawn((
            Mesh2d(handle_mesh.clone()),
            MeshMaterial2d(materials.add(CustomMaterial {
                color: LinearRgba::BLUE,
                color_texture: Some(asset_server.load("branding/icon.png")),
            })),
            Transform::from_xyz(x, y, 0.0),
        ));
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Mask(0.5)
    }
}
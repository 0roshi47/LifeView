use bevy::{
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin}, mesh::MeshTag, prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef, sprite_render::{Material2d, Material2dPlugin}
};

use bevy_egui::EguiPlugin;
use grid::Grid;
use crate::grid::update_generation;
use crate::interface::UiPlugin;

mod interface;
mod cell;
mod grid;
mod rule;

const SHADER_ASSET_PATH: &str = "shaders/cell.wgsl";

const BASE_CELL_WIDTH: usize = 100;

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
        .add_systems(FixedUpdate, update_generation)
        .add_systems(FixedUpdate, animate_materials)
        .run();
}

fn setup (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    windows: Query<&mut Window>
) {
    commands.spawn(Camera2d);
    let cell_size: f32 = windows.single().unwrap().resolution.width()/BASE_CELL_WIDTH as f32;
    let height: usize = (BASE_CELL_WIDTH*9)/16;
    let grid: Grid = Grid::new(BASE_CELL_WIDTH, height, cell_size);
    let handle_mesh: Handle<Mesh> = meshes.add(Rectangle::new(cell_size, cell_size));
    for (i, _cell) in grid.cells.iter().enumerate() {
        let x: f32 = (i%grid.width) as f32*grid.cell_size - (grid.width as f32*grid.cell_size)/2.0;
        let y: f32 = (i/grid.width) as f32*grid.cell_size - (grid.height as f32*grid.cell_size)/2.0;
        commands.spawn((
            Mesh2d(handle_mesh.clone()),
            MeshMaterial2d(materials.add(CustomMaterial {
                cell_state: 1.0
            })),
            MeshTag(i as u32),
            Transform::from_xyz(x, y, 0.0)
        ));
    }
    commands.insert_resource(grid);
} 

fn animate_materials (
    grid: Res<Grid>,
    query: Query<(&MeshTag, &MeshMaterial2d<CustomMaterial>)>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for (mesh_tag, mat_handle) in query.iter() {
        let i = mesh_tag.0 as usize;
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.cell_state = grid.cells[i].state;
        }
    }
}



#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[uniform(0)]
    cell_state: f32
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

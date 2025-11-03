use bevy::{
    mesh::MeshTag,
    prelude::*,
    reflect::TypePath,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
    sprite_render::{Material2d, Material2dPlugin},
};

use crate::Grid;

const SHADER_ASSET_PATH: &str = "shaders/cell.wgsl";

const BASE_CELL_WIDTH: usize = 75;

pub struct CellMaterialPlugin;
impl Plugin for CellMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Material2dPlugin::<CustomMaterial>::default());
        app.add_systems(Startup, setup);
        // app.add_systems(Startup, animate_materials.after(setup));
        app.add_systems(FixedUpdate, animate_materials);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
    windows: Query<&mut Window>,
) {
    commands.spawn(Camera2d);
    let cell_size: f32 = windows.single().unwrap().resolution.width() / BASE_CELL_WIDTH as f32;
    let height: usize = (BASE_CELL_WIDTH * 9) / 16;
    let grid: Grid = Grid::new(BASE_CELL_WIDTH, height, cell_size);
    let handle_mesh: Handle<Mesh> = meshes.add(Rectangle::new(cell_size, cell_size));
    // let handle_material: Handle<CustomMaterial> = materials.add(CustomMaterial {
    //     color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
    // });
    for (i, _cell) in grid.cells.iter().enumerate() {
        let x: f32 =
            (i % grid.width) as f32 * grid.cell_size - (grid.width as f32 * grid.cell_size) / 2.0;
        let y: f32 =
            (i / grid.width) as f32 * grid.cell_size - (grid.height as f32 * grid.cell_size) / 2.0;
        commands.spawn((
            Mesh2d(handle_mesh.clone()),
            MeshMaterial2d(materials.add(CustomMaterial {
                color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            })),
            MeshTag(i as u32),
            Transform::from_xyz(x + grid.cell_size / 2.0, y + grid.cell_size / 2.0, 0.0),
        ));
    }
    commands.insert_resource(grid);
}

pub fn animate_materials(
    grid: Res<Grid>,
    query: Query<(&MeshTag, &MeshMaterial2d<CustomMaterial>)>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    for (mesh_tag, mat_handle) in query.iter() {
        let i = mesh_tag.0 as usize;
        let new_color: LinearRgba = grid.grid_coloration.lerp(grid.cells[i].state);
        if let Some(mat) = materials.get_mut(&mat_handle.0) {
            mat.color = new_color
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CustomMaterial {
    #[uniform(0)]
    color: LinearRgba,
}

impl Material2d for CustomMaterial {
    fn fragment_shader() -> ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

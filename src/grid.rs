use bevy::{ecs::error::HandleError, prelude::*};
use crate::cell::Cell;

use rand::Rng;

#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    width: usize,
    height: usize,
    cell_size: f32,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width*height],
            width: width,
            height: height,
            cell_size: cell_size,
        };
        grid.init();
        grid
    }

    pub fn init(&mut self) {
        let mut rng = rand::rng();
        const DENSITY: f32 = 0.1;
        let center: IVec2 = IVec2::new(self.width as i32/2, self.height as i32/2);
        for i in 0..self.cells.len() {
            let distance_from_center: f32 = ((idx_to_vector(i as i32, self.width as i32) - center).length_squared() as f32).sqrt()*DENSITY;
            let state: f32 = 1.0 - (rng.random::<f32>() * distance_from_center).clamp(0.0, 1.0);
            // println!("{}", state);
            self.cells[i] = Cell::new(state);
        }
    }

}

pub fn idx_to_vector(idx: i32, width: i32) -> IVec2 {
    IVec2::new(idx%width as i32, idx/width as i32)
}

pub fn spawn (
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        grid: Res<Grid>,
    ) {
        let grid = grid.into_inner();
        let handle = meshes.add(Rectangle::new(grid.cell_size, grid.cell_size));
        for (i, cell) in grid.cells.iter().enumerate() {
            let x: f32 = (i%grid.width) as f32*grid.cell_size - (grid.width as f32*grid.cell_size)/2.0;
            let y: f32 = (i/grid.width) as f32*grid.cell_size - (grid.height as f32*grid.cell_size)/2.0;
            commands.spawn((
                Mesh2d(handle.clone()),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::linear_rgb(cell.state, 0.0, 1.0 - cell.state),
                    ..default()
                })),
                Transform::from_xyz(x, y, 0.0),
            ));
        }
}

// pub fn insert_mesh(
//     mut commands: Commands,
//     color_query: Query<Mesh2d, With<MeshMaterial2d<ColorMaterial>>>
// ) {

// }
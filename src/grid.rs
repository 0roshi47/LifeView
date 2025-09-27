use bevy::{picking::window, prelude::*};
use crate::cell::Cell;

use rand::Rng;

#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    width: usize,
    height: usize
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width*height],
            width: width,
            height: height
        };
        grid.init();
        grid
    }

    pub fn init(&mut self) {
        const DENSITY: f32 = 0.1;
        let mut rng = rand::rng();
        for i in 0..self.cells.len() {
            if rng.random::<f32>() <= DENSITY {
                self.cells[i] = Cell::new(1.0);
            }
        }
    }

}

pub fn spawn (
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
        grid: Res<Grid>,
    ) {
        let grid = grid.into_inner();
        for (i, _cell) in grid.cells.iter().enumerate() {
            commands.spawn((
                Mesh2d(meshes.add(Rectangle::new(10.0, 10.0))),
                MeshMaterial2d(materials.add(ColorMaterial {
                    color: Color::srgb_u8(255, 0, 0),
                    ..default()
                })),
                Transform::from_xyz((i % grid.width) as f32 - (grid.width as f32 / 2.0), (i/grid.height) as f32, 0.0),
            ));
        }
}
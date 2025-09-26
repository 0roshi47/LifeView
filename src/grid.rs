use bevy::prelude::*;
use crate::cell::Cell;

use rand::Rng;

#[derive(Clone, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    width: usize,
    height: usize
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width * height],
            width: width,
            height: height
        };
        grid.init();
        grid
    }

    pub fn spawn(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<ColorMaterial>>,
    ) {
        commands.spawn(Camera2d);
        commands.spawn((
            Mesh2d(meshes.add(Rectangle::new(1.0, 1.0))),
            MeshMaterial2d(materials.add(ColorMaterial {
                color: Color::srgb_u8(255, 0, 0),
                ..default()
            })),
            Transform::from_xyz(0.0, 0.5, 0.0),
        ));
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
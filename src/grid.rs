use bevy::prelude::*;
use crate::cell::Cell;
use crate::rule::Rule;

use rand::Rng;

#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub rule: Rule
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width*height],
            width: width,
            height: height,
            cell_size: cell_size,
            rule: Rule::default()
        };
        grid.init();
        grid
    }

    pub fn init(&mut self) {
        let mut rng = rand::rng();
        const DENSITY: f32 = 0.1;
        let center: IVec2 = IVec2::new(self.width as i32/2, self.height as i32/2);
        for i in 0..self.cells.len() {
            let distance_from_center: f32 = ((self.idx_to_vector(i as i32) - center).length_squared() as f32).sqrt()*DENSITY;
            let state: f32 = 1.0 - (rng.random::<f32>() * distance_from_center).clamp(0.0, 1.0);
            self.cells[i] = Cell::new(state);
        }
    }

    pub fn life_around(&self, pos: IVec2) -> f32 {
        let mut result: f32 = 0.0;
        for x in -1..2 {
            for y in -1..2 {
                let neighbour: IVec2 = IVec2::new(x, y);
                if neighbour == IVec2::ZERO {
                    continue;
                }
                let neighbour_cell: IVec2 = pos+neighbour;
                if !self.in_bounds(neighbour_cell) {
                    continue;
                }
                result += self.cells.get(self.vector_to_idx(neighbour_cell) as usize).unwrap().state;
            }
        }
        result
    }

    pub fn idx_to_vector(&self, idx: i32) -> IVec2 {
        IVec2::new(idx%self.width as i32, idx/self.width as i32)
    }
    
    pub fn vector_to_idx(&self, pos: IVec2) -> i32 {
        pos.x%self.width as i32 + pos.y*self.height as i32
    }

    pub fn in_bounds(&self, pos: IVec2) -> bool {
        pos.x >= 0 && pos.x < self.width as i32 && pos.y > 0 && pos.y < self.height as i32
    }

    pub fn generation (&self) -> Vec<Cell> {
        let mut result : Vec<Cell> = vec![Cell::default(); self.width*self.height];
        for (idx, _cell) in self.cells.iter().enumerate() {
            let life_around_value: f32 = self.life_around(self.idx_to_vector(idx as i32));
            result[idx] = Cell::new(self.rule.growth(life_around_value));
        }
        result
    }
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

pub fn update_generation (
    mut grid: ResMut<Grid>
) {
    let new_generation: Vec<Cell> = grid.generation();
    grid.cells = new_generation;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_idx_to_vector() {
        let grid = Grid::new(100, 100, 5.0);

        assert_eq!(grid.idx_to_vector(0), IVec2::new(0, 0));
        assert_eq!(grid.idx_to_vector(100), IVec2::new(0, 1));
        assert_eq!(grid.idx_to_vector(101), IVec2::new(1, 1));
        assert_eq!(grid.idx_to_vector(1010), IVec2::new(10, 10));
    }

    #[test]
    fn grid_vector_to_idx() {
        let grid = Grid::new(100, 100, 5.0);

        assert_eq!(grid.vector_to_idx(IVec2::new(0, 0)), 0);
        assert_eq!(grid.vector_to_idx(IVec2::new(0, 1)), 100);
        assert_eq!(grid.vector_to_idx(IVec2::new(1, 1)), 101);
        assert_eq!(grid.vector_to_idx(IVec2::new(10, 10)), 1010);
    }
}
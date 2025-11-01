use bevy::math::ops::sqrt;
use bevy::{
    prelude::*,
};

use crate::cell::Cell;
use crate::grid_coloration::GridColoration;
use crate::rule::Rule;

use rand::Rng;


#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub rule: Rule,
    pub grid_coloration: GridColoration,
    pub paused: bool,
    pub generation_type: GenerationType
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width*height],
            width: width,
            height: height,
            cell_size: cell_size,
            rule: Rule::default(),
            grid_coloration: GridColoration::default(),
            paused: true,
            generation_type: GenerationType::RANDOM
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
            let state: f32;
            if self.generation_type == GenerationType::NOISE {
                state = 1.0 - (rng.random::<f32>() * distance_from_center).clamp(0.0, 1.0);
            } else {
                state = rng.random::<f32>();
            }
            self.cells[i] = Cell::new(state);
        }
        self.paused = true;
    }

    pub fn clear(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = Cell::new(0.0);
        }
    }

    pub fn life_around(&self, pos: IVec2) -> f32 {
        let mut result: f32 = 0.0;
        //convolution operation, we iterate over a square around the cell, the value added is based on the optimal distance (the radius)
        for x in -self.rule.radius..self.rule.radius+1 {
            for y in -self.rule.radius..self.rule.radius+1 {
                let neighbour: IVec2 = IVec2::new(x, y);
                let neighbour_cell: IVec2 = self.wrap_pos(pos+neighbour);
                let distance: f32 = (((pos+neighbour) - pos).length_squared() as f32).sqrt();
                if distance > self.rule.radius as f32 {
                    continue;
                }
                let ratio: f32 = 1.0 - distance/self.rule.radius as f32;
                result += self.cells.get(self.vector_to_idx(neighbour_cell) as usize).unwrap().state*ratio;
            }
        }
        result
    }

    pub fn idx_to_vector(&self, idx: i32) -> IVec2 {
        IVec2::new(idx%self.width as i32, idx/self.width as i32)
    }
    
    pub fn vector_to_idx(&self, pos: IVec2) -> i32 {
        pos.x%self.width as i32 + pos.y*self.width as i32
    }

    pub fn wrap_pos(&self, pos: IVec2) -> IVec2 {
        IVec2::new((pos.x + self.width as i32) % self.width as i32, (pos.y + self.height as i32) % self.height as i32)
    }

    pub fn generation (
        &self,
    ) -> Vec<Cell> {
        let mut result : Vec<Cell> = vec![Cell::default(); self.width*self.height];
        for (idx, _cell) in self.cells.iter().enumerate() {
            let life_around_value: f32 = self.life_around(self.idx_to_vector(idx as i32));
            let new_value: f32 = self.cells[idx].state + self.rule.growth(life_around_value)*self.rule.delta;
            result[idx] = Cell::new(new_value.clamp(0.0, 1.0));
        }
        result
    }

    pub fn pause(&mut self) {
        self.paused = !self.paused;
    }

}

pub fn update_generation (
    mut grid: ResMut<Grid>
) {
    if grid.paused {
        return;
    }
    let new_generation: Vec<Cell> = grid.generation();
    grid.cells = new_generation;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_idx_to_vector() {
        let grid = Grid::new(100, 50, 5.0);

        assert_eq!(grid.idx_to_vector(0), IVec2::new(0, 0));
        assert_eq!(grid.idx_to_vector(100), IVec2::new(0, 1));
        assert_eq!(grid.idx_to_vector(101), IVec2::new(1, 1));
        assert_eq!(grid.idx_to_vector(1010), IVec2::new(10, 10));
    }

    #[test]
    fn grid_vector_to_idx() {
        let grid = Grid::new(100, 50, 5.0);

        assert_eq!(grid.vector_to_idx(IVec2::new(0, 0)), 0);
        assert_eq!(grid.vector_to_idx(IVec2::new(0, 1)), 100);
        assert_eq!(grid.vector_to_idx(IVec2::new(1, 1)), 101);
        assert_eq!(grid.vector_to_idx(IVec2::new(10, 10)), 1010);
    }

    #[test]
    fn grid_wrap_pos() {
        let grid = Grid::new(100, 50, 5.0);

        assert_eq!(grid.wrap_pos(IVec2::new(0, 0)), IVec2::new(0, 0));
        assert_eq!(grid.wrap_pos(IVec2::new(-1, 0)), IVec2::new(99, 0));
        assert_eq!(grid.wrap_pos(IVec2::new(-1, -1)), IVec2::new(99, 49));
        assert_eq!(grid.wrap_pos(IVec2::new(0, -1)), IVec2::new(0, 49));
    }
}

#[derive(Resource, Debug, PartialEq)]
pub enum GenerationType {
    RANDOM,
    NOISE
}
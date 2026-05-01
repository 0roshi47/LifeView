use bevy::prelude::*;

use crate::cell::Cell;
use crate::grid_coloration::GridColoration;
use crate::rule::Rule;
use crate::shapes::Shape;

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
    pub generation_type: GenerationType,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut grid = Self {
            cells: vec![Cell::default(); width * height],
            width: width,
            height: height,
            cell_size: cell_size,
            rule: Rule::default(),
            grid_coloration: GridColoration::default(),
            paused: true,
            generation_type: GenerationType::RANDOM,
        };
        grid.init();
        grid
    }

    pub fn init(&mut self) {
        self.paused = true;
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let r = self.width.min(self.height) as f32 / 6.0;
        if self.generation_type == GenerationType::EMPTY {
            return;
        }
        for i in 0..self.cells.len() {
            if self.generation_type == GenerationType::RANDOM {
                let state = rand::rng().random();
                self.cells[i] = Cell::new(state);
                continue;
            }
            let pos = self.idx_to_vector(i as i32);
            let dx = pos.x as f32 - cx;
            let dy = pos.y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            // Smooth Gaussian blob — gives Lenia something to work with
            let state = (-0.5 * (dist / (r * 0.5)).powi(2)).exp();
            self.cells[i] = Cell::new(state);
        }
    }

    pub fn clear(&mut self) {
        for i in 0..self.cells.len() {
            self.cells[i] = Cell::new(0.0);
        }
    }

    pub fn life_around(&self, pos: IVec2) -> f32 {
        let mut result: f32 = 0.0;
        let r = self.rule.radius as f32;
        for x in -self.rule.radius..self.rule.radius + 1 {
            for y in -self.rule.radius..self.rule.radius + 1 {
                let neighbour = IVec2::new(x, y);
                let distance = (neighbour.as_vec2()).length();
                if distance > r {
                    continue;
                }
                let neighbour_cell = self.wrap_pos(pos + neighbour);
                let ratio: f32 = 1.0 - distance / r;
                result += self.cells[self.vector_to_idx(neighbour_cell) as usize].state * ratio;
            }
        }
        // Normalize by the actual sum of weights so result stays in [0,1]
        let kernel_sum: f32 = {
            let mut s = 0.0f32;
            for x in -self.rule.radius..self.rule.radius + 1 {
                for y in -self.rule.radius..self.rule.radius + 1 {
                    let d = (IVec2::new(x, y).as_vec2()).length();
                    if d <= r {
                        s += 1.0 - d / r;
                    }
                }
            }
            s
        };
        result / kernel_sum.max(1.0)
    }

    pub fn idx_to_vector(&self, idx: i32) -> IVec2 {
        IVec2::new(idx % self.width as i32, idx / self.width as i32)
    }

    pub fn vector_to_idx(&self, pos: IVec2) -> i32 {
        pos.x % self.width as i32 + pos.y * self.width as i32
    }

    pub fn wrap_pos(&self, pos: IVec2) -> IVec2 {
        IVec2::new(
            (pos.x + self.width as i32) % self.width as i32,
            (pos.y + self.height as i32) % self.height as i32,
        )
    }

    pub fn generation(&self) -> Vec<Cell> {
        let mut result: Vec<Cell> = vec![Cell::default(); self.width * self.height];
        for (idx, _cell) in self.cells.iter().enumerate() {
            let life_around_value: f32 = self.life_around(self.idx_to_vector(idx as i32));
            // println!("life_around[0]: {life_around_value}"); // remove
            let new_value: f32 =
                self.cells[idx].state + self.rule.growth(life_around_value) * self.rule.delta;
            result[idx] = Cell::new(new_value.clamp(0.0, 1.0));
            // break; // <-- REMOVE THIS, it only computes cell 0 and leaves all others at 0.0
        }
        result
    }

    pub fn pause(&mut self) {
        self.paused = !self.paused;
    }

    pub fn spawn_shape(&mut self, shape_name: String, shapes: Vec<Shape>) {
        let mut shape: Shape = shapes[0].clone();
        for current_shape in shapes {
            if current_shape.name == shape_name {
                shape = current_shape;
            }
        }
        let grid_center: IVec2 = IVec2::new(self.width as i32 / 2, self.height as i32 / 2);
        for i in 0..shape.cells_state.len() {
            let idx: usize = self.vector_to_idx(grid_center + shape.cells_pos[i]) as usize;
            self.cells[idx].state = shape.cells_state[i];
        }
    }
}

pub fn update_generation(grid: Option<ResMut<Grid>>) {
    let Some(mut grid) = grid else { return };
    if grid.paused {
        return;
    }
    let new_generation = grid.generation();
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

#[derive(Resource, Debug, PartialEq, Clone, Copy)]
pub enum GenerationType {
    EMPTY,
    RANDOM,
    BLOB,
}

use bevy::prelude::*;

use crate::cell::Cell;
use crate::grid_coloration::GridColoration;
use crate::rule::{KernelDef, Rule};
use crate::shapes::Shape;

use rand::Rng;

#[derive(Resource, Debug)]
pub struct Grid {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
    pub cell_size: f32,
    pub prev_cell_size: f32,
    pub rule: Rule,
    pub grid_coloration: GridColoration,
    pub paused: bool,
    pub generation_type: GenerationType,
    pub kernel_caches: Vec<KernelCache>,
    pub prev_kernel_sig: Vec<KernelSignature>,
}

#[derive(Debug, Clone)]
pub struct KernelCache {
    pub weights: Vec<(IVec2, f32)>,
    pub sum: f32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KernelSignature {
    pub base_radius: i32,
    pub relative_radius: f32,
    pub peaks: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let rule = Rule::default();
        let num_channels = rule.num_channels;
        let mut grid = Self {
            cells: vec![Cell::new(num_channels); width * height],
            width,
            height,
            cell_size,
            prev_cell_size: cell_size,
            rule,
            grid_coloration: GridColoration::default(),
            paused: true,
            generation_type: GenerationType::RANDOM,
            kernel_caches: Vec::new(),
            prev_kernel_sig: Vec::new(),
        };
        grid.rebuild_all_kernels();
        grid.init();
        grid
    }

    pub fn needs_rebuild(&self) -> bool {
        (self.cell_size - self.prev_cell_size).abs() > f32::EPSILON
    }

    pub fn kernels_need_rebuild(&self) -> bool {
        if self.rule.kernels.len() != self.prev_kernel_sig.len() {
            return true;
        }
        for (k, sig) in self.rule.kernels.iter().zip(&self.prev_kernel_sig) {
            let current_sig = KernelSignature {
                base_radius: k.base_radius,
                relative_radius: k.relative_radius,
                peaks: k.peaks.clone(),
            };
            if current_sig != *sig {
                return true;
            }
        }
        false
    }

    pub fn rebuild_all_kernels(&mut self) {
        self.kernel_caches.clear();
        self.prev_kernel_sig.clear();
        for kernel_def in &self.rule.kernels {
            let cache = Self::build_kernel(kernel_def);
            self.prev_kernel_sig.push(KernelSignature {
                base_radius: kernel_def.base_radius,
                relative_radius: kernel_def.relative_radius,
                peaks: kernel_def.peaks.clone(),
            });
            self.kernel_caches.push(cache);
        }
    }

    pub fn rebuild_kernel_for(&mut self, kernel_idx: usize) {
        if kernel_idx >= self.rule.kernels.len() {
            return;
        }
        let kernel_def = &self.rule.kernels[kernel_idx];
        let cache = Self::build_kernel(kernel_def);
        self.kernel_caches[kernel_idx] = cache;
        self.prev_kernel_sig[kernel_idx] = KernelSignature {
            base_radius: kernel_def.base_radius,
            relative_radius: kernel_def.relative_radius,
            peaks: kernel_def.peaks.clone(),
        };
    }

    fn build_kernel(kernel_def: &KernelDef) -> KernelCache {
        let effective_r = ((kernel_def.base_radius as f32) * kernel_def.relative_radius).ceil() as i32;
        let r = effective_r.max(1);
        let mut weights = Vec::new();
        let mut sum = 0.0;

        for x in -r..=r {
            for y in -r..=r {
                let d = IVec2::new(x, y).as_vec2().length();
                if d > r as f32 || d == 0.0 {
                    continue;
                }

                let t = d / r as f32;
                let w = Self::kernel_weight(t, &kernel_def.peaks);
                weights.push((IVec2::new(x, y), w));
                sum += w;
            }
        }

        KernelCache { weights, sum }
    }

    fn kernel_weight(t: f32, peaks: &[f32]) -> f32 {
        if peaks.len() == 1 {
            let bell_t = (t - 0.5) / 0.15;
            return (-bell_t.powi(2) / 2.0).exp();
        }

        let idx = (t * peaks.len() as f32).floor() as usize;
        let idx = idx.min(peaks.len() - 1);
        let frac = t * peaks.len() as f32 - idx as f32;
        let bell_frac = (frac - 0.5) / 0.15;
        peaks[idx] * (-bell_frac.powi(2) / 2.0).exp()
    }

    pub fn init(&mut self) {
        self.paused = true;
        let num_channels = self.rule.num_channels;
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let r = self.width.min(self.height) as f32 / 6.0;
        if self.generation_type == GenerationType::EMPTY {
            for i in 0..self.cells.len() {
                self.cells[i] = Cell::new(num_channels);
            }
            return;
        }
        for i in 0..self.cells.len() {
            if self.generation_type == GenerationType::RANDOM {
                let mut channels = Vec::with_capacity(num_channels);
                for _ in 0..num_channels {
                    channels.push(rand::rng().random());
                }
                self.cells[i] = Cell::with_state(channels);
                continue;
            }
            let pos = self.idx_to_vector(i as i32);
            let dx = pos.x as f32 - cx;
            let dy = pos.y as f32 - cy;
            let dist = (dx * dx + dy * dy).sqrt();
            let state = (-0.5 * (dist / (r * 0.5)).powi(2)).exp();
            let mut channels = vec![0.0; num_channels];
            channels[0] = state;
            self.cells[i] = Cell::with_state(channels);
        }
    }

    pub fn clear(&mut self) {
        self.paused = true;
        let num_channels = self.rule.num_channels;
        for i in 0..self.cells.len() {
            self.cells[i] = Cell::new(num_channels);
        }
    }

    pub fn life_around(&self, pos: IVec2, kernel_idx: usize, _channel: usize) -> f32 {
        let cache = &self.kernel_caches[kernel_idx];
        let kernel_def = &self.rule.kernels[kernel_idx];
        let source_channel = kernel_def.c0;

        let mut result: f32 = 0.0;
        for &(offset, w) in &cache.weights {
            let neighbour = self.wrap_pos(pos + offset);
            let cell = &self.cells[self.vector_to_idx(neighbour) as usize];
            if source_channel < cell.channels.len() {
                result += cell.channels[source_channel] * w;
            }
        }

        if cache.sum == 0.0 {
            0.0
        } else {
            result / cache.sum
        }
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
        let num_channels = self.rule.num_channels;
        let mut result: Vec<Cell> = (0..self.width * self.height)
            .map(|_| Cell::new(num_channels))
            .collect();

        for idx in 0..self.cells.len() {
            let pos = self.idx_to_vector(idx as i32);

            for c in 0..num_channels {
                let mut total_growth = 0.0;

                for (ki, kernel_def) in self.rule.kernels.iter().enumerate() {
                    if kernel_def.c1 != c {
                        continue;
                    }

                    let u = self.life_around(pos, ki, c);
                    let g = if kernel_def.use_target {
                        self.rule.target(u, ki) - self.cells[idx].channels[c]
                    } else {
                        self.rule.growth(u, ki)
                    };

                    total_growth += kernel_def.height * g;
                }

                let count_c1 = self.rule.kernels.iter().filter(|k| k.c1 == c).count().max(1);
                let avg_growth = total_growth / count_c1 as f32;
                let new_value = self.cells[idx].channels[c] + avg_growth * self.rule.delta;
                result[idx].channels[c] = new_value.clamp(0.0, 1.0);
            }
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

        self.rule = shape.optimal_rule.clone();
        if self.kernels_need_rebuild() {
            self.rebuild_all_kernels();
        }

        let num_channels = self.rule.num_channels;
        for i in 0..self.cells.len() {
            if self.cells[i].channels.len() != num_channels {
                self.cells[i] = Cell::new(num_channels);
            }
        }

        let grid_center: IVec2 = IVec2::new(self.width as i32 / 2, self.height as i32 / 2);
        for i in 0..shape.cells_state.len() {
            let idx: usize = self.vector_to_idx(grid_center + shape.cells_pos[i]) as usize;
            if idx < self.cells.len() {
                if num_channels == 1 {
                    self.cells[idx].channels[0] = shape.cells_state[i];
                } else if shape.cells_state.len() == self.cells.len() * num_channels {
                    let ch = i % num_channels;
                    if ch < self.cells[idx].channels.len() {
                        self.cells[idx].channels[ch] = shape.cells_state[i];
                    }
                } else {
                    self.cells[idx].channels[0] = shape.cells_state[i];
                }
            }
        }
    }
}

pub fn update_generation(grid: Option<ResMut<Grid>>) {
    let Some(mut grid) = grid else {
        return;
    };
    if grid.paused {
        return;
    }
    if grid.kernels_need_rebuild() {
        grid.rebuild_all_kernels();
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

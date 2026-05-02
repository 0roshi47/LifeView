use bevy::{
    ecs::system::{Commands, ResMut},
    math::IVec2,
};

use crate::rule::{Rule, StateType};

#[derive(Clone, Debug)]
pub struct Shape {
    pub name: String,
    pub optimal_rule: Rule,
    pub cells_state: Vec<f32>,
    pub cells_pos: Vec<IVec2>,
}

impl Shape {
    pub fn new(
        name: String,
        optimal_rule: Rule,
        cells_state: Vec<f32>,
        cells_pos: Vec<IVec2>,
    ) -> Self {
        Self {
            name,
            optimal_rule,
            cells_state,
            cells_pos,
        }
    }

    /// Build from a 2D row-major grid of f32 values, top-left origin.
    /// Zero cells are skipped (not spawned).
    pub fn from_grid(name: impl Into<String>, optimal_rule: Rule, grid: &[&[f32]]) -> Self {
        let rows = grid.len() as i32;
        let cols = grid[0].len() as i32;
        let origin = IVec2::new(cols / 2, rows / 2);
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for (y, row) in grid.iter().enumerate() {
            for (x, &val) in row.iter().enumerate() {
                if val > 0.0 {
                    cells_state.push(val);
                    // flip y so row 0 = top of pattern = positive y in grid
                    cells_pos.push(IVec2::new(x as i32, rows - 1 - y as i32) - origin);
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }

    pub fn disc(
        name: impl Into<String>,
        optimal_rule: Rule,
        r: i32,
        state_fn: impl Fn(f32) -> f32,
    ) -> Self {
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for x in -r..=r {
            for y in -r..=r {
                let dist = ((x * x + y * y) as f32).sqrt();
                if dist <= r as f32 {
                    let t = dist / r as f32;
                    cells_state.push(state_fn(t).clamp(0.0, 1.0));
                    cells_pos.push(IVec2::new(x, y));
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }

    pub fn ring(
        name: impl Into<String>,
        optimal_rule: Rule,
        r_inner: i32,
        r_outer: i32,
        state_fn: impl Fn(f32) -> f32,
    ) -> Self {
        let mut cells_state = Vec::new();
        let mut cells_pos = Vec::new();
        for x in -r_outer..=r_outer {
            for y in -r_outer..=r_outer {
                let dist = ((x * x + y * y) as f32).sqrt();
                if dist >= r_inner as f32 && dist <= r_outer as f32 {
                    let t = (dist - r_inner as f32) / (r_outer - r_inner).max(1) as f32;
                    cells_state.push(state_fn(t).clamp(0.0, 1.0));
                    cells_pos.push(IVec2::new(x, y));
                }
            }
        }
        Self::new(name.into(), optimal_rule, cells_state, cells_pos)
    }
}

#[derive(bevy::prelude::Resource, Debug, Default)]
pub struct Shapes(pub Vec<Shape>);

impl Shapes {
    pub fn add(&mut self, shape: Shape) {
        self.0.push(shape);
    }
}

pub fn insert_shapes(mut commands: Commands) {
    commands.insert_resource(Shapes(Vec::new()));
}

pub fn add_shapes(mut shapes: ResMut<Shapes>) {
    // Exact Orbium bicaudatus pattern from Bert Chan's original Lenia source.
    // R=13, mu=0.15, sigma=0.014, dt=0.1
    #[rustfmt::skip]
    let orbium_grid: &[&[f32]] = &[
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.10,0.14,0.10,0.00,0.00,0.03,0.03,0.00,0.00,0.30,0.00,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.08,0.24,0.30,0.30,0.18,0.14,0.15,0.16,0.15,0.09,0.20,0.00,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.15,0.34,0.44,0.46,0.38,0.18,0.14,0.11,0.13,0.19,0.18,0.45,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.06,0.13,0.39,0.50,0.50,0.37,0.06,0.00,0.00,0.00,0.02,0.16,0.68,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.11,0.17,0.17,0.33,0.40,0.38,0.28,0.14,0.00,0.00,0.00,0.00,0.00,0.18,0.42,0.00,0.00],
        &[0.00,0.00,0.09,0.18,0.13,0.06,0.08,0.26,0.32,0.32,0.27,0.00,0.00,0.00,0.00,0.00,0.00,0.82,0.00,0.00],
        &[0.27,0.00,0.16,0.12,0.00,0.00,0.00,0.25,0.38,0.44,0.45,0.34,0.00,0.00,0.00,0.00,0.00,0.22,0.17,0.00],
        &[0.00,0.07,0.20,0.02,0.00,0.00,0.00,0.31,0.48,0.57,0.60,0.57,0.00,0.00,0.00,0.00,0.00,0.00,0.49,0.00],
        &[0.00,0.59,0.19,0.00,0.00,0.00,0.00,0.20,0.57,0.69,0.76,0.76,0.49,0.00,0.00,0.00,0.00,0.00,0.36,0.00],
        &[0.00,0.58,0.19,0.00,0.00,0.00,0.00,0.00,0.67,0.83,0.90,0.92,0.87,0.12,0.00,0.00,0.00,0.00,0.22,0.07],
        &[0.00,0.00,0.46,0.00,0.00,0.00,0.00,0.00,0.70,0.93,1.00,1.00,1.00,0.61,0.00,0.00,0.00,0.00,0.18,0.11],
        &[0.00,0.00,0.82,0.00,0.00,0.00,0.00,0.00,0.47,1.00,1.00,0.98,1.00,0.96,0.27,0.00,0.00,0.00,0.19,0.10],
        &[0.00,0.00,0.46,0.00,0.00,0.00,0.00,0.00,0.25,1.00,1.00,0.84,0.92,0.97,0.54,0.14,0.00,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.09,0.74,1.00,0.55,0.63,0.86,0.76,0.48,0.00,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.17,0.53,0.26,0.31,0.52,0.67,0.56,0.42,0.00,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.06,0.25,0.43,0.53,0.24,0.00,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.02,0.14,0.19,0.05,0.00],
        &[0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.00,0.02,0.01,0.00],
    ];

    let orbium = Shape::from_grid(
        "Orbium",
        Rule::new(StateType::CONTINUOUS, 0.15, 0.014, 13),
        orbium_grid,
    );
    shapes.add(orbium);

    // Aquarium — ring seed, different parameters
    let aquarium = Shape::ring(
        "Aquarium",
        Rule::new(StateType::CONTINUOUS, 0.278, 0.036, 10),
        2,
        7,
        |t| (1.0 - t).powi(2),
    );
    shapes.add(aquarium);
}

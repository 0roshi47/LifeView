use std::ptr::null;

use bevy::{ecs::{name::Name, resource::Resource, system::{Commands, Res, ResMut}}, math::IVec2};

use crate::{cell::Cell, grid::Grid, rule::{Rule, StateType}};

#[derive(Clone, Debug)]
pub struct Shape {
    pub name: String,
    pub optimal_rule: Rule,
    pub cells_state: Vec<f32>,
    pub cells_pos: Vec<IVec2>
}

impl Shape {
    pub fn new(name: String, optimal_rule: Rule, cells_state: Vec<f32>, cells_pos: Vec<IVec2>) -> Self {
        Self {
            name: name,
            optimal_rule: optimal_rule,
            cells_state: cells_state, //value of each cell state
            cells_pos: cells_pos //relative cell pos from the center of the grid
        }
    }
}

#[derive(Resource, Debug, Default)]
pub struct Shapes(pub Vec<Shape>);

impl Shapes {
    pub fn add(&mut self, shape: Shape) {
        self.0.push(shape);
    }
}

pub fn insert_shapes(
    mut commands: Commands
) {
    commands.insert_resource(Shapes(Vec::new()));
}

pub fn add_shapes(
    mut shapes: ResMut<Shapes>
) {
    let orbium: Shape = Shape::new(
        "Orbium".to_string(),
        Rule::new(StateType::CONTINUOUS, 0.15, 0.015, 13),
        vec![
            1.0, // Center cell
            0.5, // Nearby cells with lower intensity
            1.0, // Active cells in standard configuration
            0.5, 
            0.0, 
            0.5, 
            1.0, 
            0.5, 
            0.0,
        ],
        vec![
            IVec2::new(0, 0),    // Center
            IVec2::new(0, 1),    // North
            IVec2::new(1, 0),    // East
            IVec2::new(0, -1),   // South
            IVec2::new(-1, 0),   // West
            IVec2::new(1, 1),    // Northeast
            IVec2::new(1, -1),   // Southeast
            IVec2::new(-1, -1),  // Southwest
            IVec2::new(-1, 1),   // Northwest
        ]);
    shapes.add(orbium);
}
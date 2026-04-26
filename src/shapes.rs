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
    let orbium = Shape::new(
        "Orbium".to_string(),
        Rule::new(StateType::CONTINUOUS, 0.15, 0.015, 13),
        vec![1.0, 0.5, 1.0, 0.5, 0.0, 0.5, 1.0, 0.5, 0.0],
        vec![
            IVec2::new(0, 0),
            IVec2::new(0, 1),
            IVec2::new(1, 0),
            IVec2::new(0, -1),
            IVec2::new(-1, 0),
            IVec2::new(1, 1),
            IVec2::new(1, -1),
            IVec2::new(-1, -1),
            IVec2::new(-1, 1),
        ],
    );
    shapes.add(orbium);
}

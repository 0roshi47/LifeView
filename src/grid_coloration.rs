use bevy::{
    math::VectorSpace, prelude::*
};

#[derive(Clone, Debug)]
pub struct GridColoration {
    pub life_color : LinearRgba,
    pub death_color : LinearRgba
}

impl Default for GridColoration {
    fn default() -> Self {
        Self { 
            life_color: LinearRgba::new(0.1, 0.0, 0.0, 1.0),
            death_color: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl GridColoration {
    pub fn new(life_color: LinearRgba, death_color: LinearRgba) -> Self {
        Self {
            life_color: life_color,
            death_color: death_color
        }
    }

    pub fn lerp(&self, x: f32) -> LinearRgba {
        self.life_color.lerp(self.death_color, x)
    }
}
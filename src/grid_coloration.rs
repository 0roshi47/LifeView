use bevy::{
    math::VectorSpace, prelude::*
};

#[derive(Clone, Debug)]
pub struct GridColoration {
    pub color_a : LinearRgba,
    pub color_b : LinearRgba,
    pub color_c : LinearRgba,
}

impl Default for GridColoration {
    fn default() -> Self {
        Self { 
            color_a : LinearRgba::new(1.0, 1.0, 0.0, 1.0),
            color_b : LinearRgba::new(0.0, 0.0, 1.0, 1.0),
            color_c : LinearRgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

impl GridColoration {
    pub fn lerp(&self, x: f32) -> LinearRgba {
        let interpolation_a: LinearRgba = self.color_a.lerp(self.color_b, x);
        let interpolation_b: LinearRgba = self.color_b.lerp(self.color_c, x);
        interpolation_b.lerp(interpolation_a, x)
        // self.color_c.lerp(self.color_a, x)
    }
}
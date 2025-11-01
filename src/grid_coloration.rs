use bevy::{
    math::VectorSpace, prelude::*
};

#[derive(Clone, Debug)]
pub struct GridColoration {
    pub color_range: Vec<LinearRgba>,
}

impl Default for GridColoration {
    fn default() -> Self {
        Self { 
            color_range: vec![
                LinearRgba::new(0.055, 0.0, 0.055, 1.0),
                LinearRgba::new(0.047, 0.38, 0.31, 1.0),
                LinearRgba::new(0.0, 1.0, 0.0, 1.0),
                LinearRgba::new(1.0, 1.0, 0.0, 1.0)],
        }
    }
}

impl GridColoration {
    pub fn lerp(&self, x: f32) -> LinearRgba {
        let ratio: f32 = (self.color_range.len()-1) as f32*x;
        let min_color_idx: usize = ratio.floor() as usize;
        let min_color: LinearRgba = self.color_range[min_color_idx];
        let max_color_idx: usize = ratio.ceil() as usize;
        let max_color: LinearRgba = self.color_range[max_color_idx];
        min_color.lerp(max_color, ratio - ratio.floor())
    }
}

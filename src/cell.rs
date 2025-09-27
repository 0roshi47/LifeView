#[derive(Clone, Debug)]
pub struct Cell {
    pub state : f32,
}

impl Cell {
    pub fn new(state: f32) -> Self {
        Self {
            state: state
        }
    }
}

impl Default for Cell {    
    fn default() -> Self {
        Self {
            state: 0.0
        }
    }
}
#[derive(Clone, Debug)]
pub struct Cell {
    pub channels: Vec<f32>,
}

impl Cell {
    pub fn new(num_channels: usize) -> Self {
        Self {
            channels: vec![0.0; num_channels],
        }
    }

    pub fn with_state(channels: Vec<f32>) -> Self {
        Self { channels }
    }

    pub fn num_channels(&self) -> usize {
        self.channels.len()
    }
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            channels: vec![0.0; 1],
        }
    }
}

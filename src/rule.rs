use bevy::math::ops::{exp, powf};

#[derive(Clone, Debug)]
pub struct Rule {
    pub state_type: StateType,
    pub micro: f32,
    pub sigma: f32,
    pub radius: usize
}

impl Rule {
    pub fn new(state_type: StateType, micro: f32, sigma: f32, radius: usize) -> Self {
        Self {
            state_type: state_type,
            micro: micro,
            sigma: sigma,
            radius: radius
        }
    }

    pub fn growth(&self, u: f32) -> f32 {
        2.0*exp(-powf(u-self.micro, 2.0)/2.0*powf(self.sigma, 2.0)) - 1.0
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            state_type: StateType::CONTINUOUS,
            micro: 0.4,
            sigma: 0.5,
            radius: 1
        }
    }
}

#[derive(Clone, Debug)]
enum StateType {
    CONTINUOUS, DISCRETE
}
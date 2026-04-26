use bevy::math::ops::exp;

#[derive(Clone, Debug)]
pub struct Rule {
    pub state_type: StateType,
    pub micro: f32,
    pub sigma: f32,
    pub radius: i32,
    pub delta: f32,
}

impl Rule {
    pub fn new(state_type: StateType, micro: f32, sigma: f32, radius: i32) -> Self {
        Self {
            state_type: state_type,
            micro: micro,
            sigma: sigma,
            radius: radius,
            delta: 1.0,
        }
    }

    pub fn growth(&self, u: f32) -> f32 {
        let diff: f32 = u - self.micro;
        2.0 * exp(-((diff * diff) / (2.0 * self.sigma * self.sigma))) - 1.0
    }
}

impl Default for Rule {
    fn default() -> Self {
        Self {
            state_type: StateType::CONTINUOUS,
            micro: 0.15,
            sigma: 0.015,
            radius: 13,
            delta: 0.1,
        }
    }
}

#[derive(Clone, Debug)]
pub enum StateType {
    CONTINUOUS,
    DISCRETE,
}

use bevy::math::ops::{exp, powf};

pub fn growth(u: f32) -> f32 {
    let micro: f32 = 0.4;
    let sigma: f32 = 0.5;
    2.0*exp(-powf(u-micro, 2.0)/2.0*powf(sigma, 2.0)) - 1.0
}

// enum GrowthMode {
//     Gaussian,
//     Sigmoid
// }
// Fluid simulation related content

use crate::param::*;
use crate::utils::*;

// TODO : Clip, set bounds, sum forces = const

// TODO : Special args for velocity (b)
// Diffuse fluid particules
pub fn diffuse(x_init: &[f32; N * N], x: &mut [f32; N * N], factor: f32, dt: f32) {
    let delta = dt * factor * (N * N) as f32;

    for _ in 0..RESOLUTION {
        for i in 1..N - 1 {
            for j in 1..N - 1 {
                let neighbors = x[grid2index(i - 1, j)] +
                        x[grid2index(i + 1, j)] +
                        x[grid2index(i, j + 1)] +
                        x[grid2index(i, j - 1)];

                x[grid2index(i, j)] = x_init[grid2index(i, j)] + delta * neighbors;
                x[grid2index(i, j)] /= 1.0 + 4.0 * delta;
            }
        }
        // TODO : Set bound
    }
}

// Fluid simulation related content

use crate::param::*;
use crate::utils::*;

pub enum BoundMode {
    Density,
    Vertical,
    Horizontal
}

// TODO : Clip, set bounds, sum forces = const

// TODO : Special args for velocity (b)
// Diffuse fluid particules
pub fn diffuse(x_init: &[f32; N * N], x: &mut [f32; N * N], factor: f32,
        dt: f32, vertical: BoundMode) {
    let delta = dt * factor * ((N - 2) * (N - 2)) as f32;

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

        set_bounds(x, BoundMode::Density);
    }
}

fn set_bounds(x: &mut [f32; N * N], mode: BoundMode) {
    // if mode != BoundMode::Density {
        // TODO : Vertical / Horizontal
    // }

    x[grid2index(0, 0)] = 0.5  *  (x[grid2index(1, 0)] + x[grid2index(0, 1)]);
    x[grid2index(0, N - 1)] = 0.5 * (x[grid2index(1, N - 1)] + x[grid2index(0, N - 2)]);
    x[grid2index(N - 1, 0)] = 0.5 * (x[grid2index(N - 2, 0)] + x[grid2index(N - 1, 1)]);
    x[grid2index(N - 1, N - 1)] = 0.5 * (x[grid2index(N - 2, N - 1)] + x[grid2index(N - 1, N - 2)]);
}

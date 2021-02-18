// Fluid simulation related content

use crate::param::*;
use crate::utils::*;

#[derive(PartialEq)]
pub enum BoundMode {
    Density,
    VelX,
    VelY,
}

// TODO : Clip, set bounds, sum forces = const

// TODO : Special args for velocity (b)
// Diffuse fluid particules
pub fn diffuse(x_init: &[f32; N * N], x: &mut [f32; N * N], factor: f32,
        dt: f32, mode: &BoundMode) {
    let delta = dt * factor * ((N - 2) * (N - 2)) as f32;

    for _ in 0..RESOLUTION {
        // TODO : Swap x_init and x ?
        for i in 1..N - 1 {
            for j in 1..N - 1 {
                // TODO : Verify
                let neighbors = x[grid2index(i - 1, j)] +
                        x[grid2index(i + 1, j)] +
                        x[grid2index(i, j + 1)] +
                        x[grid2index(i, j - 1)];
                // let neighbors = x_init[grid2index(i - 1, j)] +
                //         x_init[grid2index(i + 1, j)] +
                //         x_init[grid2index(i, j + 1)] +
                //         x_init[grid2index(i, j - 1)];

                x[grid2index(i, j)] = x_init[grid2index(i, j)] + delta * neighbors;
                x[grid2index(i, j)] /= 1.0 + 4.0 * delta;
            }
        }

        set_bounds(x, mode);
    }
}

// Move particles
pub fn advect(dens_init: &[f32; N * N], dens: &mut [f32; N * N],
        vel_x: &[f32; N * N],
        vel_y: &[f32; N * N],
        dt: f32, mode: &BoundMode) {
    let delta = dt * N as f32;

    for i in 1..N - 1 {
        for j in 1..N - 1 {
            let x = j as f32 - delta * vel_x[grid2index(i, j)];
            let y = i as f32 - delta * vel_y[grid2index(i, j)];

            let x = x.clamp(0.5, N as f32 - 0.5);
            let y = y.clamp(0.5, N as f32 - 0.5);

            let i_start = y as usize;
            let j_start = x as usize;
            let i_end = i_start + 1;
            let j_end = j_start + 1;

            let s_end = y - i_start as f32;
            let s_start = 1.0 - s_end as f32;
            let t_end = x - j_start as f32;
            let t_start = 1.0 - t_end as f32;

            // TODO : Explain
            dens[grid2index(i, j)] =
                    t_start * (
                        s_start * dens_init[grid2index(i_start, j_start)] +
                        s_end * dens_init[grid2index(i_start, i_end)]) +
                    t_end * (
                        s_start * dens_init[grid2index(i_end, j_start)] +
                        s_end * dens_init[grid2index(i_end, j_end)]);
        }
    }

    set_bounds(dens, mode);
}

fn set_bounds(x: &mut [f32; N * N], mode: &BoundMode) {
    if mode != &BoundMode::Density {
        for i in 1..N - 1 {
            x[grid2index(0, i)]     = -x[grid2index(1, i)];
            x[grid2index(N - 1, i)] = -x[grid2index(N - 2, i)];
            x[grid2index(i, 0)]     = x[grid2index(i, 1)];
            x[grid2index(i, N - 1)] = x[grid2index(i, N - 2)];

            if mode == &BoundMode::VelY {
                x[grid2index(0, i)]     = -x[grid2index(0, i)];
                x[grid2index(N - 1, i)] = -x[grid2index(N - 1, i)];
                x[grid2index(i, 0)]     = -x[grid2index(i, 0)];
                x[grid2index(i, N - 1)] = -x[grid2index(i, N - 1)];
            }
        }
    }

    x[grid2index(0, 0)] = 0.5 * (x[grid2index(1, 0)] + x[grid2index(0, 1)]);
    x[grid2index(0, N - 1)] = 0.5 * (x[grid2index(1, N - 1)] + x[grid2index(0, N - 2)]);
    x[grid2index(N - 1, 0)] = 0.5 * (x[grid2index(N - 2, 0)] + x[grid2index(N - 1, 1)]);
    x[grid2index(N - 1, N - 1)] = 0.5 * (x[grid2index(N - 2, N - 1)] + x[grid2index(N - 1, N - 2)]);
}

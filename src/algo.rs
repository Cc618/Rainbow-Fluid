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
pub fn diffuse(x_init: &Vec<f32>, x: &mut Vec<f32>, factor: f32,
        dt: f32, mode: &BoundMode) {
    let delta = dt * factor * ((N - 2) * (N - 2)) as f32;

    for i in 1..N - 1 {
        for j in 1..N - 1 {
            let neighbors = x_init[grid2index(i - 1, j)] +
                    x_init[grid2index(i + 1, j)] +
                    x_init[grid2index(i, j + 1)] +
                    x_init[grid2index(i, j - 1)];

            x[grid2index(i, j)] = x_init[grid2index(i, j)] + delta * neighbors;
            x[grid2index(i, j)] /= 1.0 + 4.0 * delta;
        }
    }

    set_bounds(x, mode);
}

// Move particles
pub fn advect(dens_init: &Vec<f32>, dens: &mut Vec<f32>,
        vel_x: &Vec<f32>,
        vel_y: &Vec<f32>,
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

            // Lerp
            dens[grid2index(i, j)] =
                    s_start * (
                        t_start * dens_init[grid2index(i_start, j_start)] +
                        t_end * dens_init[grid2index(i_start, j_end)]) +
                    s_end * (
                        t_start * dens_init[grid2index(i_end, j_start)] +
                        t_end * dens_init[grid2index(i_end, j_end)]);
        }
    }

    set_bounds(dens, mode);
}

// TODO : Rename p and div
// Conserve mass
// pub fn project(vel_x: &mut Vec<f32>, vel_y: &mut Vec<f32>,
        // p: &mut Vec<f32>, div: &mut Vec<f32>) {
    // let h = 1.0 / (N - 2) as f32;

    // for i in 1..N - 1 {
        // for j in 1..N - 1 {
        //     let idx = grid2index(i, j);

        //     div[idx] = -0.5 * h * (
        //         vel_x[grid2index(i, j + 1)] - vel_x[grid2index(i, j - 1)] +
        //         vel_y[grid2index(i + 1, j)] - vel_y[grid2index(i - 1, j)]);

        //     p[idx] = 0.0;
        // }
    // }

    // set_bounds(div, &BoundMode::Density);
    // set_bounds(p, &BoundMode::Density);

    // for _ in 0..RESOLUTION {
        // for i in 1..N - 1 {
        //     for j in 1..N - 1 {
        //         let idx = grid2index(i, j);

        //         p[idx] = div[idx] +
        //             p[grid2index(i - 1, j)] + p[grid2index(i + 1, j)] +
        //             p[grid2index(i, j - 1)] + p[grid2index(i, j + 1)];
        //         p[idx] /= 4.0;
        //     }
        // }

        // set_bounds(p, &BoundMode::Density);
    // }

    // for i in 1..N - 1 {
        // for j in 1..N - 1 {
        //     let idx = grid2index(i, j);

        //     // TODO : Divide opti
        //     vel_x[idx] -= 0.5 * (p[grid2index(i, j + 1)] - p[grid2index(i, j - 1)]) / h;
        //     vel_y[idx] -= 0.5 * (p[grid2index(i + 1, j)] - p[grid2index(i - 1, j)]) / h;
        // }
    // }

    // set_bounds(vel_x, &BoundMode::VelX);
    // set_bounds(vel_y, &BoundMode::VelY);
// }

fn set_bounds(x: &mut Vec<f32>, mode: &BoundMode) {
    for i in 1..N - 1 {
        // Left / right walls
        x[grid2index(0, i)]     = if mode == &BoundMode::VelY
                { -x[grid2index(1, i)] } else { x[grid2index(1, i)] };
        x[grid2index(N - 1, i)] = if mode == &BoundMode::VelY
                { -x[grid2index(N - 2, i)] } else { x[grid2index(N - 2, i)] };

        // Top / bottom walls
        x[grid2index(i, 0)]     = if mode == &BoundMode::VelX
                { -x[grid2index(i, 1)] } else { x[grid2index(i, 1)] };
        x[grid2index(i, N - 1)] = if mode == &BoundMode::VelX
                { -x[grid2index(i, N - 2)] } else { x[grid2index(i, N - 2)] };
    }

    x[grid2index(0, 0)] = 0.5 * (x[grid2index(1, 0)] + x[grid2index(0, 1)]);
    x[grid2index(0, N - 1)] = 0.5 * (x[grid2index(1, N - 1)] + x[grid2index(0, N - 2)]);
    x[grid2index(N - 1, 0)] = 0.5 * (x[grid2index(N - 2, 0)] + x[grid2index(N - 1, 1)]);
    x[grid2index(N - 1, N - 1)] = 0.5 * (x[grid2index(N - 2, N - 1)] + x[grid2index(N - 1, N - 2)]);
}

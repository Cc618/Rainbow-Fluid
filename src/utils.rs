// Tiny functions / macros used everywhere

use nannou::prelude::*;

use crate::param::*;

// Grid coordinates to screen coordinates
// [0, 0] is the bottom left corner, [N - 1, N - 1] the top right one
pub fn grid2screen(i: f32, j: f32, app: &App) -> (f32, f32) {
    let bounds = app.window_rect();

    (
        map_range(j, 0.0, N as f32, bounds.left(), bounds.right()),
        map_range(i, 0.0, N as f32, bounds.bottom(), bounds.top())
    )
}

// TODO : Macro
pub fn tile_size(app: &App) -> (f32, f32) {
    let (x_start, y_start) = grid2screen(0.0, 0.0, app);
    let (x_end, y_end) = grid2screen(1.0, 1.0, app);

    (abs(x_end - x_start), abs(y_end - y_start))
}

// TODO : Macro
pub fn grid2index(i: usize, j: usize) -> usize {
    i * N + j
}

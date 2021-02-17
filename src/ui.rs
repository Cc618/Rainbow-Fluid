use nannou::prelude::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    density: [f32; N * N],
}

pub fn model(_app: &App) -> Model {
    let mut model = Model {
        density: [0.0; N * N],
    };

    model.density[grid2index(0, 0)] = 0.8;
    model.density[grid2index(1, 1)] = 0.5;
    model.density[grid2index(2, 1)] = 1.0;
    model.density[grid2index(N - 1, N - 1)] = 0.8;

    model
}

pub fn event(_app: &App, _model: &mut Model, _event: Event) {
}

pub fn update(_app: &App, model: &mut Model, _: Update) {
    // TODO : Reserve only
    let mut new_density = [0.0; N * N];

    diffuse(&model.density, &mut new_density, DIFFUSION_FACTOR, 1.0 / 60.0);
    model.density = new_density;
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let (w_tile, h_tile) = tile_size(app);

    draw.background()
        .color(PLUM);

    for i in 0..N {
        for j in 0..N {
            let (x_start, y_start) = grid2screen(i as f32, j as f32, app);
            let d = model.density[grid2index(i, j)];

            draw.rect()
                .color(rgb(d * 0.9, d * 0.8, d))
                .w_h(w_tile, h_tile)
                .x_y(w_tile * 0.5 + x_start, h_tile * 0.5 + y_start);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}


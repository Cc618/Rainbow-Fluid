use nannou::prelude::*;
use nannou::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    density: [f32; N * N],
    mouse_pressed: bool,
}

pub fn model(app: &App) -> Model {
    // Create a window that can receive user input like mouse and keyboard events.
    app.new_window().event(event).view(view).build().unwrap();

    let mut model = Model {
        density: [0.0; N * N],
        mouse_pressed: false,
    };

    model.density[grid2index(0, 0)] = 0.8;
    model.density[grid2index(1, 1)] = 0.5;
    model.density[grid2index(2, 1)] = 1.0;
    model.density[grid2index(N - 1, N - 1)] = 0.8;

    model
}

pub fn event(app: &App, model: &mut Model, e: WindowEvent) {
    use WindowEvent::*;

    match e {
        MousePressed(pos) => model.mouse_pressed = true,
        MouseReleased(pos) => model.mouse_pressed = false,
        MouseMoved(pos) => if model.mouse_pressed { mouse_drag(model, app, pos); },
        _ => {},
    }
}

pub fn update(_app: &App, model: &mut Model, _: Update) {
    // TODO : Mv in algo
    // TODO : Reserve only
    let mut new_density = [0.0; N * N];
    let dt = 1.0 / 60.0;

    diffuse(&model.density, &mut new_density, DIFFUSION_FACTOR, dt,
            BoundMode::Density);

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

// When the mouse is pressed and moved
fn mouse_drag(model: &mut Model, app: &App, pos: Point2<f32>) {
    let (i, j) = screen2grid(pos.x, pos.y, app);

    if i < N && j < N {
        model.density[grid2index(i, j)] = 1.0;
    }
}

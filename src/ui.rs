use nannou::prelude::*;
use nannou::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    density: [f32; N * N],
    vel_x: [f32; N * N],
    vel_y: [f32; N * N],
    mouse_pressed: bool,
}

pub fn model(app: &App) -> Model {
    // Create a window that can receive user input like mouse and keyboard events.
    app.new_window().event(event).view(view).build().unwrap();

    let mut model = Model {
        density: [0.0; N * N],
        vel_x: [0.0; N * N],
        vel_y: [0.0; N * N],
        mouse_pressed: false,
    };

    // TODO :
    model.vel_x[N + 1] = 1.0;
    model.vel_y[N + 1] = 1.0;
    model.density[N + 1] = 1.0;

    model
}

pub fn event(app: &App, model: &mut Model, e: WindowEvent) {
    use WindowEvent::*;

    match e {
        MousePressed(_pos) => model.mouse_pressed = true,
        MouseReleased(_pos) => model.mouse_pressed = false,
        MouseMoved(pos) => if model.mouse_pressed { mouse_drag(model, app, pos); },
        _ => {},
    }
}

pub fn update(_app: &App, model: &mut Model, _: Update) {
    // TODO : Mv in algo
    // TODO : Reserve only
    let dt = 1.0 / 30.0;
    let mut new_density = [0.0; N * N];
    let mut new_vel_x = [0.0; N * N];
    let mut new_vel_y = [0.0; N * N];

    // Density update
    diffuse(&model.density, &mut new_density, DIFFUSION_FACTOR,
            dt, &BoundMode::Density);

    advect(&new_density, &mut model.density,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::Density);

    // Velocity update
    // diffuse(&model.vel_x, &mut new_vel_x, DIFFUSION_FACTOR,
    //         dt, &BoundMode::VelX);

    // diffuse(&model.vel_y, &mut new_vel_y, DIFFUSION_FACTOR,
    //         dt, &BoundMode::VelY);

    // model.density = new_density;
    model.vel_x = new_vel_x;
    model.vel_y = new_vel_y;
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

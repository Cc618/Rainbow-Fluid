use nannou::prelude::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    // Env
    density: Vec<f32>,
    vel_x: Vec<f32>,
    vel_y: Vec<f32>,
    src_density: Vec<f32>,
    src_vel_x: Vec<f32>,
    src_vel_y: Vec<f32>,

    // Mouse
    mouse_pressed: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
    drag_mode: bool,
}

pub fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();

    let model = Model {
        // Env
        density: vec![0.0; N * N],
        vel_x: vec![0.0; N * N],
        vel_y: vec![0.0; N * N],
        src_density: vec![0.0; N * N],
        src_vel_x: vec![0.0; N * N],
        src_vel_y: vec![0.0; N * N],
        // UI
        mouse_pressed: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        mouse_dx: 0.0,
        mouse_dy: 0.0,
        drag_mode: false,
    };

    model
}

pub fn event(app: &App, model: &mut Model, e: WindowEvent) {
    use WindowEvent::*;

    match e {
        KeyPressed(key) => key_press(key, model),
        MousePressed(_pos) => model.mouse_pressed = true,
        MouseReleased(_pos) => model.mouse_pressed = false,
        MouseMoved(pos) => if model.mouse_pressed { mouse_drag(model, app, pos); },
        _ => {},
    }
}

pub fn update(app: &App, model: &mut Model, _: Update) {
    let dt = 1.0 / FPS;

    // Update mouse
    let mouse_pos = app.mouse.position();
    model.mouse_dx = mouse_pos.x - model.last_mouse_x;
    model.mouse_dy = mouse_pos.y - model.last_mouse_y;
    model.last_mouse_x = mouse_pos.x;
    model.last_mouse_y = mouse_pos.y;

    // TODO : Mv in algo
    // TODO : Reserve only / copy ?
    let mut new_density = vec![0.0; N * N];
    let mut new_vel_x = vec![0.0; N * N];
    let mut new_vel_y = vec![0.0; N * N];

    // Apply density
    apply_source(&mut model.density, &mut model.src_density, dt);

    // Density diffuse
    diffuse(&model.density, &mut new_density, dt, &BoundMode::Density);

    std::mem::swap(&mut model.density, &mut new_density);

    // Density advect
    advect(&model.density, &mut new_density,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::Density);

    std::mem::swap(&mut model.density, &mut new_density);

    // Apply velocity
    apply_source(&mut model.vel_x, &mut model.src_vel_x, dt);
    apply_source(&mut model.vel_y, &mut model.src_vel_y, dt);

    // Velocity diffuse
    diffuse(&model.vel_x, &mut new_vel_x,
            dt, &BoundMode::VelX);

    diffuse(&model.vel_y, &mut new_vel_y,
            dt, &BoundMode::VelY);

    std::mem::swap(&mut model.vel_x, &mut new_vel_x);
    std::mem::swap(&mut model.vel_y, &mut new_vel_y);

    // Velocity conserve mass
    project(&mut model.vel_x, &mut model.vel_y,
            &mut new_vel_x, &mut new_vel_y);

    // Velocity advect
    advect(&model.vel_x, &mut new_vel_x,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::VelY);

    advect(&model.vel_y, &mut new_vel_y,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::VelY);

    std::mem::swap(&mut model.vel_x, &mut new_vel_x);
    std::mem::swap(&mut model.vel_y, &mut new_vel_y);

    // Velocity conserve mass
    project(&mut model.vel_x, &mut model.vel_y,
            &mut new_vel_x, &mut new_vel_y);
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw_vel = false;

    let draw = app.draw();
    let (w_tile, h_tile) = tile_size(app);

    draw.background()
        .color(BLACK);

    // Tiles
    for i in 0..N {
        for j in 0..N {
            let (x_start, y_start) = grid2screen(i as f32, j as f32, app);
            let d = model.density[grid2index(i, j)].clamp(0.0, 1.0);

            let color = if d < 0.5 { rgb(2.0 * d, 1.5 * d, 0.0) }
                    else { rgb(1.0, 0.75 + 0.25 * d,  d) };

            draw.rect()
                .color(color)
                .w_h(w_tile, h_tile)
                .x_y(w_tile * 0.5 + x_start, h_tile * 0.5 + y_start);
        }
    }

    // Arrows
    if draw_vel {
        for i in 0..N {
            for j in 0..N {
                let (x_start, y_start) = grid2screen(i as f32, j as f32, app);
                let (x_start, y_start) = (x_start + 0.5 * w_tile, y_start + 0.5 * h_tile);

                let idx = grid2index(i, j);
                let dx = model.vel_x[idx] / MOUSE_SENSIVITY;
                let dy = model.vel_y[idx] / MOUSE_SENSIVITY;

                let (x_end, y_end) = (x_start + dx * w_tile, y_start + dy * h_tile);

                draw.line()
                    .color(rgb(1.0, 0.1, 0.1))
                    .start(pt2(x_start, y_start))
                    .end(pt2(x_end, y_end));
            }
        }
    }

    draw.to_frame(app, &frame).unwrap();
}

// TODO : Add density, update density, add velocity, update velocity
// When the mouse is pressed and moved
fn mouse_drag(model: &mut Model, app: &App, pos: Point2<f32>) {
    let dt = 1.0 / FPS;
    let (i, j) = screen2grid(pos.x, pos.y, app);

    if i < N && j < N {
        let idx = grid2index(i, j);

        // TODO : Brush size

        if !model.drag_mode {
            let speed = (model.mouse_dx * model.mouse_dx +
                         model.mouse_dy * model.mouse_dy).sqrt() / (N as f32 * 1.41);
            model.src_density[idx] += dt * speed * MOUSE_DENSITY;
        }

        model.src_vel_x[idx] += model.mouse_dx * dt * MOUSE_SENSIVITY;
        model.src_vel_y[idx] += model.mouse_dy * dt * MOUSE_SENSIVITY;
    }
}

fn key_press(key: Key, model: &mut Model) {
    match key {
        Key::R => reset(model),
        Key::Space => model.drag_mode = !model.drag_mode,
        _ => {}
    }
}

// Reset density and velocity
fn reset(model: &mut Model) {
    for i in 0..N * N {
        model.density[i] = 0.0;
        model.vel_x[i] = 0.0;
        model.vel_y[i] = 0.0;
        model.src_density[i] = 0.0;
        model.src_vel_x[i] = 0.0;
        model.src_vel_y[i] = 0.0;
    }
}

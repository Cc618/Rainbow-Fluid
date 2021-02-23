use nannou::prelude::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    // Env
    // (r, g, b) = densities
    pub r: Vec<f32>,
    pub g: Vec<f32>,
    pub b: Vec<f32>,
    pub vel_x: Vec<f32>,
    pub vel_y: Vec<f32>,
    pub new_r: Vec<f32>,
    pub new_g: Vec<f32>,
    pub new_b: Vec<f32>,
    pub new_vel_x: Vec<f32>,
    pub new_vel_y: Vec<f32>,
    pub src_r: Vec<f32>,
    pub src_g: Vec<f32>,
    pub src_b: Vec<f32>,
    pub src_vel_x: Vec<f32>,
    pub src_vel_y: Vec<f32>,

    // UI
    mouse_pressed: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
    drag_mode: bool,
    mode: usize,
    brush_r: f32,
    brush_g: f32,
    brush_b: f32,
    frame_id: usize,
}

pub fn model(app: &App) -> Model {
    app.new_window().event(event).view(view).build().unwrap();

    let mut model = Model {
        // Env
        r: vec![0.0; N * N],
        g: vec![0.0; N * N],
        b: vec![0.0; N * N],
        vel_x: vec![0.0; N * N],
        vel_y: vec![0.0; N * N],
        new_r: vec![0.0; N * N],
        new_g: vec![0.0; N * N],
        new_b: vec![0.0; N * N],
        new_vel_x: vec![0.0; N * N],
        new_vel_y: vec![0.0; N * N],
        src_r: vec![0.0; N * N],
        src_g: vec![0.0; N * N],
        src_b: vec![0.0; N * N],
        src_vel_x: vec![0.0; N * N],
        src_vel_y: vec![0.0; N * N],

        // UI
        mouse_pressed: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        mouse_dx: 0.0,
        mouse_dy: 0.0,
        drag_mode: false,
        mode: 0,
        brush_r: 0.9,
        brush_g: 0.7,
        brush_b: 0.1,
        frame_id: 0,
    };

    // Subtractive color mode
    if COLOR_MODE == 1 {
        model.brush_r = 1.0 - model.brush_r;
        model.brush_g = 1.0 - model.brush_g;
        model.brush_b = 1.0 - model.brush_b;
    }

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
    model.frame_id += 1;

    // Update mouse
    let mouse_pos = app.mouse.position();
    model.mouse_dx = mouse_pos.x - model.last_mouse_x;
    model.mouse_dy = mouse_pos.y - model.last_mouse_y;
    model.last_mouse_x = mouse_pos.x;
    model.last_mouse_y = mouse_pos.y;

    // Update physics
    let dt = 1.0 / FPS;
    update_env(model, dt);

    // Update example mode
    update_mode(model);
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

            // The color is white if the density is very high for all values
            let colormap = |c: f32| {
                const RAMP: f32 = 0.1;

                if c > 0.8 {
                    (0.8 + (c - 0.8) * RAMP).min(1.0)
                } else {
                    c.max(0.0)
                }
            };

            let r = colormap(model.r[grid2index(i, j)]);
            let g = colormap(model.g[grid2index(i, j)]);
            let b = colormap(model.b[grid2index(i, j)]);

            let color = if COLOR_MODE == 0 {
                rgb(r, g, b)
            } else {
                rgb(1.0 - r, 1.0 - g, 1.0 - b)
            };

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

    app.main_window().capture_frame(format!("render/frame{:05}.png", model.frame_id));
}

// When the mouse is pressed and moved
fn mouse_drag(model: &mut Model, app: &App, pos: Point2<f32>) {
    let dt = 1.0 / FPS;
    let (i, j) = screen2grid(pos.x, pos.y, app);
    let brush_factor = 1.0 / (BRUSH_N * BRUSH_N) as f32;

    for w in -BRUSH_N..BRUSH_N + 1 {
        for h in -BRUSH_N..BRUSH_N + 1 {
            let brush_i = i as i32 + h;
            let brush_j = j as i32 + w;

            // Negative / positive overflow
            if brush_i < 0 || brush_j < 0 ||
                    brush_i >= N as i32 || brush_j >= N as i32 {
                continue;
            }

            let idx = grid2index(brush_i as usize, brush_j as usize);

            if !model.drag_mode {
                let speed = (model.mouse_dx * model.mouse_dx +
                            model.mouse_dy * model.mouse_dy).sqrt() / (N as f32 * 1.41);
                let density = dt * brush_factor * speed * MOUSE_DENSITY;

                model.src_r[idx] += model.brush_r * density;
                model.src_g[idx] += model.brush_g * density;
                model.src_b[idx] += model.brush_b * density;
            }

            model.src_vel_x[idx] += model.mouse_dx * dt * brush_factor * MOUSE_SENSIVITY;
            model.src_vel_y[idx] += model.mouse_dy * dt * brush_factor * MOUSE_SENSIVITY;
        }
    }
}

fn key_press(key: Key, model: &mut Model) {
    match key {
        Key::R => reset(model),
        Key::B => change_brush_color(model),
        Key::Space => model.drag_mode = !model.drag_mode,
        Key::Left => model.mode = (model.mode + N_MODES - 1) % N_MODES,
        Key::Right => model.mode = (model.mode + 1) % N_MODES,
        _ => {}
    }
}

// Reset density and velocity
fn reset(model: &mut Model) {
    for i in 0..N * N {
        model.r[i] = 0.0;
        model.g[i] = 0.0;
        model.b[i] = 0.0;
        model.vel_x[i] = 0.0;
        model.vel_y[i] = 0.0;
        model.src_r[i] = 0.0;
        model.src_g[i] = 0.0;
        model.src_b[i] = 0.0;
        model.src_vel_x[i] = 0.0;
        model.src_vel_y[i] = 0.0;
    }
}

fn update_mode(model: &mut Model) {
    const TILE_ON_DENSITY: f32 = 5e1;
    const TILE_ON_UPFORCE: f32 = 1e0;
    const TILE_ON_FORCE: f32 = 3e2;
    const TILE_ON_WIND: f32 = 2e-1;
    const TILE_ON_GRAVITY: f32 = -8e-1;
    let _dt = 1.0 / FPS;

    // Fire
    if model.mode == 1 {
        let idx = grid2index(N / 2, N / 2);
        model.src_r[idx] = TILE_ON_DENSITY;
        model.src_g[idx] = TILE_ON_DENSITY;
        model.src_b[idx] = TILE_ON_DENSITY;

        for i in 0..N * N {
            model.src_vel_x[i] = TILE_ON_WIND;
            model.src_vel_y[i] = TILE_ON_UPFORCE;
        }
    } else if model.mode == 2 {
        model.src_r[grid2index(N / 2, 4)] = TILE_ON_DENSITY;
        model.src_r[grid2index(N / 2, N - 1 - 4)] = TILE_ON_DENSITY;
        model.src_g[grid2index(N / 2, 4)] = TILE_ON_DENSITY;
        model.src_g[grid2index(N / 2, N - 1 - 4)] = TILE_ON_DENSITY;
        model.src_r[grid2index(N / 2, 4)] = TILE_ON_DENSITY;
        model.src_b[grid2index(N / 2, N - 1 - 4)] = TILE_ON_DENSITY;

        for i in 0..N * N {
            model.src_vel_y[i] = TILE_ON_GRAVITY;
        }

        model.src_vel_x[grid2index(N / 2, 4)] = TILE_ON_FORCE;
        model.src_vel_x[grid2index(N / 2, N - 1 - 4)] = -TILE_ON_FORCE;
    }
}

fn change_brush_color(model: &mut Model) {
    let tmp = model.brush_r;

    model.brush_r = model.brush_g;
    model.brush_g = model.brush_b;
    model.brush_b = tmp;
}

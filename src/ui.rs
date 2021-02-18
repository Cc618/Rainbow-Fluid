use nannou::prelude::*;

use crate::utils::*;
use crate::param::*;
use crate::algo::*;

pub struct Model {
    // Env
    density: Vec<f32>,
    vel_x: Vec<f32>,
    vel_y: Vec<f32>,

    // Mouse
    mouse_pressed: bool,
    last_mouse_x: f32,
    last_mouse_y: f32,
    mouse_dx: f32,
    mouse_dy: f32,
}

pub fn model(app: &App) -> Model {
    // Create a window that can receive user input like mouse and keyboard events.
    app.new_window().event(event).view(view).build().unwrap();

    let mut model = Model {
        density: vec![0.0; N * N],
        vel_x: vec![0.0; N * N],
        vel_y: vec![0.0; N * N],
        mouse_pressed: false,
        last_mouse_x: 0.0,
        last_mouse_y: 0.0,
        mouse_dx: 0.0,
        mouse_dy: 0.0,
    };

    model
}

pub fn event(app: &App, model: &mut Model, e: WindowEvent) {
    use WindowEvent::*;

    match e {
        KeyPressed(key) => if key == Key::R { reset(model); },
        MousePressed(_pos) => model.mouse_pressed = true,
        MouseReleased(_pos) => model.mouse_pressed = false,
        MouseMoved(pos) => if model.mouse_pressed { mouse_drag(model, app, pos); },
        _ => {},
    }
}

// TODO : rm
// fn log_type<T>(_: &T) {
//     println!("Type : {}", std::any::type_name::<T>())
// }

pub fn update(app: &App, model: &mut Model, _: Update) {
    let dt = 1.0 / 30.0;

    // TODO
    model.density[grid2index(N / 2, N / 2)] = 1.0;

    // Update mouse
    let mouse_pos = app.mouse.position();
    model.mouse_dx = (mouse_pos.x - model.last_mouse_x) * MOUSE_SENSIVITY;
    model.mouse_dy = (mouse_pos.y - model.last_mouse_y) * MOUSE_SENSIVITY;
    model.last_mouse_x = mouse_pos.x;
    model.last_mouse_y = mouse_pos.y;


    // TODO : Mv in algo
    // TODO : Reserve only / copy ?
    let mut new_density = vec![0.0; N * N];
    let mut new_vel_x = vec![0.0; N * N];
    let mut new_vel_y = vec![0.0; N * N];

    // Density update
    diffuse(&model.density, &mut new_density, DIFFUSION_FACTOR,
            dt, &BoundMode::Density);

    std::mem::swap(&mut model.density, &mut new_density);

    advect(&model.density, &mut new_density,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::Density);

    std::mem::swap(&mut model.density, &mut new_density);

    // Velocity update
    diffuse(&model.vel_x, &mut new_vel_x, DIFFUSION_FACTOR,
            dt, &BoundMode::VelX);

    std::mem::swap(&mut model.vel_x, &mut new_vel_x);

    diffuse(&model.vel_y, &mut new_vel_y, DIFFUSION_FACTOR,
            dt, &BoundMode::VelY);

    std::mem::swap(&mut model.vel_y, &mut new_vel_y);

    // TODO : Verify vel x and y not inversed
    project(&mut model.vel_x, &mut model.vel_y,
            &mut new_vel_x, &mut new_vel_y);

    std::mem::swap(&mut model.vel_x, &mut new_vel_x);
    std::mem::swap(&mut model.vel_y, &mut new_vel_y);

    // TODO : Not the same in the parper
    advect(&model.vel_x, &mut new_vel_x,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::Density);

    advect(&model.vel_y, &mut new_vel_y,
            &model.vel_x, &model.vel_y,
            dt, &BoundMode::Density);

    std::mem::swap(&mut model.vel_x, &mut new_vel_x);
    std::mem::swap(&mut model.vel_y, &mut new_vel_y);







//     // let (mut new_vel_x, mut new_vel_x_2) = (new_vel_x_2, new_vel_x);
//     // let (mut new_vel_y, mut new_vel_y_2) = (new_vel_y_2, new_vel_y);
//     std::mem::swap(new_vel_x, new_vel_x_2);
//     std::mem::swap(new_vel_y, new_vel_y_2);

//     project(&mut new_vel_x, &mut new_vel_y,
//             &mut new_vel_x_2, &mut new_vel_y_2);

//     // let (mut new_vel_x, mut new_vel_x_2) = (new_vel_x_2, new_vel_x);
//     // let (mut new_vel_y, mut new_vel_y_2) = (new_vel_y_2, new_vel_y);
//     std::mem::swap(new_vel_x, new_vel_x_2);
//     std::mem::swap(new_vel_y, new_vel_y_2);

//     // TODO : Not y for 3rd arg ?
//     advect(&new_vel_x, &mut new_vel_x_2,
//             &new_vel_x_2, &new_vel_y_2,
//             dt, &BoundMode::VelX);

//     advect(&new_vel_y, &mut new_vel_y_2,
//             &new_vel_x_2, &new_vel_y_2,
//             dt, &BoundMode::VelY);

//     project(&mut new_vel_x, &mut new_vel_y,
//             &mut new_vel_x, &mut new_vel_y_2);


//     // TODO : Use refs ?
//     // model.density = new_density;
//     model.vel_x = *new_vel_x;
//     model.vel_y = *new_vel_y;
}

pub fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let (w_tile, h_tile) = tile_size(app);

    draw.background()
        .color(BLACK);

    // Tiles
    for i in 0..N {
        for j in 0..N {
            let (x_start, y_start) = grid2screen(i as f32, j as f32, app);
            let d = model.density[grid2index(i, j)].clamp(0.0, 1.0);

            draw.rect()
                .color(rgb(d * 0.9, d * 0.8, d))
                .w_h(w_tile, h_tile)
                .x_y(w_tile * 0.5 + x_start, h_tile * 0.5 + y_start);
        }
    }

    // Arrows
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

    draw.to_frame(app, &frame).unwrap();
}

// TODO : Add density, update density, add velocity, update velocity
// When the mouse is pressed and moved
fn mouse_drag(model: &mut Model, app: &App, pos: Point2<f32>) {
    let (i, j) = screen2grid(pos.x, pos.y, app);

    if i < N && j < N {
        let idx = grid2index(i, j);

        model.density[idx] = 1.0;

        if model.mouse_dx != 0.0 || model.mouse_dy != 0.0 {
            // Normalize
            let norm = model.mouse_dx * model.mouse_dx + model.mouse_dy * model.mouse_dy;
            let norm = norm.sqrt();

            model.vel_x[idx] = model.mouse_dx / norm * MOUSE_SENSIVITY;
            model.vel_y[idx] = model.mouse_dy / norm * MOUSE_SENSIVITY;
        }
    }
}

// Reset density and velocity
fn reset(model: &mut Model) {
    for i in 0..N * N {
        model.density[i] = 0.0;
        model.vel_x[i] = 0.0;
        model.vel_y[i] = 0.0;
    }
}

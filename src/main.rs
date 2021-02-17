mod ui;
mod algo;
mod utils;
mod param;

use ui::*;

fn main() {
    nannou::app(model)
        .event(event)
        .update(update)
        .simple_window(view)
        .run();
}

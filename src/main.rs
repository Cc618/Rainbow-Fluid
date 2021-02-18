mod ui;
mod algo;
mod utils;
mod param;

use ui::*;

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

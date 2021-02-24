mod ui;
mod algo;
mod utils;
mod param;

use ui::*;

fn main() {
    println!("# Welcome to Rainbow Fluid Simulation !");
    println!("- Drag using your mouse to paint the canvas");
    println!("- Frames can be recorded within the render directory");
    println!("* For further details about controls and more, see the README.md file");

    nannou::app(model)
        .update(update)
        .run();
}

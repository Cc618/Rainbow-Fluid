mod ui;
mod algo;
mod utils;
mod param;

use ui::*;

fn main() {
    println!("# Welcome to Rainbow Fluid Simulation !");
    println!("- Drag using your mouse to paint the canvas");
    println!("- Press R to reset, B to change brush");
    println!("- Frames can be recorded within the render directory");
    println!("* For further details about controls / tweaking etc., see the README.md file");

    nannou::app(model)
        .update(update)
        .run();
}

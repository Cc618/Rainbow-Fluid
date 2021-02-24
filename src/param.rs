// Constants

// Grid size
pub const N: usize = 64;

// Brush radius size for user interaction
pub const BRUSH_N: i32 = N as i32 / 16;

// Frame rate of the application (used to compute the delta time)
pub const FPS: f32 = 20.0;

// Iterations, the higher, the more accurate the simulation is
pub const RESOLUTION: usize = 20;

// How much the fluid expands
pub const DIFFUSION_FACTOR: f32 = 9e-3;

// How much velocity / density when we move the mouse
pub const MOUSE_SENSIVITY: f32 = 9e0;
pub const MOUSE_DENSITY: f32 = 62e1;

// 0 = No mode
// 1 = Fire mode
// 2 = Falls mode
pub const N_MODES: usize = 3;

// 0 = Additive
// 1 = Subtractive
pub const COLOR_MODE: usize = 1;

// Whether we render images in render/
pub const SAVE_RENDER: bool = false;

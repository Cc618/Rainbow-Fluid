# Rainbow Fluid Simulation
Grid based RGB fluid simulation.

## Features
- Made in Rust with the Nannou library
- Grid-based (eulerian) simulation
- RGB support, 3 densities are used, one for each fluid
- Controls to change brushes / reset grid...

## Screenshots
Since rendering large grids can be expensive, we can save the simulation as a GIF image with a constant frame-rate.
Here are some examples :

![128px Grid, Additive colors](renders/128_add.gif)
![128px Grid, Subtractive colors](renders/128_sub_2.gif)
![128px Grid, Subtractive colors](renders/128_sub_3.gif)
![128px Grid, Additive colors](renders/128_add_2.gif)

## Controls
- Mouse Drag : "Paint" the canvas (add density and velocity)
- R : Reset the canvas
- B : Change brush color
- Left / Right : Change experiment mode
- Space : Toggle density (add only velocity on mouse drag)

## Tweaking
The file [params.rs](src/params.rs) provides customizable parameters :

- N : Grid size
- COLOR_MODE : 0 = Additive, 1 = Subtractive
- SAVE_RENDER : Whether we render images in render/
- DIFFUSION_FACTOR : How much the fluid expands
- MOUSE_SENSIVITY : How much velocity when we move the mouse
- MOUSE_DENSITY : How much density when we move the mouse
- BRUSH_N : Brush radius size for user interaction
- FPS : Frame rate of the application (used to compute the delta time)
- RESOLUTION : Iterations, the higher, the more accurate the simulation is

Most used parameters are N, COLOR_MODE and SAVE_RENDER.

## References
This implementation of real-time eulerian fluid dynamics is inspired by multiple papers / implementations :

- [Real-Time Fluid Dynamics for Games (GDC 3) paper](https://www.dgp.toronto.edu/public_user/stam/reality/Research/pdf/GDC03.pdf)
- [Fluid Simulation thesis](https://www.cs.ubc.ca/~rbridson/fluidsimulation/fluids_notes.pdf)
- [Parallel Grid-Based Fluid Simulation website](https://sstritte.github.io/fluid-sim/)
- [pf20-fluid-grid repo (Processing)](https://github.com/cc1539/pf20-fluid-grid)
- [Eulerian Fluid Simulation repo (Processing)](https://github.com/antoinefournier/Eulerian-Fluid-Simulation)

## Notes
This is my first project in Rust, thus the code might be not very clean at some extent.

## License
[MIT License](LICENSE)

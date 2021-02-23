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
![128px Grid, Additive colors](renders/128_add_2.gif)
![128px Grid, Subtractive colors](renders/128_sub_2.gif)
![128px Grid, Subtractive colors](renders/128_sub_3.gif)

<!-- TODO : Controls -->

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

# Gravitation Particles

A Barnes-Hut implementation of n-body gravitation simulation in Rust.

![Gravitation Particles Demo](https://github.com/BaGreal2/gravitation-particles/blob/main/1.gif?raw=true)

# Running the project

The video rendering feature is implemented using the `ffmpeg`, so you will need that installed. Assuming you also have `cargo` installed, all you need to run a project is simply:

```bash
cargo run
```

# Usage

Usage is pretty simple:

- In `consts.rs` you can adjust window and world resolution as well as some other params

- In `main.rs` you can create "galaxies" using the `create_galaxy()` or `spawn_circle()` functions or by just inserting particles into particles vector

- After program is in run, you can see fps in the window title

- To start rendering record you need to press `R` on your keyboard and then `S` to stop the record. After the recording process is stopped, video will be automatically created from screenshot images and saved into `results` folder in the project root directory

- To move around the canvas you can use mouse scroll wheel to zoom in/out and hover cursor onto the edges of the window to move around

# Algorithm

[Barnes-Hut simulation Wiki](https://en.wikipedia.org/wiki/Barnes%E2%80%93Hut_simulation)

The Barnes-Hut algorithm is a way of optimizing n-body simulations. It allows to group particles into groups if they're enough far away to be considered as a single object. The complexity of this algorithm is O(_n_ log(_n_)) compared to a Direct algorithm which complexity is O(_n_<sup>2</sup>)

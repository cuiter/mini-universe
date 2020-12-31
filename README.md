# Mini Universe

An experiment in simulation theory. This program simulates a virtual universe
with plants and agents (creatures) that have to evolve to stay alive and
produce offspring.

![](https://cuiter.me/ext/mini-universe.png)

Each agent has a set of genes that determines their attributes such as their
color, size and speed. Every agent also has a brain that determines how to
move. Currently, these brains are very simple, only containing two neurons, but
it is still effective.
If an agent goes too long without food, it dies. If it survives long enough,
it will produce offspring with a slightly different brain and genes.

# Dependencies

Mini Universe requires a Rust compiler (preferably version 1.42 or newer) and
the SDL2 and SDL2-image libraries to be installed.

Ubuntu:  
`sudo apt install cargo libsdl2-dev libsdl2-image-dev`

NixOS:  
`nix-shell -p cargo SDL2 SDL2_image`

# How to run

1. Clone this repository
2. `cargo run --release -- [seed]`  
   The seed integer argument is optional. If no seed is given, a random seed
   will be generated.  
   Its purpose is to make simulations reproducible. The same seed
   will always produce the same result.

# Controls

| Button                   | Action                                   |
|--------------------------|------------------------------------------|
| WASD / arrow keys        | Move camera                              |
| Scroll wheel             | Zoom in/out                              |
| Comma (,) / Period (.)   | Slow down / speed up time                |
| Space bar                | Pause / resume simulation               |
| R                        | Restart simulation                       |
| T                        | Time travel (enter time on command line) |

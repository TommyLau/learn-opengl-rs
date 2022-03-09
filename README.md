# learn-opengl-rs [![Build Status](https://github.com/TommyLau/learn-opengl-rs/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/TommyLau/learn-opengl-rs/actions/workflows/ci.yml)

This is a Rust port of https://github.com/JoeyDeVries/LearnOpenGL which is implemented in C++.

The OpenGL tutorials could be found at: https://learnopengl.com/

This repository's code structure has been kept as same as possible to the original C++ source code, and obey the Rust naming conventions at the same time.

Tutorials can be run with the following command (for example: `src/_1_getting_started/_3_6_shaders_exercise3.rs`):

```bash
cargo run 1.3.6
```

> If no argument is given, the program will run the latest tutorial.

## Chapters

### [1. Getting started](src/_1_getting_started)
### [2. Lighting](src/_2_lighting)
### [3. Model Loading](src/_3_model_loading)

## References

- https://learnopengl.com/
- https://learnopengl-cn.github.io/
- https://github.com/JoeyDeVries/LearnOpenGL
- https://github.com/bwasty/learn-opengl-rs

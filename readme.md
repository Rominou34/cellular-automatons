# Cellular Automatons

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust program simulating various [cellular automatons](https://en.wikipedia.org/wiki/Cellular_automaton), built as a learning project in order to get my hands on some of Rust features, and on the rendering engine used, [Macroquad](https://macroquad.rs/).

As it is a learning project, don't expect anything too serious and great code quality following Rust standards : I don't know them yet.

## Features

* **Simulation**
  * [x] Simulate [Conway's Game of Life](https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life)
  * [ ] Simulate [Elementary cellular automatons](https://en.wikipedia.org/wiki/Elementary_cellular_automaton)
  * [ ] Allow multiple states using colors
* **Rendering**
  * [ ] Add a camera system to move around
  * [ ] Add zooming capabilities
  * [ ] Add speed selector : restrict framerate
* **Miscellaneous**
  * [ ] Add the ability to trigger cell states by clicking on it
  * [ ] Add the ability to pause the simulation
  * [ ] Add a main menu to select the algorithm before launching the simulation
  * [ ] Add the ability to launch elementary cellular automatons using their [Wolfram code](https://en.wikipedia.org/wiki/Wolfram_code)
  * [ ] Add a save / load feature
  * [ ] Optimize the code
mod engine;
mod automatons;

use macroquad::prelude::*;
use ::rand::prelude::*;
use std::{thread, time};

use crate::engine::Engine;
use crate::automatons::Automaton;

struct Grid {
    width: u32,
    height: u32,
    cells: Vec<Vec<bool>>
}

impl Grid {
    fn new(width: u32, height: u32) -> Grid {
        let mut cells = Vec::new();
        for y in 1..height + 1 {
            let mut line = Vec::new();
            for x in 1..width + 1 {
                let alive: bool = rand::gen_range(0, 2) == 1;
                line.push(alive);
            }
            cells.push(line);
        }
        //println!("{:?}", grid.cells);

        // Creates a glider (used to test the algorithm)
        // let mut cells = Vec::new();
        // for y in 1..height + 1 {
        //     let mut line = Vec::new();
        //     for x in 1..width + 1 {
        //         let mut alive: bool = false;
        //         if y == 1 && x == 3 {
        //             alive = true;
        //         } else if y == 2 && (x == 1 || x == 3) {
        //             alive = true;
        //         } else if y == 3 && (x == 2 || x == 3) {
        //             alive = true;
        //         }
        //         line.push(alive);
        //     }
        //     cells.push(line);
        // }
        return Grid { width, height, cells: cells.clone() }
    }
}

enum State {
    Menu,
    Running,
    Paused
}

pub struct Simulator {
    state: State,
    automatons: Vec<Box<dyn Automaton>>
}

impl Simulator {
    pub fn new(automatons: Vec<Box<dyn Automaton>>) -> Simulator {
        Simulator { state: State::Menu, automatons}
    }
}

#[macroquad::main("cellular-automatons")]
async fn main() {
    let mut grid = Grid::new(40, 40);

    let automatons = Vec::new();
    let engine = Engine::new(automatons);

    loop {
        render(&grid);

        engine.tick();

        grid.cells = iterate(&grid);

        //let one_tenth = time::Duration::from_millis(100);
        //thread::sleep(one_tenth);
        next_frame().await
    }
}

fn iterate(grid: &Grid) -> Vec<Vec<bool>> {
    let mut buffer = Vec::new();
    let mut y = 0;
    for line in &grid.cells {
        let mut new_line = Vec::new();
        let mut x = 0;
        for cell in line {
            let neighbours = count_neighbours(x, y, grid);
            let mut new_cell: bool = false;
            if *cell && (neighbours == 2 || neighbours == 3) {
                new_cell = true
            } else if !*cell && neighbours == 3 {
                new_cell = true
            }
            new_line.push(new_cell);
            x += 1;
        }
        buffer.push(new_line);
        y += 1;
    }
    return buffer;
}

fn count_neighbours(x: u32, y: u32, grid: &Grid) -> u8 {
    let min_x = if x >= 1 { x - 1 } else { x };
    let max_x = if x <= grid.width - 2 { x + 1 } else { x };
    let min_y = if y >= 1 { y - 1 } else { y };
    let max_y = if y <= grid.height - 2 { y + 1 } else { y };

    let mut neighbours: u8 = 0;
    for this_y in min_y..max_y + 1 {
        for this_x in min_x..max_x + 1 {
            if (this_y != y || this_x != x) && grid.cells[this_y as usize][this_x as usize] {
                neighbours += 1;
            }
        }
    }

    return neighbours;
}

fn render(grid: &Grid) {
    let mut y = 0.;
    let cell_size = 20.;
    for line in &grid.cells {
        let mut x = 0.;
        for cell in line {
            let color: Color = if *cell { WHITE } else { BLACK };
            draw_rectangle(x, y, cell_size, cell_size, color);
            x += cell_size + 2.;
        }
        y += cell_size + 2.;
    }
}
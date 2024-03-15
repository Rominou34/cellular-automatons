/// Module containing the automatons. Each automaton is a struct using the
/// Automaton trait and declaring these functions: new(), iterate(), render()

use macroquad::prelude::*;

enum AutomatonType {
    Elementary,
    Grid
}

// pub struct Automaton {
//     name: String,
//     automaton_type: AutomatonType
// }

pub trait Automaton {
    fn iterate(&mut self);
    fn render(&self);
}

pub struct GameOfLife {
    width: u32,
    height: u32,
    cells: Vec<Vec<bool>>
}

impl GameOfLife {
    pub fn new(width: u32, height: u32) -> GameOfLife {
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
        return GameOfLife { width, height, cells: cells.clone() }
    }

    fn count_neighbours(&self, x: u32, y: u32) -> u8 {
        let min_x = if x >= 1 { x - 1 } else { x };
        let max_x = if x <= self.width - 2 { x + 1 } else { x };
        let min_y = if y >= 1 { y - 1 } else { y };
        let max_y = if y <= self.height - 2 { y + 1 } else { y };

        let mut neighbours: u8 = 0;
        for this_y in min_y..max_y + 1 {
            for this_x in min_x..max_x + 1 {
                if (this_y != y || this_x != x) && self.cells[this_y as usize][this_x as usize] {
                    neighbours += 1;
                }
            }
        }

        return neighbours;
    }
}

impl Automaton for GameOfLife {
    fn iterate(&mut self) {
        let mut buffer = Vec::new();
        let mut y = 0;
        for line in &self.cells {
            let mut new_line = Vec::new();
            let mut x = 0;
            for cell in line {
                let neighbours = self.count_neighbours(x, y);
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
        self.cells = buffer;
    }

    fn render(&self) {
        let mut y = 0.;
        let cell_size = 20.;
        for line in &self.cells {
            let mut x = 0.;
            for cell in line {
                let color: Color = if *cell { WHITE } else { BLACK };
                draw_rectangle(x, y, cell_size, cell_size, color);
                x += cell_size + 2.;
            }
            y += cell_size + 2.;
        }
    }
}
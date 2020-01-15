extern crate wasm_bindgen;
extern crate rand;

use rand::Rng;
use wasm_bindgen::prelude::*;
use crate::universe::Universe;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Void = 0,
    Sand = 1,
    Water = 2
}

impl Cell {
    pub fn simulate(&self, x: u32, y: u32, cells: &mut Vec<Cell>, universe: &Universe) {

        match *self {
            Cell::Sand => self.simulate_sand(x, y, cells, universe),
            Cell::Void => self.simulate_void(x, y, cells, universe),
            Cell::Water => self.simulate_water(x, y, cells, universe),
        }
    }

    pub fn simulate_void(self, x: u32, y: u32, cells: &mut Vec<Cell>, universe: &Universe) {

    }

    pub fn simulate_water(self, x: u32, y: u32, cells: &mut Vec<Cell>, universe: &Universe) {

        let height = universe.height();
        let width = universe.width();

        let down = y + 1;

        if down < height {

            let index = universe.get_index(x, y);
            let current_cell = universe.get_cell(x, y);

            let down_cell = universe.get_cell(x, down);

            if down_cell == Cell::Void {

                let down_index = universe.get_index(x, down);

                cells[index] = Cell::Void;
                cells[down_index] = Cell::Water;

            } else {

                let mut adjacent = if rand::random() {
                    x - 1
                } else {
                    x + 1
                };

                if adjacent < width && adjacent > 0 {

                    let diagonal_cell = universe.get_cell(adjacent, down);

                    if diagonal_cell == Cell::Void {

                        let diagonal_index = universe.get_index(adjacent, down);

                        cells[index] = Cell::Void;
                        cells[diagonal_index] = Cell::Water;

                    } else {

                        let adjacent_cell = universe.get_cell(adjacent, y);

                        if adjacent_cell == Cell::Void {

                            let adjacent_index = universe.get_index(adjacent, y);
                            cells[index] = Cell::Void;
                            cells[adjacent_index] = Cell::Water;
                        }
                    }
                }
            }
        }
    }

    pub fn simulate_sand(self, x: u32, y: u32, cells: &mut Vec<Cell>, universe: &Universe) {

        let height = universe.height();
        let width = universe.width();

        let down = y + 1;

        if down < height {

            let index = universe.get_index(x, y);
            let current_cell = universe.get_cell(x, y);

            let down_cell = universe.get_cell(x, down);

            if down_cell == Cell::Void {

                let down_index = universe.get_index(x, down);

                cells[index] = Cell::Void;
                cells[down_index] = Cell::Sand;

            } else {

                let mut adjacent = if rand::random() {
                    x - 1
                } else {
                    x + 1
                };

                if adjacent < universe.width() && adjacent > 0 {

                    let adjacent_cell = universe.get_cell(adjacent, down);

                    if adjacent_cell == Cell::Void {

                        let adjacent_index = universe.get_index(adjacent, down);

                        cells[index] = Cell::Void;
                        cells[adjacent_index] = Cell::Sand;
                    }
                }
            }
        }
    }
}
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::cell::Cell;
use crate::material::Material;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
    mats: Vec<Material>
}

#[wasm_bindgen]
impl Universe {

    pub fn new(width: u32, height: u32) -> Universe {

        let cells = vec![Cell::Void; (width * height) as usize];

        let mat = Material {
            cell: Cell::Void,
            used: false
        };

        let mats = vec![mat; (width * height) as usize];

        Universe {
            width,
            height,
            cells,
            mats
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn set_cell(&mut self, x: u32, y: u32, cell: Cell) {

        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    pub fn get_cell(&self, x: u32, y: u32) -> Cell {
        let index = self.get_index(x, y);
        self.cells[index]
    }

    pub fn get_index(&self, x: u32, y: u32) -> usize {
        (y * self.width + x) as usize
    }

    pub fn tick(&mut self) {

        let mut next = self.cells.clone();

        for x in 0..self.width {
            for y in 0..self.height {

                self.get_cell(x, y).simulate(x, y, &mut next, &self);
            }
        }

        self.cells = next;
    }
}
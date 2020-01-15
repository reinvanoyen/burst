extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use crate::cell::Cell;

#[wasm_bindgen]
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Material {
    pub cell: Cell,
    pub used: bool,
}
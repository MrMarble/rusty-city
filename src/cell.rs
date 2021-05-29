use crate::{species::Species, universe::Universe};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub specie: Species,
    pub x: i32,
    pub y: i32,
    pub clock: i32,
}

impl Cell {
    pub fn new(specie: Species, x: i32, y: i32) -> Cell {
        Cell {
            specie,
            x,
            y,
            clock: 0,
        }
    }
    pub fn update(&self, universe: &mut Universe) {
        self.specie.update(*self, universe);
    }
}

pub static EMPTY_CELL: Cell = Cell {
    specie: Species::Empty,
    clock: 0,
    x: 0,
    y: 0,
};

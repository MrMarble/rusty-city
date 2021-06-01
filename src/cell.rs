use crate::{species::Species, universe::Universe};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    specie: Species,
    clock: i32,
}

impl Cell {
    pub fn new(specie: Species, clock: i32) -> Cell {
        Cell { specie, clock }
    }

    pub fn update(&self, x: i32, y: i32, universe: &mut Universe) {
        self.specie.update(x, y, universe)
    }

    pub fn clock(&self) -> i32 {
        self.clock
    }

    pub fn specie(&self) -> Species {
        self.specie
    }
}

pub static EMPTY_CELL: Cell = Cell {
    specie: Species::Empty,
    clock: 0,
};

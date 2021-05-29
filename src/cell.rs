use crate::{species::Species, universe::Universe};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    specie: Species,
    x: i32,
    y: i32,
    clock: i32,
}

impl Cell {
    pub fn new(specie: Species, x: i32, y: i32, clock: i32) -> Cell {
        Cell {
            specie,
            x,
            y,
            clock,
        }
    }

    pub fn update(&self, universe: &mut Universe) {
        self.specie.update(*self, universe);
    }

    pub fn clock(&self) -> i32 {
        self.clock
    }

    pub fn coords(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn specie(&self) -> Species {
        self.specie
    }
}

pub static EMPTY_CELL: Cell = Cell {
    specie: Species::Empty,
    clock: 0,
    x: 0,
    y: 0,
};

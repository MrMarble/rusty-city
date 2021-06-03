use macroquad::prelude::{const_vec2, Vec2};

use crate::{species::Species, universe::Universe};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub velocity: Vec2,
    specie: Species,
}

impl Cell {
    pub fn new(specie: Species, velocity: Vec2) -> Cell {
        Cell { velocity, specie }
    }

    pub fn update(&self, x: i32, y: i32, universe: &mut Universe) {
        self.specie.update(x, y, universe)
    }

    pub fn specie(&self) -> Species {
        self.specie
    }
}

pub static EMPTY_CELL: Cell = Cell {
    specie: Species::Empty,
    velocity: const_vec2!([0., 0.]),
};

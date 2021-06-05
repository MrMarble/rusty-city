use macroquad::prelude::{const_vec2, Vec2};

use crate::{species::Species, universe::Universe};

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    specie: Species,
    pub velocity: Vec2,
    clock: i32,
}

impl Cell {
    pub fn new(specie: Species, clock: i32, velocity: Vec2) -> Cell {
        Cell {
            specie,
            velocity,
            clock,
        }
    }
    pub fn from(c: Cell) -> Self {
        Cell {
            specie: c.specie,
            clock: c.clock + 1,
            velocity: c.velocity,
        }
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
    velocity: const_vec2!([0., 0.]),
};

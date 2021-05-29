use crate::{
    cell::{Cell, EMPTY_CELL},
    universe::Universe,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum Species {
    Empty = 0xFFFFFF,
    Wall = 0x000000,
    Sand = 0xC2B280,
}

impl Species {
    pub fn update(&self, cell: Cell, universe: &mut Universe) {
        match self {
            Self::Sand => update_sand(cell, universe),
            _ => {}
        }
    }
}

fn update_sand(cell: Cell, universe: &mut Universe) {
    if cell.clock() - universe.generation() == 1 {
        return;
    }
    let (x, y) = cell.coords();

    let nbr = universe.get_cell(x, y + 1);

    if nbr.specie() == Species::Empty {
        universe.set(x, y, EMPTY_CELL);
        universe.set(
            x,
            y + 1,
            Cell::new(cell.specie(), x, y + 1, cell.clock() + 1),
        );
    } else if universe.get_cell(x + 1, y + 1).specie() == Species::Empty {
        universe.set(x, y, EMPTY_CELL);
        universe.set(
            x + 1,
            y + 1,
            Cell::new(cell.specie(), x + 1, y + 1, cell.clock() + 1),
        );
    } else if universe.get_cell(x - 1, y + 1).specie() == Species::Empty {
        universe.set(x, y, EMPTY_CELL);
        universe.set(
            x - 1,
            y + 1,
            Cell::new(cell.specie(), x - 1, y + 1, cell.clock() + 1),
        );
    }
}

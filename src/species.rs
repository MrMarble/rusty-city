use crate::{cell::Cell, universe::Universe, utils::rand_dir_2};

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
    let dx = rand_dir_2();

    let nbr = universe.get_cell(x, y + 1);

    if nbr.specie() == Species::Empty {
        universe.update_cell(0, 1, cell);
    } else if universe.get_cell(x + dx, y + 1).specie() == Species::Empty {
        universe.update_cell(dx, 1, cell);
    } else if universe.get_cell(x + dx * -1, y + 1).specie() == Species::Empty {
        universe.update_cell(dx * -1, 1, cell);
    }
}

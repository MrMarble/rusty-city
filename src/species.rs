use crate::{
    cell::Cell,
    universe::Universe,
    utils::{rand_dir, rand_dir_2},
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum Species {
    Empty = 0xFFFFFF,
    Wall = 0x000000,
    Sand = 0xC2B280,
    Water = 0x1B7CED,
}

impl Species {
    pub fn update(&self, cell: Cell, universe: &mut Universe) {
        match self {
            Self::Sand => update_sand(cell, universe),
            Self::Water => update_water(cell, universe),
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
    } else if nbr.specie() == Species::Water {
        universe.replace_cell(cell, nbr);
    }
}

fn update_water(cell: Cell, universe: &mut Universe) {
    if cell.clock() - universe.generation() == 1 {
        return;
    }

    let (x, y) = cell.coords();
    let dx = rand_dir();

    let below = universe.get_cell(x, y + 1);
    let dx1 = universe.get_cell(x + dx, y + 1);
    let dx0 = universe.get_cell(x + dx, y);

    // below
    if below.specie() == Species::Empty {
        universe.update_cell(0, 1, cell);
    // below left/right
    } else if dx1.specie() == Species::Empty {
        universe.update_cell(dx, 1, cell);
    // below left/right
    } else if universe.get_cell(x + -dx, y + 1).specie() == Species::Empty {
        universe.update_cell(-dx, 1, cell);
    // left/right
    } else if dx0.specie() == Species::Empty {
        universe.update_cell(dx, 0, cell);
    // left/right
    } else if universe.get_cell(x + -dx, y).specie() == Species::Empty {
        universe.update_cell(-dx, 0, cell);
    }
}

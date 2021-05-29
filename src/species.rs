use crate::{
    cell::{Cell, EMPTY_CELL},
    universe::Universe,
};

#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum Species {
    Empty = 0xFFFFFF,
    Sand = 0xC2B280,
    Wall = 0x000000,
}

impl Species {
    pub fn update(&self, cell: Cell, universe: &mut Universe) {
        match self {
            Self::Sand => {
                if cell.clock - universe.generation == 1 {
                    return;
                }
                let nbr = universe.get_cell(cell.x, cell.y + 1);

                if nbr.specie == Species::Empty {
                    universe.set(cell.x, cell.y, EMPTY_CELL);
                    universe.set(
                        cell.x,
                        cell.y + 1,
                        Cell {
                            specie: cell.specie,
                            x: cell.x,
                            y: cell.y + 1,
                            clock: cell.clock + 1,
                        },
                    );
                } else if universe.get_cell(cell.x + 1, cell.y + 1).specie == Species::Empty {
                    universe.set(cell.x, cell.y, EMPTY_CELL);
                    universe.set(
                        cell.x + 1,
                        cell.y + 1,
                        Cell {
                            specie: cell.specie,
                            x: cell.x + 1,
                            y: cell.y + 1,
                            clock: cell.clock + 1,
                        },
                    );
                } else if universe.get_cell(cell.x - 1, cell.y + 1).specie == Species::Empty {
                    universe.set(cell.x, cell.y, EMPTY_CELL);
                    universe.set(
                        cell.x - 1,
                        cell.y + 1,
                        Cell {
                            specie: cell.specie,
                            x: cell.x - 1,
                            y: cell.y + 1,
                            clock: cell.clock + 1,
                        },
                    );
                }
            }
            _ => {}
        }
    }
}

use macroquad::prelude::vec2;

use crate::{
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
    pub fn update(&self, x: i32, y: i32, universe: &mut Universe) {
        match self {
            Self::Sand => update_sand(x, y, universe),
            Self::Water => update_water(x, y, universe),
            _ => {}
        }
    }
}

fn update_sand(x: i32, y: i32, universe: &mut Universe) {
    let cell = universe.get_cell(x, y);
    if cell.clock() - universe.generation() == 1 {
        return;
    }

    let dx = rand_dir_2();

    let nbr = universe.get_cell(x, y + 1);

    if nbr.specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(0., 1.), cell);
    } else if universe.get_cell(x + dx, y + 1).specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(dx as f32, 1.), cell);
    } else if nbr.specie() == Species::Water {
        universe.replace_cell(vec2(x as f32, y as f32), vec2(x as f32, (y + 1) as f32));
    }
}

fn update_water(x: i32, y: i32, universe: &mut Universe) {
    let cell = universe.get_cell(x, y);
    if cell.clock() - universe.generation() == 1 {
        return;
    }

    let dx = rand_dir();

    let below = universe.get_cell(x, y + 1);
    let dx1 = universe.get_cell(x + dx, y + 1);
    let dx0 = universe.get_cell(x + dx, y);

    // below
    if below.specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(0., 1.), cell);
    // below left/right
    } else if dx1.specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(dx as f32, 1.), cell);
    // below left/right
    } else if universe.get_cell(x + -dx, y + 1).specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(-dx as f32, 1.), cell);
    // left/right
    } else if dx0.specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(dx as f32, 0.), cell);
    // left/right
    } else if universe.get_cell(x + -dx, y).specie() == Species::Empty {
        universe.update_cell(vec2(x as f32, y as f32), vec2(-dx as f32, 0.), cell);
    }
}

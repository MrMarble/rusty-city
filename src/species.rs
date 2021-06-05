use macroquad::{
    prelude::{get_frame_time, vec2},
    rand::gen_range,
};

use crate::{
    cell::{Cell, EMPTY_CELL},
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
    let mut cell = universe.get_cell(x, y);
    if cell.clock() - universe.generation() == 1 {
        return;
    }

    let dx = rand_dir_2();
    let nbr = universe.get_cell(x, y + 1);

    // update velocity
    let time_delta = get_frame_time();
    cell.velocity.y = (cell.velocity.y + universe.gravity() * time_delta)
        .clamp(-universe.gravity(), universe.gravity());

    if universe.in_bounds(x, y + 1)
        && !universe.is_empty(x, y + 1)
        && nbr.specie() != Species::Water
    {
        cell.velocity.y /= 2.;
    }

    let vi_x = x + cell.velocity.x as i32;
    let vi_y = y + cell.velocity.y as i32;

    // Physics using velocity
    if universe.in_bounds(vi_x, vi_y)
        && (universe.is_empty(vi_x, vi_y)
            || (universe.get_cell(vi_x, vi_y).specie() == Species::Water
                && universe.get_cell(vi_x, vi_y).clock() - universe.generation() != 1
                && universe.get_cell(vi_x, vi_y).velocity.length() - cell.velocity.length()
                    > universe.gravity()))
    {
        let mut dest_cell = universe.get_cell(vi_x, vi_y);

        if dest_cell.specie() == Species::Water {
            let rand_x = gen_range(-2, 2);
            dest_cell.velocity = vec2(rand_x as f32, -4.);

            universe.set(vi_x, vi_y, Cell::from(cell));

            for y in -10..0 {
                for x in -10..10 {
                    if universe.is_empty(vi_x + x, vi_y + y) {
                        universe.set(vi_x + x, vi_y + y, Cell::from(dest_cell));
                        break;
                    }
                }
            }
            // Just in case
            universe.set(x, y, EMPTY_CELL);
        } else if dest_cell.specie() == Species::Empty {
            universe.set(vi_x, vi_y, Cell::from(cell));
            universe.set(x, y, Cell::from(dest_cell));
        }
    } else if nbr.specie() == Species::Empty {
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

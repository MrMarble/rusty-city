use macroquad::{
    prelude::{get_frame_time, vec2},
    rand::gen_range,
};

use crate::{universe::Universe, utils::rand_dir_2};

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
                && universe.get_cell(vi_x, vi_y).velocity.length() - cell.velocity.length()
                    > universe.gravity()))
    {
        let mut dest_cell = universe.get_cell(vi_x, vi_y);

        if dest_cell.specie() == Species::Water {
            let rand_x = gen_range(-2, 2);
            dest_cell.velocity = vec2(rand_x as f32, -4.);

            universe.move_cell(x, y, vi_x, vi_y);
            for y in -10..0 {
                for x in -10..10 {
                    if universe.is_empty(vi_x + x, vi_y + y) {
                        universe.set(vi_x, vi_y, dest_cell);
                        universe.move_cell(vi_x, vi_y, vi_x + x, vi_y + y);
                        break;
                    }
                }
            }
        } else if dest_cell.specie() == Species::Empty {
            universe.set(x, y, cell);
            universe.move_cell(x, y, vi_x, vi_y);
        }
    } else if nbr.specie() == Species::Empty {
        cell.velocity.y += universe.gravity() * time_delta;
        universe.set(x, y, cell);

        universe.move_cell(x, y, x, y + 1);
    } else if universe.get_cell(x + dx, y + 1).specie() == Species::Empty
        || nbr.specie() == Species::Water
    {
        cell.velocity.y += universe.gravity() * time_delta;

        universe.set(x, y, cell);

        universe.move_cell(x, y, x + dx, y + 1);
    }
}

fn update_water(x: i32, y: i32, universe: &mut Universe) {
    let mut cell = universe.get_cell(x, y);

    let fall_rate = 2;
    let spread_rate = 5;
    let rand_spread = if rand_dir_2() == 1 {
        spread_rate
    } else {
        -spread_rate
    };

    // update velocity
    let time_delta = get_frame_time();
    cell.velocity.y = (cell.velocity.y + universe.gravity() * time_delta)
        .clamp(-universe.gravity(), universe.gravity());

    if universe.in_bounds(x, y + 1) && !universe.is_empty(x, y + 1) {
        cell.velocity.y /= 2.;
    }

    let vi_x = x + cell.velocity.x as i32;
    let vi_y = y + cell.velocity.y as i32;

    // Physics using velocity
    universe.set(x, y, cell);
    if universe.in_bounds(vi_x, vi_y) && universe.is_empty(vi_x, vi_y) {
        universe.move_cell(x, y, vi_x, vi_y);
    } else if universe.is_empty(x, y + fall_rate) {
        universe.move_cell(x, y, x, y + fall_rate);
    } else if universe.is_empty(x + rand_spread, y + fall_rate) {
        universe.move_cell(x, y, x + rand_spread, y + fall_rate);
    } else if universe.is_empty(x + -rand_spread, y + fall_rate) {
        universe.move_cell(x, y, x + -rand_spread, y + fall_rate);
    } else if universe.in_bounds(x, y + fall_rate) && universe.is_empty(x, y + fall_rate) {
        universe.move_cell(x, y, x, y + fall_rate);
    } else if universe.in_bounds(x + -rand_spread, y + fall_rate)
        && universe.is_empty(x + -rand_spread, y + fall_rate)
    {
        universe.move_cell(x, y, x + -rand_spread, y + fall_rate);
    } else if universe.in_bounds(x + rand_spread, y + fall_rate)
        && universe.is_empty(x + rand_spread, y + fall_rate)
    {
        universe.move_cell(x, y, x + rand_spread, y + fall_rate);
    } else {
        for i_y in 0..fall_rate {
            for i_x in (0..spread_rate).rev() {
                if universe.in_bounds(x - i_x, y + i_y) && universe.is_empty(x - i_x, y + i_y) {
                    universe.move_cell(x, y, x - i_x, y + i_y);
                    break;
                }
                if universe.in_bounds(x + i_x, y + i_y) && universe.is_empty(x + i_x, y + i_y) {
                    universe.move_cell(x, y, x + i_x, y + i_y);
                    break;
                }
            }
        }
    }
}

use macroquad::{
    prelude::{draw_rectangle, vec2, Color, Vec2},
    rand::gen_range,
};

use crate::{
    cell::{Cell, EMPTY_CELL},
    species::Species,
    utils::value_of,
};

pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    changes: Vec<(usize, usize)>,
    generation: i32,
    scale: f32,
    gravity: f32,
    pub non_empty_cells: u32,
}

impl Universe {
    pub fn new(width: f32, height: f32, scale: f32) -> Universe {
        let s_width = (width / scale) as i32;
        let s_height = (height / scale) as i32;
        let cells = (0..s_width * s_height).map(|_| EMPTY_CELL).collect();

        Universe {
            width: s_width,
            height: s_height,
            cells,
            changes: vec![],
            scale,
            generation: 0,
            gravity: 10.,
            non_empty_cells: 0,
        }
    }

    pub fn move_cell(&mut self, x: i32, y: i32, xto: i32, yto: i32) {
        self.changes
            .push((self.get_index(xto, yto), self.get_index(x, y)));
    }

    pub fn commit_changes(&mut self) {
        if self.changes.is_empty() {
            return;
        }

        let mut i = 0;
        while i < self.changes.len() - 1 {
            let specie = self.cells[self.changes[i as usize].0].specie();
            if specie != Species::Empty && specie != Species::Water {
                self.changes[i as usize] = self.changes.pop().unwrap();
            } else {
                i += 1;
            }
        }

        self.changes.sort_by(|a, b| a.0.cmp(&b.0));

        let mut i_prev = 0;
        self.changes.push((0, 0));
        for i in 0..self.changes.len() - 1 {
            if self.changes[i + 1].0 != self.changes[i].0 {
                let r = i_prev + gen_range(0, i - i_prev);
                let dst = self.changes[r].0;
                let src = self.changes[r].1;
                self.cells.swap(dst, src);
                i_prev = i + 1;
            }
        }
        self.changes.clear();
    }

    pub fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                cell.update(x, y, self);
            }
        }
        self.generation += 1;
        self.commit_changes();
    }

    pub fn render(&self) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.specie() != Species::Empty {
                let (r, g, b) = value_of(cell.specie() as u32);
                let pos = self.get_position(i as i32);
                draw_rectangle(
                    pos.x * self.scale,
                    pos.y * self.scale,
                    self.scale,
                    self.scale,
                    Color::new(r, g, b, 1.),
                );
            }
        }
    }

    pub fn paint(&mut self, x: f32, y: f32, size: f32, mat: Species) {
        let radius = size / 2.0 / self.scale;

        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        for dx in -floor..ciel {
            for dy in -floor..ciel {
                if ((dx * dx) + (dy * dy)) as f32 > (radius * radius) {
                    continue;
                };
                let px = x as i32 + dx;
                let py = y as i32 + dy;

                let i = self.get_index(px, py);
                let current_specie = self.get_cell(px, py).specie();

                if px < 0
                    || px > self.width - 1
                    || py < 0
                    || py > self.height - 1
                    || mat == current_specie
                {
                    continue;
                }

                if current_specie == Species::Empty || mat == Species::Empty {
                    self.cells[i] = Cell::new(mat, vec2(gen_range(-1., 1.), gen_range(-2., 5.)))
                }

                if mat == Species::Empty {
                    self.non_empty_cells -= 1;
                } else if !(current_specie != Species::Empty) {
                    self.non_empty_cells += 1;
                }
            }
        }
    }

    pub fn generation(&self) -> i32 {
        self.generation
    }

    pub fn gravity(&self) -> f32 {
        self.gravity
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        if x >= self.width || x < 0 || y >= self.height || y < 0 {
            return Cell::new(Species::Wall, vec2(0., 0.));
        }
        let i = self.get_index(x, y);
        self.cells[i]
    }

    pub fn get_position(&self, index: i32) -> Vec2 {
        let x = index % self.width;
        let y = index / self.width;
        vec2(x as f32, y as f32)
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x + self.width * y) as usize
    }

    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x < self.width && y < self.height
    }
    pub fn is_empty(&self, x: i32, y: i32) -> bool {
        self.in_bounds(x, y) && self.get_cell(x, y).specie() == Species::Empty
    }
}

mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn new_universe() {
        let u = Universe::new(433., 367., 5.);
        for i in 0..u.cells.len() {
            let xy = u.get_position(i as i32);

            let x = i % u.width as usize;
            let y = i / u.width as usize;

            assert_eq!((xy.x as usize, xy.y as usize), (x, y));
            assert_eq!(
                u.get_index(xy.x as i32, xy.y as i32),
                u.get_index(x as i32, y as i32),
                "Testing get_position"
            );
            assert_eq!(
                u.get_index(xy.x as i32, xy.y as i32),
                i,
                "Testing get_index"
            );
        }
    }
}

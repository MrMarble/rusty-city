use macroquad::prelude::{draw_rectangle, Color};

use crate::{
    cell::{Cell, EMPTY_CELL},
    species::Species,
    utils::value_of,
};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};

pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    generation: i32,
}

impl Universe {
    pub fn new(width: i32, height: i32) -> Universe {
        let cells = (0..width * height).map(|_| EMPTY_CELL).collect();

        Universe {
            width,
            height,
            cells,
            generation: 0,
        }
    }

    pub fn tick(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let cell = self.get_cell(x, y);
                cell.update(self);
            }
        }
        self.generation += 1
    }

    pub fn render(&self) {
        let (tx, rx): (Sender<Vec<Cell>>, Receiver<Vec<Cell>>) = channel();
        let t = thread::spawn(move || {
            let pixels = rx.recv().unwrap();
            for pixel in pixels {
                if pixel.specie() != Species::Empty {
                    let (r, g, b) = value_of(pixel.specie() as u32);
                    let (x, y) = pixel.coords();
                    draw_rectangle(x as f32, y as f32, 1., 1., Color::new(r, g, b, 1.));
                }
            }
        });
        tx.send(self.cells.clone()).unwrap();
        t.join().unwrap();
    }

    pub fn paint(&mut self, x: i32, y: i32, size: f64) {
        let radius: f64 = size / 2.0;

        let floor = (radius + 1.0) as i32;
        let ciel = (radius + 1.5) as i32;

        for dx in -floor..ciel {
            for dy in -floor..ciel {
                if (((dx * dx) + (dy * dy)) as f64) > (radius * radius) {
                    continue;
                };
                let px = x + dx;
                let py = y + dy;

                let i = self.get_index(px, py);

                if px < 0 || px > self.width - 1 || py < 0 || py > self.height - 1 {
                    continue;
                }
                if self.get_cell(px, py).specie() == Species::Empty {
                    self.cells[i] = Cell::new(Species::Sand, px, py, self.generation)
                }
            }
        }
    }

    pub fn generation(&self) -> i32 {
        self.generation
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    pub fn get_cell(&self, x: i32, y: i32) -> Cell {
        if x >= self.width || x < 0 || y >= self.height || y < 0 {
            return Cell::new(Species::Wall, x, y, 0);
        }
        let i = self.get_index(x, y);
        self.cells[i]
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }
}
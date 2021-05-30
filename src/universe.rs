use macroquad::prelude::{draw_rectangle, Color};

use crate::{
    cell::{Cell, EMPTY_CELL},
    species::Species,
    utils::value_of,
};

#[cfg(not(target_arch = "wasm32"))]
const NTHREADS: usize = 4;

pub struct Universe {
    width: i32,
    height: i32,
    cells: Vec<Cell>,
    generation: i32,
    pub non_empty_cells: u32,
}

impl Universe {
    pub fn new(width: i32, height: i32) -> Universe {
        let cells = (0..width * height).map(|_| EMPTY_CELL).collect();

        Universe {
            width,
            height,
            cells,
            generation: 0,
            non_empty_cells: 0,
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn render(&self) {
        let num_taks_per_thread = self.cells.len() / NTHREADS;

        crossbeam::scope(|scope| {
            let threads: Vec<_> = self
                .cells
                .chunks(num_taks_per_thread)
                .map(|chunk| {
                    scope.spawn(move |_| {
                        chunk.iter().cloned().map(|c| {
                            if c.specie() != Species::Empty {
                                let (r, g, b) = value_of(c.specie() as u32);
                                let (x, y) = c.coords();
                                return (x as f32, y as f32, Color::new(r, g, b, 1.));
                            }
                            (
                                0.,
                                0.,
                                Color {
                                    ..Default::default()
                                },
                            )
                        })
                    })
                })
                .collect();
            threads.into_iter().for_each(|t| {
                t.join().unwrap().for_each(|p| {
                    if p.0 != 0. {
                        draw_rectangle(p.0, p.1, 1., 1., p.2)
                    }
                })
            });
        })
        .unwrap();
    }

    #[cfg(target_arch = "wasm32")]
    pub fn render(&self) {
        for pixel in &self.cells {
            if pixel.specie() != Species::Empty {
                let (r, g, b) = value_of(pixel.specie() as u32);
                let (x, y) = pixel.coords();
                draw_rectangle(x as f32, y as f32, 1., 1., Color::new(r, g, b, 1.));
            }
        }
    }

    pub fn paint(&mut self, x: i32, y: i32, size: f64, mat: Species) {
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
                if mat == Species::Empty || self.get_cell(px, py).specie() == Species::Empty {
                    self.cells[i] = Cell::new(mat, px, py, self.generation)
                }

                if mat == Species::Empty {
                    self.non_empty_cells -= 1;
                } else {
                    self.non_empty_cells += 1;
                }
            }
        }
    }

    pub fn generation(&self) -> i32 {
        self.generation
    }

    pub fn replace_cell(&mut self, a: Cell, b: Cell) {
        let (a_x, a_y) = a.coords();
        let (b_x, b_y) = b.coords();

        self.set(a_x, a_y, Cell::new(b.specie(), a_x, a_y, b.clock() + 1));
        self.set(b_x, b_y, Cell::new(a.specie(), b_x, b_y, a.clock() + 1));
    }

    pub fn update_cell(&mut self, dx: i32, dy: i32, cell: Cell) {
        let (x, y) = cell.coords();
        self.set(x, y, EMPTY_CELL);
        self.set(
            x + dx,
            y + dy,
            Cell::new(cell.specie(), x + dx, y + dy, cell.clock() + 1),
        );
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

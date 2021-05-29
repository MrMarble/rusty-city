use macroquad::prelude::*;
use rgb::{ComponentBytes, RGB8};
use std::{
    sync::mpsc::{channel, Receiver, Sender},
    thread,
};
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
enum Specie {
    Empty = 0xFFFFFF,
    Sand = 0xC2B280,
    Wall = 0x000000,
}
#[derive(Clone, Copy, Debug)]
struct Cell {
    specie: Specie,
    x: i32,
    y: i32,
    clock: i32,
}
impl Specie {
    pub fn update(&self, cell: Cell, universe: &mut Universe) {
        match self {
            Self::Sand => {
                if cell.clock - universe.generation == 1 {
                    return;
                }
                let nbr = universe.get_cell(cell.x, cell.y + 1);

                if nbr.specie == Specie::Empty {
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
                } else if universe.get_cell(cell.x + 1, cell.y + 1).specie == Specie::Empty {
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
                } else if universe.get_cell(cell.x - 1, cell.y + 1).specie == Specie::Empty {
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
impl Cell {
    pub fn new(specie: Specie, x: i32, y: i32) -> Cell {
        Cell {
            specie,
            x,
            y,
            clock: 0,
        }
    }
    pub fn update(&self, universe: &mut Universe) {
        self.specie.update(*self, universe);
    }
}

static EMPTY_CELL: Cell = Cell {
    specie: Specie::Empty,
    clock: 0,
    x: 0,
    y: 0,
};

struct Universe {
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
        //let mut pixels: Vec<RGB8> = vec![];
        let (tx, rx): (Sender<Vec<Cell>>, Receiver<Vec<Cell>>) = channel();
        let t = thread::spawn(move || {
            let pixels = rx.recv().unwrap();
            for pixel in pixels {
                //pixels.push(value_of(pixel.specie as u32));
                if pixel.specie != Specie::Empty {
                    let (r, g, b) = value_of(pixel.specie as u32);
                    draw_rectangle(
                        pixel.x as f32,
                        pixel.y as f32,
                        1.,
                        1.,
                        Color::new(r, g, b, 1.),
                    );
                }
            }
        });
        tx.send(self.cells.clone()).unwrap();
        t.join().unwrap();
        //let texture = Texture2D::from_rgba8(600, 600, &pixels.as_bytes());
        //draw_texture(texture, 0., 0., WHITE);
    }

    pub fn paint(&mut self, x: i32, y: i32) {
        let size = 30;
        let radius: f64 = (size as f64) / 2.0;

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
                if self.get_cell(px, py).specie == Specie::Empty {
                    self.cells[i] = Cell {
                        specie: Specie::Sand,
                        x: px,
                        y: py,
                        clock: self.generation,
                    }
                }
            }
        }
    }

    pub fn set(&mut self, x: i32, y: i32, cell: Cell) {
        let index = self.get_index(x, y);
        self.cells[index] = cell;
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (x * self.height + y) as usize
    }
    fn get_cell(&self, x: i32, y: i32) -> Cell {
        if x >= self.width || x < 0 || y >= self.height || y < 0 {
            return Cell {
                specie: Specie::Wall,
                x,
                y,
                clock: 0,
            };
        }
        let i = self.get_index(x, y);
        self.cells[i]
    }
}
fn value_of(color: u32) -> (f32, f32, f32) {
    let r: f32 = ((color >> 16) & 0xFF) as f32 / 255. as f32;
    let g: f32 = ((color >> 8) & 0xFF) as f32 / 255. as f32;
    let b: f32 = ((color) & 0xFF) as f32 / 255. as f32;
    (r, g, b)
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Rusty City".to_owned(),
        window_height: 600,
        window_width: 600,
        window_resizable: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut u = Universe::new(screen_width() as i32, screen_height() as i32);
    loop {
        clear_background(WHITE);

        u.tick();
        u.render();

        draw_text(
            &format!("{}", macroquad::time::get_fps()),
            20.0,
            20.0,
            30.0,
            DARKGRAY,
        );
        draw_text(
            &format!("{}x{}", screen_width(), screen_height()),
            20.0,
            50.0,
            20.0,
            DARKGRAY,
        );
        if is_mouse_button_down(MouseButton::Left) {
            let pos = mouse_position();
            &u.paint(pos.0 as i32, pos.1 as i32);
        }
        if is_key_pressed(KeyCode::Enter) {
            u = Universe::new(screen_width() as i32, screen_height() as i32);
        }

        next_frame().await
    }
}

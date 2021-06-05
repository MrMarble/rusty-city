use crate::rand::rand;

pub fn value_of(color: u32) -> (f32, f32, f32) {
    let r: f32 = ((color >> 16) & 0xFF) as f32 / 255.;
    let g: f32 = ((color >> 8) & 0xFF) as f32 / 255.;
    let b: f32 = ((color) & 0xFF) as f32 / 255.;
    (r, g, b)
}

pub fn rand_dir() -> i32 {
    (rand() as i32 % 3) - 1
}

pub fn rand_dir_2() -> i32 {
    if (rand() % 2) == 0 {
        -1
    } else {
        1
    }
}

pub fn event_distance(a: (f32, f32), b: (f32, f32)) -> f32 {
    ((a.0 - b.0).powf(2.) + (b.1 - b.1).powf(2.)).sqrt()
}

pub fn add(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 + b.0, a.1 + b.1)
}

pub fn sub(a: (f32, f32), b: (f32, f32)) -> (f32, f32) {
    (a.0 - b.0, a.1 - b.1)
}

pub fn scalef(a: (f32, f32), scale: f32) -> (f32, f32) {
    (a.0 * scale, a.1 * scale)
}

pub fn magnitude(a: (f32, f32)) -> f32 {
    (a.0.powf(2.) + a.1.powf(2.)).sqrt()
}

pub fn norm(a: (f32, f32)) -> (f32, f32) {
    let mag = magnitude(a);
    (a.0 / mag, a.1 / mag)
}

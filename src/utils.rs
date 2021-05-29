pub fn value_of(color: u32) -> (f32, f32, f32) {
    let r: f32 = ((color >> 16) & 0xFF) as f32 / 255.;
    let g: f32 = ((color >> 8) & 0xFF) as f32 / 255.;
    let b: f32 = ((color) & 0xFF) as f32 / 255.;
    (r, g, b)
}

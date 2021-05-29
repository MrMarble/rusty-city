mod cell;
mod species;
mod universe;
mod utils;

use macroquad::prelude::*;
use species::Species;
use universe::Universe;

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
    let mut brush_size = 30.;
    let mut brush_mat = Species::Sand;
    let mut universe = Universe::new(screen_width() as i32, screen_height() as i32);
    loop {
        let (mx, my) = mouse_position();
        let (_, wy) = mouse_wheel();

        clear_background(WHITE);

        universe.tick();
        universe.render();

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
            universe.paint(mx as i32, my as i32, brush_size, brush_mat);
        }
        if is_key_pressed(KeyCode::Enter) {
            universe = Universe::new(screen_width() as i32, screen_height() as i32);
        }

        match get_last_key_pressed() {
            Some(KeyCode::Key0) => brush_mat = Species::Empty,
            Some(KeyCode::Key1) => brush_mat = Species::Sand,
            Some(KeyCode::Key2) => brush_mat = Species::Water,
            Some(KeyCode::Key3) => brush_mat = Species::Wall,
            _ => {}
        }

        if wy != 0. {
            brush_size = 5.0f64.max(100.0f64.min(brush_size + 5. * wy as f64));
        }

        draw_circle_lines(mx, my, (brush_size / 2.) as f32, 1., BLACK);
        next_frame().await
    }
}

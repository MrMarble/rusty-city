mod cell;
mod species;
mod universe;
mod utils;

use macroquad::prelude::*;
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
    let mut universe = Universe::new(screen_width() as i32, screen_height() as i32);
    loop {
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
            let pos = mouse_position();
            universe.paint(pos.0 as i32, pos.1 as i32);
        }
        if is_key_pressed(KeyCode::Enter) {
            universe = Universe::new(screen_width() as i32, screen_height() as i32);
        }

        next_frame().await
    }
}

mod cell;
mod debug;
mod species;
mod universe;
mod utils;

use macroquad::prelude::*;
use species::Species;
use universe::Universe;
use utils::{add, event_distance, norm, scalef, sub};

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
    let mut brush_size = 30f32;
    let scale: f32 = 5.;
    let mut brush_mat = Species::Sand;
    let mut universe = Universe::new(screen_width(), screen_height(), scale);

    let mut running = true;

    let mut last_mouse: (f32, f32) = (0., 0.);

    loop {
        let (mx, my) = mouse_position();

        clear_background(WHITE);

        if running {
            universe.tick();
        } else if is_key_pressed(KeyCode::Right) {
            universe.tick();
        }

        universe.render();

        {
            if is_mouse_button_pressed(MouseButton::Left) {
                last_mouse = mouse_position();
            }

            // Inputs
            if is_mouse_button_down(MouseButton::Left) {
                let mut mouse = (mx, my);
                let mut i = 0;

                universe.paint(mouse.0 / scale, mouse.1 / scale, brush_size, brush_mat);

                // Smooth painting... not very pleased with the result
                while event_distance(mouse, last_mouse) > brush_size / scale {
                    let d = event_distance(mouse, last_mouse);
                    mouse = add(
                        mouse,
                        scalef(norm(sub(last_mouse, mouse)), (brush_size / scale).min(d)),
                    );
                    i += 1;
                    if i > 15 {
                        break;
                    }
                    universe.paint(mouse.0 / scale, mouse.1 / scale, brush_size, brush_mat);
                }
                last_mouse = mouse;
            }

            match get_last_key_pressed() {
                Some(KeyCode::Key0) => brush_mat = Species::Empty,
                Some(KeyCode::Key1) => brush_mat = Species::Sand,
                Some(KeyCode::Key2) => brush_mat = Species::Water,
                Some(KeyCode::Key3) => brush_mat = Species::Wall,
                Some(KeyCode::Space) => running ^= true,
                Some(KeyCode::Enter) => {
                    universe = Universe::new(screen_width(), screen_height(), scale)
                }
                _ => {}
            }
        }

        {
            // Change brush size
            let (_, wy) = mouse_wheel();

            if wy != 0. {
                brush_size = 5f32.max(100f32.min(brush_size + 5. * wy));
            }
            draw_circle_lines(mx, my, brush_size / 2., 1., BLACK);
        }

        debug::debug(&universe);
        next_frame().await
    }
}

use macroquad::experimental::collections::storage;

use macroquad::{
    hash,
    prelude::*,
    ui::{root_ui, widgets::Window},
};

use crate::universe::Universe;

struct State {
    window_opened: bool,
}

pub fn debug(universe: &Universe) {
    if storage::try_get::<State>().is_none() {
        storage::store(State {
            window_opened: false,
        })
    }
    let mut state = storage::get_mut::<State>();

    draw_text(
        &format!("{}", macroquad::time::get_fps()),
        5.,
        20.,
        30.,
        DARKGRAY,
    );
    let selectable_rect = Rect::new(5., 5., 30., 30.);

    if selectable_rect.contains(mouse_position().into()) {
        draw_rectangle(
            selectable_rect.x,
            selectable_rect.y,
            30.0,
            30.0,
            Color::new(0., 0.0, 1.0, 0.4),
        );
        if is_mouse_button_pressed(MouseButton::Left) {
            state.window_opened ^= true;
        }
    }
    if state.window_opened {
        Window::new(hash!(), vec2(5., 50.), vec2(200., 100.))
            .label("Info")
            .ui(&mut *root_ui(), |ui| {
                ui.label(
                    vec2(10., 10.),
                    &format!("Window size: {}x{}", screen_width(), screen_height()),
                );
                ui.label(
                    vec2(10., 20.),
                    &format!(
                        "Cursor position: {}x{}",
                        mouse_position().0,
                        mouse_position().1
                    ),
                );
                ui.label(
                    vec2(10., 30.),
                    &format!("Game tick: {}", universe.generation()),
                );
                ui.label(
                    vec2(10., 40.),
                    &format!("Drawn cells: {}", universe.non_empty_cells),
                );
            });
    }
}

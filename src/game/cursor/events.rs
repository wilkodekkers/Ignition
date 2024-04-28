use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::math::{vec2, Vec2};
use bevy::prelude::{Commands, EventReader, Gizmos, MouseButton, Query};
use bevy::render::color;
use bevy::input::ButtonState;

use super::components::SelectionStartPosition;

fn mouse_entity_selection_start_event(
    mut mouse_button: EventReader<MouseButtonInput>,
    selection_start_query: Query<SelectionStartPosition>,
    mut commands: Commands
) {

    for button_input in mouse_button.read() {

        if button_input.button == MouseButton::Left && button_input.state == ButtonState::Pressed {
            let mouse_position = button_input.window.single().cursor_position.unwrap();
            commands.spawn(SelectionStartPosition { x: mouse_position.x, y: mouse_position.z });
        }

        if button_input.button == MouseButton::Left && button_input.state == ButtonState::Released {
            commands.entity(selection_start_query.first()).despawn();
        }

    }
}

fn mouse_entity_selection_drag_event(
    mut motion_evr: EventReader<MouseMotion>,
    start_position: Query<SelectionStartPosition>,
    mut gizmos: Gizmos,
) {

    for position in start_position.iter() {
        for ev in motion_evr.read() {
            let start: Vec2 = vec2(position.x, position.y);

            gizmos.rect_2d(
                start,
                0.0,
                vec2(position.x + ev.delta.x, position.y + ev.delta.y),
                color::Color::rgb(0.0, 255.0, 0.0)
            );
        }
    }
}
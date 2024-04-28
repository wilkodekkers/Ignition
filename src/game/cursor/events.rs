use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::math::{vec2, Vec2};
use bevy::prelude::{Commands, Entity, EventReader, Gizmos, MouseButton, Query, With};
use bevy::render::color;
use bevy::input::ButtonState;
use bevy::window::Window;

use super::components::SelectionStartPosition;

pub(crate) fn mouse_entity_selection_start_event(
    mut commands: Commands,
    windows: Query<&Window>,
    mut mouse_button: EventReader<MouseButtonInput>,
    selection_start_query: Query<Entity, With<SelectionStartPosition>>
) {
    for button_input in mouse_button.read() {
        if button_input.button == MouseButton::Left && button_input.state == ButtonState::Pressed {
            let Some(position) = windows.single().cursor_position() else {
                return;
            };

            commands.spawn(SelectionStartPosition { x: position.x, y: position.y });
        }

        if button_input.button == MouseButton::Left && button_input.state == ButtonState::Released {
            for entity in selection_start_query.iter() {
                commands.entity(entity).remove::<SelectionStartPosition>();
            }
        }

    }
}

pub(crate) fn mouse_entity_selection_drag_event(
    mut motion_evr: EventReader<MouseMotion>,
    start_position: Query<&SelectionStartPosition>,
    mut gizmos: Gizmos,
) {
    for position in start_position.iter() {
        for ev in motion_evr.read() {
            let start: Vec2 = vec2(
                position.x,
                position.y
            );

            gizmos.rect_2d(
                start,
                0.0,
                vec2(start.x + ev.delta.x, start.y + ev.delta.y),
                color::Color::rgb(0.0, 255.0, 0.0)
            );
        }
    }
}
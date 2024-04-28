use bevy::input::mouse::{MouseButtonInput, MouseMotion};
use bevy::prelude::*;

pub mod systems;
mod events;
mod components;

use systems::draw_cursor;

pub struct CursorPlugin;

impl Plugin for CursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_cursor)
            .add_event::<MouseButtonInput>()
            .add_event::<MouseMotion>();
    }
}

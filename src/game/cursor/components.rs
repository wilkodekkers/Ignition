use bevy::prelude::*;

#[derive(Component)]
pub struct SelectionStartPosition {
    pub(crate) x: f32,
    pub(crate) y: f32
}
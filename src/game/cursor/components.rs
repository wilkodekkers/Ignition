use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct SelectionStartPosition {
    pub(crate) x: i32,
    pub(crate) y: i32
}
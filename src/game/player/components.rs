use bevy::prelude::*;

#[derive(Component)]
pub struct Player {}

#[derive(Resource)]
pub struct Animations(pub Vec<Handle<AnimationClip>>);

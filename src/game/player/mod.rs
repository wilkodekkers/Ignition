use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, systems::run_animation);
    }
}

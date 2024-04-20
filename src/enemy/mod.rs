use bevy::prelude::*;

pub mod components;
pub mod systems;

use systems::spawn_enemy;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemy);
    }
}

mod cursor;
pub mod enemy;
pub mod player;
mod systems;

use crate::cursor::CursorPlugin;
use crate::player::PlayerPlugin;
use crate::enemy::EnemyPlugin;

use systems::*;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rts_camera::RtsCameraPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(DefaultPickingPlugins)
        .add_plugins(PlayerPlugin)
        .add_plugins(EnemyPlugin)
        .add_plugins(CursorPlugin)
        .add_systems(Startup, (spawn_camera, spawn_light, spawn_ground).chain())
        .run();
}

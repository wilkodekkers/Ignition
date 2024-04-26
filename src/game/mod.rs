pub mod cursor;
pub mod enemy;
pub mod player;
mod systems;

use bevy::prelude::*;
use bevy_mod_picking::DefaultPickingPlugins;
use bevy_rts_camera::RtsCameraPlugin;
use cursor::CursorPlugin;
use enemy::EnemyPlugin;
use player::PlayerPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
            // States
            .init_state::<SimulationState>()
            // Events
            .add_event::<GameOver>()
            // Plugins
            .add_plugins(RtsCameraPlugin)
            .add_plugins(DefaultPickingPlugins)
            .add_plugins(PlayerPlugin)
            .add_plugins(EnemyPlugin)
            .add_plugins(CursorPlugin)
            // Systems
            .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}

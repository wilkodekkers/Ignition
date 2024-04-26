mod components;
mod systems;

use bevy::prelude::*;
use crate::AppState;
use components::OnMainMenuScreen;
use crate::systems::despawn_screen;
use systems::main_menu_setup;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter
            .add_systems(OnEnter(AppState::Menu), main_menu_setup)
            // Exit
            .add_systems(OnExit(AppState::Menu), despawn_screen::<OnMainMenuScreen>);
    }
}

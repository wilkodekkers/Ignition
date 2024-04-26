mod events;
mod game;
mod main_menu;
mod systems;

use game::*;
use main_menu::*;
use systems::*;

use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        .init_state::<AppState>()
        // My Plugins
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        // Systems
        .add_systems(Update, transistion_to_game_state)
        .add_systems(Update, transistion_to_main_menu_state)
        .add_systems(Update, exit_game)
        // Startup Systems
        .add_systems(Startup, (spawn_camera, spawn_light, spawn_ground).chain())
        .run();
}

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    InGame,
    GameOver,
}

mod systems;
mod components;

use bevy::prelude::*;
use crate::AppState;
use systems::{splash_setup, countdown};
use components::OnSplashScreen;
use crate::systems::despawn_screen;

pub struct SplashPlugin;

impl Plugin for SplashPlugin {
    fn build(&self, app: &mut App) {
        app
            // Enter
            .add_systems(OnEnter(AppState::Splash), splash_setup)
            // Update
            .add_systems(Update, countdown.run_if(in_state(AppState::Splash)))
            // Exit
            .add_systems(OnExit(AppState::Splash), despawn_screen::<OnSplashScreen>);
    }
}



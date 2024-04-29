use bevy::prelude::*;

use crate::AppState;

use super::components::{OnSplashScreen, SplashTimer};

pub fn splash_setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Load logo
    let icon = asset_server.load("branding/logo.png");
    let background_image = asset_server.load("splash/background.png");
    // Background
    commands.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        OnSplashScreen,
    ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                image: UiImage::new(background_image),
                ..default()
            });
        });
    // Display logo
    commands.spawn((
        NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        OnSplashScreen,
    ))
        .with_children(|parent| {
            parent.spawn(ImageBundle {
                style: Style {
                    width: Val::Px(300.0),
                    ..default()
                },
                image: UiImage::new(icon),
                ..default()
            });
        });
    // Timer
    commands.insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)));
}

pub fn countdown(
    mut game_state: ResMut<NextState<AppState>>,
    time: Res<Time>,
    mut timer: ResMut<SplashTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(AppState::Menu);
    }
}

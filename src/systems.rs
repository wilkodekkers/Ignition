use bevy::{app::AppExit, prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_rts_camera::{Ground, RtsCamera, RtsCameraControls};

use crate::{AppState, SimulationState};

pub fn spawn_ground(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(PlaneMeshBuilder {
                half_size: Vec2::splat(100.0),
                ..default()
            }),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
        Ground,
    ));
}

pub fn spawn_light(mut commands: Commands) {
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle::default(),
        RtsCamera::default(),
        RtsCameraControls::default(),
    ));
}

pub fn transistion_to_game_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyG) {
        if state.get() != &AppState::Game {
            next_state.set(AppState::Game);
            println!("Transitioning to InGame");
        }
    }
}

pub fn transistion_to_main_menu_state(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    state: Res<State<AppState>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut next_simulation_state: ResMut<NextState<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyM) {
        if state.get() != &AppState::Menu {
            next_state.set(AppState::Menu);
            next_simulation_state.set(SimulationState::Paused);
            println!("Transitioning to MainMenu");
        }
    }
}

pub fn exit_game(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut app_exit_event_write: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_write.send(AppExit);
    }
}

pub fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

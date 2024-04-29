use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use super::components::{Player, Animations};

pub fn run_animation(
    animations: Res<Animations>,
    mut player_query: Query<&mut AnimationPlayer, Added<AnimationPlayer>>
) {
    for mut player in &mut player_query {
        player.play(animations.0[0].clone_weak()).repeat();
    }
}

pub fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Init animations
    commands.insert_resource(Animations(vec![asset_server.load("models/soldier.glb#Animation0")]));

    // Spawn model
    commands.spawn((
        SceneBundle {
            scene: asset_server.load("models/soldier.glb#Scene0"),
            ..default()
        },
        PickableBundle::default(),
        Player {},
    ));
}

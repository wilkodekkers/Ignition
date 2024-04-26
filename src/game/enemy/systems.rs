use bevy::prelude::*;
use bevy_mod_picking::PickableBundle;

use super::components::Enemy;

pub fn spawn_enemy(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(255, 144, 124)),
            transform: Transform::from_xyz(10.0, 0.5, 10.0),
            ..default()
        },
        PickableBundle::default(),
        Enemy {},
    ));
}

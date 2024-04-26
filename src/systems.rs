use bevy::{prelude::*, render::mesh::PlaneMeshBuilder};
use bevy_rts_camera::{Ground, RtsCamera, RtsCameraControls};

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

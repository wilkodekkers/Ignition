use bevy::prelude::*;
use bevy_rts_camera::{Ground, RtsCamera, RtsCameraControls, RtsCameraPlugin};


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, RtsCameraPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // ground
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Plane3d::default().mesh().size(80.0, 80.0)),
            material: materials.add(Color::GRAY),
            ..default()
        },
        Ground,
    ));
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle::default(),
        RtsCamera::default(),
        RtsCameraControls::default(),
    ));
}

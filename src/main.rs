use bevy::math::vec3;
use bevy::prelude::*;
use bevy::render::mesh::PlaneMeshBuilder;
use bevy_mod_picking::{DefaultPickingPlugins, PickableBundle};
use bevy_mod_picking::events::{Drag, Pointer};
use bevy_mod_picking::prelude::On;
use bevy_rts_camera::{RtsCamera, RtsCameraControls, RtsCameraPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RtsCameraPlugin)
        .add_plugins(DefaultPickingPlugins)
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
            mesh: meshes.add(PlaneMeshBuilder {
                half_size: Vec2::splat(2.5),
                ..default()
            }),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
            ..default()
        },
    ));
    // cube
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(0.0, 0.5, 0.0),
            ..default()
        },
        PickableBundle::default(),
        On::<Pointer<Drag>>::target_component_mut::<Transform>(|drag, transform| {
            println!("{}", drag.pointer_location.position);
            let normalized = drag.pointer_location.position.normalize();
            transform.translation = vec3(normalized.x, 0.0, normalized.y);
        })
    ));
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

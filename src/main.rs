use bevy::{math::vec3, prelude::*};
use ingredient::IngredientPlugin;

mod ingredient;
mod recipe;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_plugins(IngredientPlugin)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(3.0, 15.0, -6.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Icosphere {
                radius: 1.0,
                subdivisions: 2,
                ..Default::default()
            }
            .try_into()
            .unwrap(),
        ),
        material: materials.add(Color::RED.into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..Default::default()
    });

    commands.spawn(PbrBundle {
        mesh: meshes.add(
            shape::Plane {
                size: 100.0,
                ..Default::default()
            }
            .into(),
        ),
        material: materials.add(Color::WHITE.into()),
        ..Default::default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(vec3(-2.0, -30.0, 5.0), Vec3::Y),
        ..Default::default()
    });
}

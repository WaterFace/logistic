use bevy::{math::vec3, prelude::*};

use ingredient::{IngredientPlugin, IngredientType};
use recipe::{Recipe, RecipePlugin, Recipes};
use ui::UiPlugin;

mod ingredient;
mod recipe;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((IngredientPlugin, RecipePlugin, UiPlugin))
        .add_systems(Startup, (setup, setup_recipes))
        .run();
}

fn setup_recipes(mut recipes: ResMut<Recipes>) {
    // Ore generation
    recipes.add_recipe(Recipe {
        input: vec![],
        output: vec![(IngredientType::Ore, 1.0)],
        automatic: true,
        delay: 0.2,
    });

    // Manual ore generation, not hooked up yet
    recipes.add_recipe(Recipe {
        input: vec![],
        output: vec![(IngredientType::Ore, 50.0)],
        automatic: false,
        delay: 5.0,
    });

    // Coal generation
    recipes.add_recipe(Recipe {
        input: vec![],
        output: vec![(IngredientType::Coal, 5.0)],
        automatic: true,
        delay: 3.0,
    });

    // Smelting
    recipes.add_recipe(Recipe {
        input: vec![(IngredientType::Ore, 1.0), (IngredientType::Coal, 0.5)],
        output: vec![(IngredientType::Iron, 1.0)],
        automatic: true,
        delay: 1.0,
    });
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

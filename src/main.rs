use bevy::{math::vec3, prelude::*};

use camera::CameraPlugin;
use ingredient::{IngredientPlugin, IngredientType};
use node::NodePlugin;
use recipe::{Recipe, RecipePlugin, Recipes};
use ui::UiPlugin;

mod camera;
mod ingredient;
mod node;
mod recipe;
mod ui;
mod utils;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins((
            IngredientPlugin,
            RecipePlugin,
            UiPlugin,
            NodePlugin,
            CameraPlugin,
        ))
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

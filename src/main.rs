use bevy::{math::vec3, prelude::*};

use camera::CameraPlugin;
use floating_text::FloatingTextPlugin;
use ingredient::{IngredientPlugin, IngredientType};
use link::LinkPlugin;
use node::NodePlugin;
use picking::PickingPlugin;
use recipe::{Recipe, RecipePlugin, Recipes};
use ui::UiPlugin;

mod camera;
mod floating_text;
mod ingredient;
mod link;
mod node;
mod picking;
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
            LinkPlugin,
            CameraPlugin,
            PickingPlugin,
            FloatingTextPlugin,
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

    // Steel
    recipes.add_recipe(Recipe {
        input: vec![(IngredientType::Iron, 10.0), (IngredientType::Coal, 50.0)],
        output: vec![(IngredientType::Steel, 10.0)],
        automatic: true,
        delay: 15.0,
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: true,
            ..Default::default()
        },
        transform: Transform::IDENTITY.looking_to(vec3(-2.0, -30.0, 5.0), Vec3::Y),
        ..Default::default()
    });
}

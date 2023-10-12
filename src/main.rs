use bevy::{math::vec3, prelude::*};

use camera::CameraPlugin;
use floating_text::FloatingTextPlugin;
use game_builder::GameBuilder;
use ingredient::IngredientPlugin;
use link::LinkPlugin;
use node::NodePlugin;
use picking::PickingPlugin;
use recipe::RecipePlugin;
use ui::UiPlugin;

mod camera;
mod floating_text;
mod game_builder;
mod ingredient;
mod link;
mod node;
mod picking;
mod recipe;
mod ui;
mod utils;

fn main() {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
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
        .add_systems(Startup, setup);

    // TODO: load this stuff from configuration files
    let game_builder = GameBuilder::new()
        .add_ingredient("ingr_iron_ore", "Iron Ore", Some(150.0), Color::BEIGE)
        .add_ingredient("ingr_coal", "Coal", Some(300.0), Color::BLACK)
        .add_ingredient("ingr_iron_ingot", "Iron", Some(250.0), Color::GRAY)
        .add_ingredient("ingr_steel_ingot", "Steel", Some(100.0), Color::DARK_GRAY)
        .add_recipe(
            "reci_mine_iron_ore",
            [],
            [("ingr_iron_ore", 1.0)],
            0.2,
            true,
        )
        .add_recipe(
            "reci_manual_iron_ore",
            [],
            [("ingr_iron_ore", 50.0)],
            5.0,
            false,
        )
        .add_recipe("reci_mine_coal", [], [("ingr_coal", 5.0)], 3.0, true)
        .add_recipe(
            "reci_smelt_iron",
            [("ingr_iron_ore", 2.0), ("ingr_coal", 1.0)],
            [("ingr_iron_ingot", 2.0)],
            2.0,
            true,
        )
        .add_recipe(
            "reci_smelt_steel",
            [("ingr_iron_ingot", 10.0), ("ingr_coal", 50.0)],
            [("ingr_steel_ingot", 10.0)],
            15.0,
            true,
        );

    let (ingredients, recipes) = game_builder.build();

    app.insert_resource(ingredients).insert_resource(recipes);

    app.run();
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

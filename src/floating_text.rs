use bevy::prelude::*;

use crate::{
    camera::MainCamera,
    node::{Node, NodeRegistry},
    recipe::{RecipeEvent, Recipes},
    utils,
};

#[derive(Debug, Component, Default)]
struct FloatingText {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    timer: Timer,
}

fn position_floating_text(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Style, &mut Text, &mut FloatingText)>,
    main_camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok((main_camera, camera_transform)) = main_camera_query.get_single() else {
        return;
    };

    for (entity, mut style, mut text, mut floating_text) in query.iter_mut() {
        let dt = time.delta_seconds();

        floating_text.timer.tick(time.delta());
        if floating_text.timer.finished() {
            commands.entity(entity).despawn();
            continue;
        }

        // Borrow checker nonsense
        let vel = floating_text.velocity;
        floating_text.position += vel * dt;
        let accel = floating_text.acceleration;
        floating_text.velocity += accel * dt;

        if let Some(screen_pos) =
            main_camera.world_to_viewport(camera_transform, floating_text.position)
        {
            style.left = Val::Px(screen_pos.x);
            style.top = Val::Px(screen_pos.y);
            for section in text.sections.iter_mut() {
                section
                    .style
                    .color
                    .set_a(1.0 - floating_text.timer.percent());
            }
        }
    }
}

fn recipe_completion(
    mut commands: Commands,
    mut reader: EventReader<RecipeEvent>,
    recipes: Res<Recipes>,
    node_registry: Res<NodeRegistry>,
    node_query: Query<&Transform, With<Node>>,
    settings: Res<FloatingTextSettings>,
) {
    if !settings.enabled {
        return;
    }

    for ev in reader.into_iter() {
        let RecipeEvent::FinishRecipe(i) = ev else {
            // We only care about FinishRecipe events for now
            continue;
        };

        let recipe = recipes.get_recipe(i);

        for (ty, amount) in recipe.output.iter() {
            let Some(node_e) = node_registry.get(ty) else {
                warn!("No node registered for {}", ty.name());
                continue;
            };

            let Ok(node_transform) = node_query.get(*node_e) else {
                warn!(
                    "Couldn't find transform of node registered for {}",
                    ty.name()
                );
                continue;
            };

            commands.spawn(floating_text_bundle(
                utils::format_f64(*amount),
                node_transform.translation,
            ));
        }
    }
}

fn floating_text_bundle(label: String, position: Vec3) -> impl Bundle {
    (
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                align_content: AlignContent::Center,
                ..Default::default()
            },
            text: Text::from_section(
                label,
                TextStyle {
                    font_size: 24.0,
                    ..Default::default()
                },
            ),
            ..Default::default()
        },
        FloatingText {
            position,
            velocity: 4.0 * Vec3::Y,
            acceleration: 9.0 * Vec3::NEG_Y,
            timer: Timer::from_seconds(1.5, TimerMode::Once),
        },
    )
}

#[derive(Resource, Debug)]
pub struct FloatingTextSettings {
    pub enabled: bool,
}

impl Default for FloatingTextSettings {
    fn default() -> Self {
        FloatingTextSettings { enabled: true }
    }
}

pub struct FloatingTextPlugin;

impl Plugin for FloatingTextPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FloatingTextSettings>()
            .add_systems(Update, recipe_completion)
            .add_systems(Update, position_floating_text);
    }
}

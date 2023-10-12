use bevy::prelude::*;

use bevy_egui::{
    egui::{self, Ui},
    EguiContexts, EguiPlugin, EguiSet, EguiStartupSet,
};

use crate::{
    camera::SetTarget,
    ingredient::{IngredientIndex, Ingredients},
    node::NodeRegistry,
    recipe::{Recipe, Recipes},
    utils,
};

#[derive(Debug, Default, Resource)]
pub struct SelectedNode {
    pub selected: Option<IngredientIndex>,
}

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
            .init_resource::<SelectedNode>()
            .add_systems(
                Startup,
                configure_visuals.after(EguiStartupSet::InitContexts),
            )
            .add_systems(PostUpdate, draw_ui.after(EguiSet::InitContexts));
    }
}

fn configure_visuals(mut contexts: EguiContexts) {
    contexts.ctx_mut().set_visuals(egui::Visuals {
        button_frame: false,
        ..egui::Visuals::dark()
    });
}

fn draw_ui(
    mut contexts: EguiContexts,
    ingredients: Res<Ingredients>,
    mut hide_display: Local<bool>,
    mut owned_labels: Local<Vec<String>>,
    main_window_query: Query<Entity, With<bevy::window::PrimaryWindow>>,
    mut writer: EventWriter<SetTarget>,
    node_registry: Res<NodeRegistry>,
    selected_node: Res<SelectedNode>,
    recipes: Res<Recipes>,
) {
    let Ok(main_window) = main_window_query.get_single() else {
        return;
    };
    let Some(ctx) = contexts.try_ctx_for_window_mut(main_window) else {
        return;
    };
    if owned_labels.is_empty() {
        for _ in 0..ingredients.len() {
            owned_labels.push(String::new());
        }
    }
    egui::SidePanel::left("ingredient display")
        .resizable(false)
        .show_animated(ctx, !*hide_display, |ui| {
            for (ty, ingr) in ingredients.iter() {
                ui.horizontal(|ui| {
                    use std::fmt::Write;
                    let mut label = &mut owned_labels[ty.ix()];
                    label.clear();
                    write!(label, "{}: ", &ingr.name).unwrap();
                    utils::write_format_number(&mut label, ingr.current).unwrap();
                    if ui
                        .button(&*label)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        info!("{} clicked!", &ingr.name);
                        if let Some(e) = node_registry.get(&ty) {
                            writer.send(SetTarget(*e));
                        }
                    }
                });
            }
        });
    egui::SidePanel::left("ingredient display handle")
        .resizable(false)
        .exact_width(16.0)
        .show(ctx, |ui| {
            let rect = ui.max_rect();

            let label = match *hide_display {
                true => ">",
                false => "<",
            };
            let id = ui.horizontal_centered(|ui| ui.button(label).id).inner;
            if ui
                .interact(rect, id, egui::Sense::click().union(egui::Sense::hover()))
                .on_hover_cursor(egui::CursorIcon::PointingHand)
                .clicked()
            {
                *hide_display = !*hide_display;
            };
        });

    if let Some(selected_ingredient) = selected_node.selected {
        egui::SidePanel::right("node panel")
            .resizable(false)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    egui::Grid::new("recipe list")
                        .num_columns(1)
                        .striped(true)
                        .show(ui, |ui| {
                            // TODO: cache these results somewhere
                            for (_i, recipe_holder) in recipes.enumerate() {
                                if recipe_holder
                                    .recipe
                                    .output
                                    .iter()
                                    .any(|(i, _)| *i == selected_ingredient)
                                {
                                    recipe_item(ui, &recipe_holder.recipe, &ingredients);
                                    ui.end_row();
                                }
                            }
                        });
                });
            });
    }
}

fn recipe_item(ui: &mut Ui, recipe: &Recipe, ingredients: &Ingredients) {
    use std::fmt::Write;
    let mut s1 = "I recieve: ".to_string();
    for (i, q) in recipe.input.iter() {
        utils::write_format_number(&mut s1, q.value()).unwrap();
        write!(&mut s1, " {}, ", ingredients.get(*i).name).unwrap();
    }

    write!(&mut s1, "\nYou recieve: ").unwrap();
    for (i, q) in recipe.output.iter() {
        utils::write_format_number(&mut s1, q.value()).unwrap();
        write!(&mut s1, " {}, ", ingredients.get(*i).name).unwrap();
    }
    ui.label(s1);
}

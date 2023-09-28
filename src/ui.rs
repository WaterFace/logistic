use bevy::prelude::*;

use bevy_egui::{egui, EguiContexts, EguiPlugin, EguiSet, EguiStartupSet};

use crate::{
    ingredient::{IngredientType, Ingredients},
    utils,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(EguiPlugin)
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
) {
    let Ok(main_window) = main_window_query.get_single() else {
        return;
    };
    let Some(ctx) = contexts.try_ctx_for_window_mut(main_window) else {
        return;
    };
    if owned_labels.is_empty() {
        for _ in IngredientType::values() {
            owned_labels.push(String::new());
        }
    }
    egui::SidePanel::left("ingredient display")
        .resizable(false)
        .show_animated(ctx, !*hide_display, |ui| {
            for ty in IngredientType::values() {
                ui.horizontal(|ui| {
                    use std::fmt::Write;
                    let mut label = &mut owned_labels[*ty as usize];
                    label.clear();
                    write!(label, "{}: ", ty.name()).unwrap();
                    utils::write_format_f64(&mut label, ingredients.get(*ty).quantity).unwrap();
                    if ui
                        .button(&*label)
                        .on_hover_cursor(egui::CursorIcon::PointingHand)
                        .clicked()
                    {
                        info!("{} clicked!", ty.name());
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
}

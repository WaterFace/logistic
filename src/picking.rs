use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::EguiContexts;
use bevy_mod_picking::prelude::*;

#[derive(Default)]
struct CursorStack(Vec<bevy_egui::egui::CursorIcon>);

fn handle_pointer_state(
    mut contexts: EguiContexts,
    mut over_reader: EventReader<Pointer<Over>>,
    mut out_reader: EventReader<Pointer<Out>>,
    main_window_query: Query<Entity, With<PrimaryWindow>>,
    mut stack: Local<CursorStack>,
) {
    let Ok(window) = main_window_query.get_single() else {
        return;
    };
    let Some(ctx) = contexts.try_ctx_for_window_mut(window) else {
        return;
    };

    for _ev in over_reader.into_iter() {
        stack.0.push(bevy_egui::egui::CursorIcon::PointingHand);
    }

    for _ev in out_reader.into_iter() {
        let _ = stack.0.pop();
    }

    // main_window.cursor.icon = *stack.0.last().unwrap_or(&CursorIcon::Default);
    ctx.set_cursor_icon(
        *stack
            .0
            .last()
            .unwrap_or(&bevy_egui::egui::CursorIcon::Default),
    );
}

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(Update, handle_pointer_state);
    }
}

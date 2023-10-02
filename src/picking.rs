use bevy::{prelude::*, window::PrimaryWindow};
use bevy_mod_picking::prelude::*;

#[derive(Default)]
struct CursorStack(Vec<CursorIcon>);

fn handle_pointer_state(
    mut over_reader: EventReader<Pointer<Over>>,
    mut out_reader: EventReader<Pointer<Out>>,
    mut main_window_query: Query<&mut Window, With<PrimaryWindow>>,
    mut stack: Local<CursorStack>,
) {
    let Ok(mut main_window) = main_window_query.get_single_mut() else {
        return;
    };

    for _ev in over_reader.into_iter() {
        stack.0.push(CursorIcon::Hand);
    }

    for _ev in out_reader.into_iter() {
        let _ = stack.0.pop();
    }

    main_window.cursor.icon = *stack.0.last().unwrap_or(&CursorIcon::Default);
}

pub struct PickingPlugin;

impl Plugin for PickingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPickingPlugins)
            .add_systems(Update, handle_pointer_state);
    }
}

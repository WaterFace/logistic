use std::fmt::Write;

use bevy::prelude::*;

use belly::prelude::*;

use crate::ingredient::{IngredientType, Ingredients};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(BellyPlugin)
            .add_systems(Startup, setup)
            .add_systems(Update, update_ingredient_displays);
    }
}

#[derive(Component, Debug)]
struct IngredientDisplay {
    ty: IngredientType,
}

fn setup(mut commands: Commands) {
    commands.add(StyleSheet::load("ui/global.ess"));
    commands.add(StyleSheet::load("ui/ingredient_display.ess"));
    commands.add(eml! {
        <body c:ingredient-list on:ready=setup_ingredient_displays>
            <button c:header on:press=|_|{info!("Header clicked")}>"Ingredients"</button>
        </body>
    });
    // <label bind:value=from!(Ingredients:get(ty.clone()).quantity|fmt.quantity("{quantity:0.2}"))/>
}

fn setup_ingredient_displays(ctx: &mut EventContext<impl Event>) {
    for ty in IngredientType::values() {
        // Create a new entity holding an ingredient type
        let display = ctx.commands().spawn(IngredientDisplay { ty: *ty }).id();

        //
        ctx.select(".ingredient-list").add_child(eml! {
            <div c:ingredient-display-panel>
                <label {display} c:ingredient-display s:color=ty.color()/>
            </div>
        });
    }
}

fn update_ingredient_displays(
    mut query: Query<(&mut belly::widgets::common::Label, &IngredientDisplay)>,
    ingredients: Res<Ingredients>,
) {
    if !ingredients.is_changed() {
        return;
    }

    for (mut label, display) in &mut query {
        label.value.clear();
        let ty = display.ty;
        let quantity = ingredients.get(ty).quantity;
        write!(&mut label.value, "{}: {:0.1}", ty.name(), quantity).unwrap();
    }
}

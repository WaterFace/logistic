use bevy::{prelude::*, text::DEFAULT_FONT_HANDLE};

#[derive(Debug)]
pub struct Ingredient {
    quantity: f64,
    capacity: Option<f64>,
    color: Color,
}

impl Ingredient {
    pub fn add_ingredient(&mut self, amount: f64) {
        match self.capacity {
            None => self.quantity += amount,
            Some(cap) => self.quantity = f64::min(cap, self.quantity + amount),
        }
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            quantity: 0.0,
            capacity: None,
            color: Color::WHITE,
        }
    }
}

#[derive(Resource, Debug)]
pub struct Ingredients {
    pub ore: Ingredient,
    pub iron: Ingredient,
    pub coal: Ingredient,
}

impl Default for Ingredients {
    fn default() -> Self {
        Ingredients {
            ore: Ingredient {
                color: Color::BEIGE,
                ..Default::default()
            },
            iron: Ingredient {
                color: Color::GRAY,
                ..Default::default()
            },
            coal: Ingredient {
                color: Color::BLACK,
                capacity: Some(10.0),
                ..Default::default()
            },
        }
    }
}

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ingredients>()
            .add_systems(Update, tick_ingredients)
            .add_systems(Startup, setup_ui)
            .add_systems(Update, update_ingredient_displays.after(tick_ingredients));
    }
}

fn tick_ingredients(mut ingredients: ResMut<Ingredients>, time: Res<Time>) {
    let dt = time.delta_seconds_f64();

    ingredients.ore.add_ingredient(1.0 * dt);
    ingredients.coal.add_ingredient(2.5 * dt);

    // info!("{:?}", ingredients);
}

fn setup_ui(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|c| {
            ingredient_display(c, DEFAULT_FONT_HANDLE.typed(), 0);
            ingredient_display(c, DEFAULT_FONT_HANDLE.typed(), 1);
            ingredient_display(c, DEFAULT_FONT_HANDLE.typed(), 2);
        });
}

#[derive(Component, Debug)]
struct IngredientDisplayMarker(u32);

fn ingredient_display(parent: &mut ChildBuilder, font: Handle<Font>, ingredient_index: u32) {
    parent
        .spawn(NodeBundle {
            background_color: Color::DARK_GRAY.into(),
            style: Style {
                min_width: Val::Vw(15.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|c| {
            c.spawn(TextBundle {
                text: Text::from_section(
                    format!("{}", ingredient_index),
                    TextStyle {
                        font,
                        font_size: 24.0,
                        ..Default::default()
                    },
                ),
                ..Default::default()
            })
            .insert(IngredientDisplayMarker(ingredient_index));
        });
}

fn update_ingredient_displays(
    mut query: Query<(&mut Text, &IngredientDisplayMarker)>,
    ingredients: Res<Ingredients>,
) {
    for (mut display, &IngredientDisplayMarker(index)) in &mut query {
        let text = match index {
            0 => Text::from_section(
                format!("Ore: {:.0}", ingredients.ore.quantity),
                TextStyle {
                    color: ingredients.ore.color,
                    ..display.sections[0].style.clone()
                },
            ),
            1 => Text::from_section(
                format!("Iron: {:.0}", ingredients.iron.quantity),
                TextStyle {
                    color: ingredients.iron.color,
                    ..display.sections[0].style.clone()
                },
            ),
            2 => Text::from_section(
                format!("Coal: {:.0}", ingredients.coal.quantity),
                TextStyle {
                    color: ingredients.coal.color,
                    ..display.sections[0].style.clone()
                },
            ),
            _ => continue,
        };
        *display = text;
    }
}

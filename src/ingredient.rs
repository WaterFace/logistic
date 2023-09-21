use bevy::{prelude::*, text::DEFAULT_FONT_HANDLE};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum IngredientType {
    Ore,
    Coal,
    Iron,
}

impl IngredientType {
    pub fn name(&self) -> &'static str {
        match self {
            IngredientType::Ore => "Ore",
            IngredientType::Coal => "Coal",
            IngredientType::Iron => "Iron",
        }
    }
}

#[derive(Debug)]
pub struct Ingredient {
    pub quantity: f64,
    pub capacity: Option<f64>,
    pub color: Color,
}

impl Ingredient {
    pub fn add_ingredient(&mut self, amount: f64) {
        match self.capacity {
            None => self.quantity += amount,
            Some(cap) => self.quantity = f64::min(cap, self.quantity + amount),
        }
    }

    pub fn spend_ingredient(&mut self, amount: f64) {
        self.quantity = f64::max(0.0, self.quantity - amount);
    }

    pub fn with_color(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
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

#[derive(Resource)]
pub struct Ingredients {
    ore: Ingredient,
    coal: Ingredient,
    iron: Ingredient,
}

impl Ingredients {
    pub fn get(&self, ty: IngredientType) -> &Ingredient {
        match ty {
            IngredientType::Ore => &self.ore,
            IngredientType::Coal => &self.coal,
            IngredientType::Iron => &self.iron,
        }
    }

    pub fn get_mut(&mut self, ty: IngredientType) -> &mut Ingredient {
        match ty {
            IngredientType::Ore => &mut self.ore,
            IngredientType::Coal => &mut self.coal,
            IngredientType::Iron => &mut self.iron,
        }
    }
}

impl Default for Ingredients {
    fn default() -> Self {
        Ingredients {
            ore: Ingredient::with_color(Color::BEIGE),
            coal: Ingredient::with_color(Color::GRAY),
            iron: Ingredient::with_color(Color::BLACK),
        }
    }
}

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ingredients>()
            .add_systems(Startup, setup_ui)
            .add_systems(PostUpdate, update_ingredient_displays);
    }
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

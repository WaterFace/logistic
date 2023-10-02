use bevy::prelude::*;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[repr(usize)]
pub enum IngredientType {
    Ore,
    Coal,
    Iron,
    Steel,
}

// Keep this up to date
const INGREDIENT_TYPES: &'static [IngredientType] = &[
    IngredientType::Ore,
    IngredientType::Coal,
    IngredientType::Iron,
    IngredientType::Steel,
];

impl IngredientType {
    pub const fn values() -> &'static [Self] {
        INGREDIENT_TYPES
    }

    pub fn name(&self) -> &'static str {
        match self {
            IngredientType::Ore => "Ore",
            IngredientType::Coal => "Coal",
            IngredientType::Iron => "Iron",
            IngredientType::Steel => "Steel",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            IngredientType::Ore => Color::BEIGE,
            IngredientType::Coal => Color::BLACK,
            IngredientType::Iron => Color::GRAY,
            IngredientType::Steel => Color::WHITE,
        }
    }
}

#[derive(Debug)]
pub struct Ingredient {
    pub quantity: f64,
    pub capacity: Option<f64>,
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
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            quantity: 0.0,
            capacity: None,
        }
    }
}

#[derive(Resource)]
pub struct Ingredients {
    ingredients: [Ingredient; INGREDIENT_TYPES.len()],
}

impl Ingredients {
    pub fn get(&self, ty: IngredientType) -> &Ingredient {
        &self.ingredients[ty as usize]
    }

    pub fn get_mut(&mut self, ty: IngredientType) -> &mut Ingredient {
        &mut self.ingredients[ty as usize]
    }
}

impl Default for Ingredients {
    fn default() -> Self {
        Ingredients {
            ingredients: [
                Ingredient::default(), // Ore
                Ingredient::default(), // Coal
                Ingredient::default(), // Iron
                Ingredient::default(), // Steel
            ],
        }
    }
}

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ingredients>();
    }
}

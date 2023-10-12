use bevy::prelude::*;

use crate::quantity::Quantity;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct IngredientIndex(usize);

impl IngredientIndex {
    pub fn ix(&self) -> usize {
        self.0
    }
}

#[derive(Debug)]
pub struct Ingredient {
    pub name: String,
    pub color: Color,
    pub current: f64,
    pub capacity: Option<Quantity>,
}

impl Ingredient {
    pub fn add_ingredient(&mut self, amount: f64) {
        match self.capacity {
            None => self.current += amount,
            Some(cap) => self.current = f64::min(cap.value(), self.current + amount),
        }
    }

    pub fn spend_ingredient(&mut self, amount: f64) {
        self.current = f64::max(0.0, self.current - amount);
    }
}

impl Default for Ingredient {
    fn default() -> Self {
        Ingredient {
            name: String::new(),
            color: Color::WHITE,
            current: 0.0,
            capacity: None,
        }
    }
}

#[derive(Resource, Default)]
pub struct Ingredients {
    ingredients: Vec<Ingredient>,
}

impl Ingredients {
    pub fn get(&self, ty: IngredientIndex) -> &Ingredient {
        &self.ingredients[ty.0]
    }

    pub fn get_mut(&mut self, ty: IngredientIndex) -> &mut Ingredient {
        &mut self.ingredients[ty.0]
    }

    pub fn add_ingredient(&mut self, ingredient: Ingredient) -> IngredientIndex {
        let ix = self.ingredients.len();
        self.ingredients.push(ingredient);
        IngredientIndex(ix)
    }

    pub fn len(&self) -> usize {
        self.ingredients.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = (IngredientIndex, &Ingredient)> {
        self.ingredients
            .iter()
            .enumerate()
            .map(|(i, ingr)| (IngredientIndex(i), ingr))
    }
}

pub struct IngredientPlugin;

impl Plugin for IngredientPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Ingredients>();
    }
}

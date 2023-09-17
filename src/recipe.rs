use bevy::prelude::*;

use crate::ingredient::{IngredientType, Ingredients};

pub struct Recipe {
    input: Vec<(IngredientType, f64)>,
    output: Vec<(IngredientType, f64)>,
    automatic: bool,
    delay: f64,
}

impl Recipe {
    pub fn can_run_n_times(&self, ingredients: &Ingredients, n: u32) -> bool {
        for &(input_ingredient, amount) in &self.input {
            let ingredient = ingredients.get(input_ingredient);
            if ingredient.quantity < amount * n as f64 {
                return false;
            }
        }

        // TODO: decide what should happen if performing the recipe would exceed the cap on an output ingredient

        return true;
    }

    pub fn can_run(&self, ingredients: &Ingredients) -> bool {
        self.can_run_n_times(ingredients, 1)
    }
}

pub struct RecipeHolder {
    recipe: Recipe,
    automation_enabled: bool,
    time: f64,
}

#[derive(Resource)]
pub struct Recipes {
    recipes: Vec<RecipeHolder>,
}

fn tick_recipes(
    mut recipes: ResMut<Recipes>,
    mut ingredients: ResMut<Ingredients>,
    time: Res<Time>,
) {
    for recipe_holder in &mut recipes.recipes {
        if recipe_holder.time < recipe_holder.recipe.delay {
            recipe_holder.time += time.delta_seconds_f64();
        }
    }
}

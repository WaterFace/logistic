use bevy::prelude::*;

use crate::ingredient::{IngredientType, Ingredients};

#[derive(Debug)]
pub struct Recipe {
    pub input: Vec<(IngredientType, f64)>,
    pub output: Vec<(IngredientType, f64)>,
    pub automatic: bool,
    pub delay: f64,
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

    #[inline]
    pub fn can_run(&self, ingredients: &Ingredients) -> bool {
        self.can_run_n_times(ingredients, 1)
    }
}

#[derive(Debug)]
pub struct RecipeHolder {
    pub recipe: Recipe,
    pub automation_enabled: bool,
    pub time: f64,
    pub started: bool,
}

impl RecipeHolder {
    pub fn from_recipe(recipe: Recipe) -> Self {
        RecipeHolder {
            recipe,
            automation_enabled: true,
            time: 0.0,
            started: false,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct RecipeIndex(usize);

#[derive(Resource, Default)]
pub struct Recipes {
    recipes: Vec<RecipeHolder>,
}

impl Recipes {
    pub fn add_recipe(&mut self, recipe: Recipe) -> RecipeIndex {
        let i = self.recipes.len();
        let holder = RecipeHolder::from_recipe(recipe);
        self.recipes.push(holder);
        RecipeIndex(i)
    }

    pub fn enumerate(&self) -> impl Iterator<Item = (RecipeIndex, &RecipeHolder)> {
        self.recipes
            .iter()
            .enumerate()
            .map(|(i, r)| (RecipeIndex(i), r))
    }

    pub fn get_recipe(&self, index: &RecipeIndex) -> &Recipe {
        &self.get_recipe_holder(index).recipe
    }

    pub fn get_recipe_holder(&self, index: &RecipeIndex) -> &RecipeHolder {
        let RecipeIndex(i) = index;

        if *i >= self.recipes.len() {
            panic!(
                "Tried to access recipe at index {}, but only {} recipes are registered.",
                i,
                self.recipes.len()
            );
        }

        &self.recipes[*i]
    }

    pub fn get_recipe_holder_mut(&mut self, index: &RecipeIndex) -> &mut RecipeHolder {
        let RecipeIndex(i) = index;

        if *i >= self.recipes.len() {
            panic!(
                "Tried to access recipe at index {}, but only {} recipes are registered.",
                i,
                self.recipes.len()
            );
        }

        &mut self.recipes[*i]
    }
}

#[derive(Event, Debug)]
pub enum RecipeEvent {
    StartRecipe(RecipeIndex),
    FinishRecipe(RecipeIndex),
}

fn tick_recipes(
    mut recipes: ResMut<Recipes>,
    ingredients: Res<Ingredients>,
    mut writer: EventWriter<RecipeEvent>,
    time: Res<Time>,
) {
    for (i, recipe_holder) in recipes.recipes.iter_mut().enumerate() {
        if recipe_holder.started {
            recipe_holder.time += time.delta_seconds_f64();

            if recipe_holder.time >= recipe_holder.recipe.delay {
                writer.send(RecipeEvent::FinishRecipe(RecipeIndex(i)))
            }
        } else if recipe_holder.recipe.automatic
            && recipe_holder.automation_enabled
            && recipe_holder.recipe.can_run(&ingredients)
        {
            writer.send(RecipeEvent::StartRecipe(RecipeIndex(i)))
        }
    }
}

fn process_recipe_events(
    mut recipes: ResMut<Recipes>,
    mut ingredients: ResMut<Ingredients>,
    mut reader: EventReader<RecipeEvent>,
) {
    for event in reader.into_iter() {
        match event {
            RecipeEvent::StartRecipe(i) => {
                // TODO: figure out whether the can_run check should run here or wherever
                // these events are emitted. For now, it's wherever they're emitted

                let recipe_holder = recipes.get_recipe_holder_mut(i);

                // Deduct input ingredients
                for (ty, amount) in &recipe_holder.recipe.input {
                    let ingredient = ingredients.get_mut(*ty);
                    ingredient.spend_ingredient(*amount);
                }

                // Flag the recipe so it starts ticking
                recipe_holder.started = true;
            }
            RecipeEvent::FinishRecipe(i) => {
                let recipe_holder = recipes.get_recipe_holder_mut(i);

                // Add output ingredients
                for (ty, amount) in &recipe_holder.recipe.output {
                    let ingredient = ingredients.get_mut(*ty);
                    ingredient.add_ingredient(*amount);
                }

                // Reset the recipe
                recipe_holder.started = false;
                recipe_holder.time = 0.0;
            }
        }
    }
}

pub struct RecipePlugin;

impl Plugin for RecipePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RecipeEvent>()
            .init_resource::<Recipes>()
            .add_systems(Update, tick_recipes)
            .add_systems(Update, process_recipe_events.after(tick_recipes));
    }
}

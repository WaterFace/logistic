use bevy::{
    prelude::{error, Color},
    utils::HashMap,
};

struct Ingredient {
    id: String,
    name: String, // TODO: localization
    cap: Option<f64>,
    color: Color,
}

struct Recipe {
    id: String,
    input: Vec<(String, f64)>,
    output: Vec<(String, f64)>,
    delay: f64,
    automatic: bool,
}

pub struct GameBuilder {
    ingredients: Vec<Ingredient>,
    recipes: Vec<Recipe>,
}

impl Default for GameBuilder {
    fn default() -> Self {
        GameBuilder {
            ingredients: vec![],
            recipes: vec![],
        }
    }
}

impl GameBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_ingredient(
        mut self,
        id: impl Into<String>,
        name: impl Into<String>,
        cap: Option<f64>,
        color: Color,
    ) -> Self {
        let ingredient = Ingredient {
            id: id.into(),
            name: name.into(),
            cap,
            color,
        };

        self.ingredients.push(ingredient);

        self
    }

    pub fn add_recipe<S: Into<String>>(
        mut self,
        id: impl Into<String>,
        input: impl IntoIterator<Item = (S, f64)>,
        output: impl IntoIterator<Item = (S, f64)>,
        delay: f64,
        automatic: bool,
    ) -> Self {
        let recipe = Recipe {
            id: id.into(),
            input: Vec::from_iter(input.into_iter().map(|(s, q)| (s.into(), q))),
            output: Vec::from_iter(output.into_iter().map(|(s, q)| (s.into(), q))),
            delay,
            automatic,
        };

        self.recipes.push(recipe);

        self
    }

    pub fn build(self) -> (crate::ingredient::Ingredients, crate::recipe::Recipes) {
        let mut ingredient_map: HashMap<String, crate::ingredient::IngredientIndex> =
            HashMap::new();

        let mut ingredients_resource = crate::ingredient::Ingredients::default();

        for ingredient in self.ingredients {
            let new_ingr = crate::ingredient::Ingredient {
                capacity: ingredient.cap.map(|q| q.into()),
                color: ingredient.color,
                name: ingredient.name,
                ..Default::default()
            };
            let ix = ingredients_resource.add_ingredient(new_ingr);
            let id = ingredient.id.clone();
            match ingredient_map.insert(ingredient.id, ix) {
                None => {} // We're good
                Some(_) => {
                    error!(
                        "Multiple ingredients with id {}. Only the last one added will take effect",
                        id
                    );
                }
            }
        }

        let mut recipe_map: HashMap<String, crate::recipe::RecipeIndex> = HashMap::new();

        let mut recipes_resource = crate::recipe::Recipes::default();

        for recipe in self.recipes {
            let input = recipe.input.iter().filter_map(|(s, q)| {
                match ingredient_map.get(s) {
                    None => {
                        error!("Recipe {} refers to ingredient {}, but that ingredient was not registered", recipe.id, s);
                        None
                    }
                    Some(ix) => {
                        Some((*ix, crate::quantity::Quantity::new(*q)))
                    }
                }
            }).collect();

            let output = recipe.output.iter().filter_map(|(s, q)| {
                match ingredient_map.get(s) {
                    None => {
                        error!("Recipe {} refers to ingredient {}, but that ingredient was not registered", recipe.id, s);
                        None
                    }
                    Some(ix) => {
                        Some((*ix, crate::quantity::Quantity::new(*q)))
                    }
                }
            }).collect();

            let new_recipe = crate::recipe::Recipe {
                id: recipe.id.clone(),
                automatic: recipe.automatic,
                delay: recipe.delay.into(),
                input,
                output,
            };

            let ix = recipes_resource.add_recipe(new_recipe);

            let id = recipe.id.clone();
            match recipe_map.insert(recipe.id, ix) {
                None => {} // We're good
                Some(_) => {
                    error!(
                        "Multiple recipes with id {}. Only the last one added will take effect",
                        id
                    );
                }
            }
        }

        (ingredients_resource, recipes_resource)
    }
}

use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};

use crate::ingredient::IngredientType;

#[derive(Component)]
pub struct Node {
    pub ty: IngredientType,
    pub visible: bool,
}

#[derive(Resource, Default)]
pub struct NodeRegistry {
    map: HashMap<IngredientType, Entity>,
}

impl core::ops::Deref for NodeRegistry {
    type Target = HashMap<IngredientType, Entity>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl core::ops::DerefMut for NodeRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NodeRegistry>()
            .add_systems(Startup, setup_nodes);
    }
}

fn setup_nodes(
    mut commands: Commands,
    mut registry: ResMut<NodeRegistry>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Icosphere {
            radius: 1.0,
            subdivisions: 2,
            ..Default::default()
        }
        .try_into()
        .unwrap(),
    );

    for ty in IngredientType::values() {
        let t = 2.0 * PI * (*ty as usize as f32 / IngredientType::values().len() as f32);
        let e = commands
            .spawn((
                PbrBundle {
                    mesh: mesh.clone(),
                    material: materials.add(ty.color().into()),
                    transform: Transform::from_xyz(2.0 * f32::cos(t), 0.5, 2.0 * f32::sin(t)),
                    ..Default::default()
                },
                Node {
                    ty: *ty,
                    visible: true,
                },
            ))
            .id();

        match registry.get_mut(ty) {
            None => {
                registry.insert(*ty, e);
            }
            Some(old_e) => {
                *old_e = e;
                warn!("Overwriting association in NodeRegistry")
            }
        }
    }
}

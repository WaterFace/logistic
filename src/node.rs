use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};

use bevy_mod_picking::prelude::*;

use crate::ingredient::IngredientType;

#[derive(Component, Debug)]
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

fn add_pointer_event_listeners(mut commands: Commands, query: Query<Entity, Added<Node>>) {
    for e in query.iter() {
        commands
            .entity(e)
            .insert((PickableBundle::default(), RaycastPickTarget::default()))
            .insert(On::<Pointer<Over>>::run(handle_pointer_over))
            .insert(On::<Pointer<Out>>::run(handle_pointer_out))
            .insert(On::<Pointer<Click>>::run(handle_pointer_click));
    }
}

fn handle_pointer_over(listener: Listener<Pointer<Over>>, query: Query<&Node>) {
    if let Ok(node) = query.get(listener.target) {
        info!("Pointer over {:?}", node);
    }
}

fn handle_pointer_out(listener: Listener<Pointer<Out>>, query: Query<&Node>) {
    if let Ok(node) = query.get(listener.target) {
        info!("Pointer out {:?}", node);
    }
}

fn handle_pointer_click(listener: Listener<Pointer<Click>>, query: Query<&Node>) {
    if let Ok(node) = query.get(listener.target) {
        info!("Pointer click {:?}", node);
    }
}

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NodeRegistry>()
            .add_systems(Startup, setup_nodes)
            .add_systems(Update, add_pointer_event_listeners);
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

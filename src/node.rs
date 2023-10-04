use std::f32::consts::PI;

use bevy::{prelude::*, utils::HashMap};

use bevy_mod_picking::prelude::*;

use crate::ingredient::IngredientType;

#[derive(Component, Debug)]
pub struct Node {
    pub ty: IngredientType,
    pub visible: bool,
}

#[derive(Component, Debug)]
struct NodeScale {
    target_scale: f32,
}

impl Default for NodeScale {
    fn default() -> Self {
        NodeScale { target_scale: 1.0 }
    }
}

fn scale_nodes(mut query: Query<(&mut Transform, &NodeScale)>, time: Res<Time>) {
    const SCALE_SPEED: f32 = 15.0;
    for (mut transform, node_scale) in query.iter_mut() {
        let target_scale = node_scale.target_scale;
        let current_scale = transform.scale.length();
        let ratio = target_scale / current_scale;

        transform.scale *= ratio.powf(time.delta_seconds() * SCALE_SPEED);
    }
}

/// Stores a mapping from `IngredientType` to the `Entity` of the corresponding node
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
            .insert((
                On::<Pointer<Over>>::run(handle_pointer_over),
                On::<Pointer<Out>>::run(handle_pointer_out),
                On::<Pointer<Click>>::run(handle_pointer_click),
            ));
    }
}

fn handle_pointer_over(
    listener: Listener<Pointer<Over>>,
    mut query: Query<(&Node, &mut NodeScale)>,
) {
    if let Ok((_, mut node_scale)) = query.get_mut(listener.target) {
        node_scale.target_scale = 1.1;
    }
}

fn handle_pointer_out(listener: Listener<Pointer<Out>>, mut query: Query<(&Node, &mut NodeScale)>) {
    if let Ok((_, mut node_scale)) = query.get_mut(listener.target) {
        node_scale.target_scale = 1.0;
    }
}

// Does nothing for now
fn handle_pointer_click() {}

pub struct NodePlugin;

impl Plugin for NodePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<NodeRegistry>()
            .add_systems(Startup, setup_nodes)
            .add_systems(Update, (add_pointer_event_listeners, scale_nodes));
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
                NodeScale::default(),
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

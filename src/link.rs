use bevy::{prelude::*, utils::HashMap};

use crate::{
    camera::MainCamera,
    node::{Node, NodeRegistry},
    recipe::Recipes,
};

#[derive(Component)]
pub struct Link {
    e1: Entity,
    e2: Entity,
}

// This is probably a bad idea
impl Default for Link {
    fn default() -> Self {
        Link {
            e1: Entity::PLACEHOLDER,
            e2: Entity::PLACEHOLDER,
        }
    }
}

#[derive(Default, Bundle)]
struct LinkBundle {
    pub mesh: Handle<Mesh>,
    pub material: Handle<StandardMaterial>,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub computed_visibility: ComputedVisibility,
    pub link: Link,
}

#[derive(Default, Resource)]
struct LinkRegistry {
    map: HashMap<(Entity, Entity), Entity>,
}

impl LinkRegistry {
    fn add_link(&mut self, e1: Entity, e2: Entity, link: Entity) -> Option<Entity> {
        let k = (Entity::min(e1, e2), Entity::max(e1, e2));
        self.map.insert(k, link)
    }

    fn contains(&self, e1: Entity, e2: Entity) -> bool {
        let k = (Entity::min(e1, e2), Entity::max(e1, e2));
        self.map.contains_key(&k)
    }
}

#[derive(Resource)]
struct LinkVisuals {
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
}

fn setup_link_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(
        shape::Quad {
            size: Vec2::splat(1.0),
            ..Default::default()
        }
        .into(),
    );

    let material = materials.add(Color::BLUE.into());

    commands.insert_resource(LinkVisuals { mesh, material })
}

fn create_links(
    recipes: Res<Recipes>,
    mut link_registry: ResMut<LinkRegistry>,
    nodes: Res<NodeRegistry>,
    link_visuals: Res<LinkVisuals>,
    mut commands: Commands,
) {
    for (_i, holder) in recipes.enumerate() {
        let recipe = &holder.recipe;
        for (t1, _) in recipe.input.iter() {
            for (t2, _) in recipe.output.iter() {
                let Some(e1) = nodes.get(t1) else {
                    continue;
                };

                let Some(e2) = nodes.get(t2) else {
                    continue;
                };

                if link_registry.contains(*e1, *e2) {
                    continue;
                }

                let link = commands
                    .spawn(LinkBundle {
                        link: Link { e1: *e1, e2: *e2 },
                        mesh: link_visuals.mesh.clone(),
                        material: link_visuals.material.clone(),
                        ..Default::default()
                    })
                    .id();

                link_registry.add_link(*e1, *e2, link);
            }
        }
    }
}

fn update_links(
    mut link_query: Query<(&mut Transform, &mut Visibility, &Link), Without<MainCamera>>,
    node_query: Query<(&Transform, &Node), (Without<Link>, Without<MainCamera>)>,
    camera_query: Query<&Transform, With<MainCamera>>,
) {
    let Ok(camera_transform) = camera_query.get_single() else {
        debug!("No main camera");
        return;
    };

    for (mut link_transform, mut link_visibility, Link { e1, e2 }) in link_query.iter_mut() {
        let Ok((transform_a, node_a)) = node_query.get(*e1) else {
            debug!("No node 1");
            continue;
        };
        let Ok((transform_b, node_b)) = node_query.get(*e2) else {
            debug!("No node 2");
            continue;
        };
        let a = transform_a.translation;
        let b = transform_b.translation;
        let c = camera_transform.translation;
        let closest_point_on_line = a + (c - a).project_onto(b - a);
        let dir_to_camera = closest_point_on_line - c;

        link_transform.translation = (a + b) / 2.0;
        link_transform.look_to(dir_to_camera, b - a);
        link_transform.scale.y = (b - a).length();
        link_transform.scale.x = 0.2;

        if !node_a.visible || !node_b.visible {
            *link_visibility = Visibility::Hidden;
        } else {
            *link_visibility = Visibility::Visible;
        }
    }
}

pub struct LinkPlugin;

impl Plugin for LinkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LinkRegistry>()
            .add_systems(
                Startup,
                (setup_link_visuals, apply_deferred, create_links).chain(),
            )
            .add_systems(Update, update_links);
    }
}

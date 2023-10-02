use bevy::{input::mouse::MouseWheel, prelude::*};

#[derive(Component)]
pub struct MainCamera;

#[derive(Component)]
pub struct CameraZoom {
    distance: f32,
    min_dist: f32,
    max_dist: f32,
}

impl Default for CameraZoom {
    fn default() -> Self {
        CameraZoom {
            distance: 15.0,
            min_dist: 5.0,
            max_dist: 30.0,
        }
    }
}

#[derive(Component, Default)]
pub struct CameraFocus {
    target: Vec3,
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(3.0, 15.0, -16.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        MainCamera,
        CameraZoom::default(),
        CameraFocus::default(),
        bevy_mod_picking::prelude::RaycastPickCamera::default(),
    ));
}

fn follow_target(
    mut query: Query<(&mut Transform, &CameraZoom, &CameraFocus), With<MainCamera>>,
    time: Res<Time>,
) {
    let Ok((mut transform, zoom, focus)) = query.get_single_mut() else {
        return;
    };

    let current_position = transform.translation;
    let desired_position = focus.target - transform.forward() * zoom.distance;
    let translation = (desired_position - current_position) * time.delta_seconds();

    transform.translation += translation;
}

fn zoom(
    mut query: Query<&mut CameraZoom, With<MainCamera>>,
    mut mousewheel: EventReader<MouseWheel>,
) {
    let Ok(mut zoom) = query.get_single_mut() else {
        return;
    };

    for ev in mousewheel.into_iter() {
        let MouseWheel { y, .. } = ev;
        zoom.distance = f32::clamp(zoom.distance - 2.0 * y, zoom.min_dist, zoom.max_dist);
    }
}

#[derive(Event)]
pub struct SetTarget(pub Entity);

fn set_target(
    mut main_camera_query: Query<&mut CameraFocus, With<MainCamera>>,
    other_query: Query<&Transform, Without<MainCamera>>,
    mut events: EventReader<SetTarget>,
) {
    let Ok(mut camera_focus) = main_camera_query.get_single_mut() else {
        return;
    };

    for ev in events.into_iter() {
        let SetTarget(e) = ev;
        let Ok(transform) = other_query.get(*e) else {
            warn!(
                "Tried to set focus to an entity that doesn't exist or does not have a transform"
            );
            continue;
        };

        camera_focus.target = transform.translation;
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetTarget>()
            .add_systems(Startup, setup_camera)
            .add_systems(Update, (follow_target, set_target, zoom));
    }
}

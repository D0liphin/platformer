use bevy::prelude::*;

mod camera_controls;
pub use camera_controls::*;

fn sys_spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    });
}

/// Spawns a camera in your world
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys_spawn_camera);
    }
}
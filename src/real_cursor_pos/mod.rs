use bevy::{prelude::*, window::PrimaryWindow};

use crate::util::get_real_cursor_position;

#[derive(Resource)]
pub struct RealCursorPosition(Option<Vec2>);

impl RealCursorPosition {
    pub fn get(&self) -> Option<Vec2> {
        self.0.clone()
    }
}

fn sys_update_real_cursor_position(
    mut real_cursor_pos: ResMut<RealCursorPosition>,
    q_window: Query<&Window, With<PrimaryWindow>>,
    q_camera: Query<(&OrthographicProjection, &Transform), With<Camera2d>>,
) {
    let (camera_orth_proj, camera_trf) = q_camera.single();
    real_cursor_pos.0 =
        get_real_cursor_position(q_window.single(), camera_orth_proj, &camera_trf.translation);
}

pub struct RealCursorPositionPlugin;

impl Plugin for RealCursorPositionPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(RealCursorPosition(None))
            .add_systems(PreUpdate, sys_update_real_cursor_position);
    }
}

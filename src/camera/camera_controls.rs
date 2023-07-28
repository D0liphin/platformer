use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::egui;

use crate::objects::player::Player;
pub struct CameraControlsPlugin;

impl Plugin for CameraControlsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CameraControlsUiState {
            enabled: true,
            follow_player: false,
        })
        .insert_resource(AbsoluteMouseMotion::default())
        .add_systems(Last, sys_camera_controls)
        .add_systems(
            Last,
            sys_update_absolute_mouse_motion.after(sys_camera_controls),
        );
    }
}

#[derive(Resource)]
pub struct CameraControlsUiState {
    enabled: bool,
    follow_player: bool,
}

/// Add a UI for the camera controls to your debug window
pub fn show_camera_controls_ui(ui: &mut egui::Ui, state: &mut CameraControlsUiState) {
    ui.heading("Camera Controls");
    ui.checkbox(&mut state.enabled, "enabled");
    ui.checkbox(&mut state.follow_player, "follow player");
    ui.separator();
}

#[derive(Resource, Default, Debug)]
struct AbsoluteMouseMotion {
    last_physical_position: Option<Vec2>,
    delta: Option<Vec2>,
}

fn sys_update_absolute_mouse_motion(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut absolute_mouse_motion: ResMut<AbsoluteMouseMotion>,
) {
    let Ok(window) = window_query.get_single() else {
        return;
    };

    absolute_mouse_motion.delta = if let (Some(last_pos), Some(pos)) = (
        absolute_mouse_motion.last_physical_position,
        window.physical_cursor_position(),
    ) {
        Some(pos - last_pos)
    } else {
        None
    };

    absolute_mouse_motion.last_physical_position = window.physical_cursor_position();
}

fn sys_camera_controls(
    ui_state: Res<CameraControlsUiState>,
    mouse_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    absolute_mouse_motion: Res<AbsoluteMouseMotion>,
    q_player: Query<&GlobalTransform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
    mut mouse_wheel_evr: EventReader<MouseWheel>,
) {
    if !ui_state.enabled {
        return;
    }

    let (mut camera_trf, mut orth_proj) = camera_query.get_single_mut().expect("no camera exists!");

    // panning
    if ui_state.follow_player {
        let player_trf = q_player.single();
        let (cx, cy) = (camera_trf.translation.x, camera_trf.translation.y);
        let (px, py) = (player_trf.translation().x, player_trf.translation().y);

        camera_trf.translation.x = ((px - cx) * 0.5) + cx;
        camera_trf.translation.y = ((py - cy) * 0.5) + cy;
    } else {
        if mouse_input.pressed(MouseButton::Left) && keyboard_input.pressed(KeyCode::AltLeft) {
            if let Some(pan_by) = absolute_mouse_motion.delta {
                let pan_by = pan_by * orth_proj.scale;
                camera_trf.translation += Vec3::new(-pan_by.x, pan_by.y, 0.);
            }
        }
    }

    for ev in mouse_wheel_evr.iter() {
        match ev.unit {
            MouseScrollUnit::Line => {
                let scale_by = 1. - 0.1 * ev.y;
                let scale_by = if scale_by < 0.9 { 0.9 } else { scale_by };
                orth_proj.scale *= scale_by;
            }
            MouseScrollUnit::Pixel => {
                // TODO
                eprintln!("pixel scroll units not currently supported");
            }
        }
    }
}

use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
pub mod gizmos_ext;

use crate::{
    camera::{show_camera_controls_ui, CameraControlsUiState},
    command_line::{show_command_line_ui, CommandLineUiState}, objects::player::Player,
};

fn sys_debug_ui(
    mut contexts: EguiContexts,
    mut commands: Commands,
    mut camera_controls_ui_state: ResMut<CameraControlsUiState>,
    mut command_line_ui_state: ResMut<CommandLineUiState>,
    mut q_player: Query<&mut Transform, With<Player>>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        show_camera_controls_ui(ui, &mut camera_controls_ui_state);
        show_command_line_ui(&mut commands, &mut q_player, ui, &mut command_line_ui_state);
    });
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_debug_ui);
    }
}

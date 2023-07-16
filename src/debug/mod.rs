use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};

use crate::camera::{show_camera_controls_ui, CameraControlsUiState};

fn sys_debug_ui(
    mut contexts: EguiContexts,
    mut camera_controls_ui_state: ResMut<CameraControlsUiState>,
) {
    egui::Window::new("Debug").show(contexts.ctx_mut(), |ui| {
        show_camera_controls_ui(ui, &mut camera_controls_ui_state);
    });
}

pub struct DebugUiPlugin;

impl Plugin for DebugUiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sys_debug_ui);
    }
}

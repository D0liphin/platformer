mod camera;
mod debug;
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use camera::{CameraControlsPlugin, CameraPlugin};
use debug::DebugUiPlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin))
        .add_plugins((CameraPlugin, CameraControlsPlugin))
        .add_plugins(DebugUiPlugin)
        .run();
}

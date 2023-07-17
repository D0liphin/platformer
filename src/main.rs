mod camera;
mod command_line;
mod debug;
mod level;
mod types;
mod physics;

use bevy_inspector_egui::quick::WorldInspectorPlugin;
pub use types::*;

use bevy::{prelude::*, utils::HashMap};
use bevy_egui::EguiPlugin;
use camera::{CameraControlsPlugin, CameraPlugin};
use command_line::CommandLinePlugin;
use debug::DebugUiPlugin;
use level::*;

fn sys_spawn_grassland(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    *clear_color = ClearColor(Color::rgba(0.9, 0.95, 1., 1.));
    let level_descriptor = LevelDescriptor {
        ident: "grassland",
        hitboxes: HashMap::new(),
    };

    level_descriptor.spawn(&asset_server, &mut commands);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin,
        ))
        .add_plugins(CameraPlugin)
        // debug plugins
        .add_plugins((CameraControlsPlugin, CommandLinePlugin, DebugUiPlugin))
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(Startup, sys_spawn_grassland) // TODO: remove
        .run();
}

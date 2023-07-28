#![feature(generic_const_exprs, core_intrinsics)]

mod animation;
mod bitflags;
mod bytes_util;
mod camera;
mod command_line;
mod debug;
mod level;
mod level_editor;
mod objects;
mod physics;
mod types;
mod util;

use std::path::PathBuf;

use animation::AnimationPlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_rapier2d::{prelude::*, render::RapierDebugRenderPlugin};
use level_editor::{ChunkDecoration, ChunkDescriptor, ChunkLocation, Level};
use objects::{player::PlayerPlugin, KinematicObjectPlugin};
pub use types::*;

use bevy::{asset::FileAssetIo, prelude::*, utils::HashMap};
use bevy_egui::EguiPlugin;
use camera::{CameraControlsPlugin, CameraPlugin};
use command_line::CommandLinePlugin;
use debug::DebugUiPlugin;

macro_rules! map {
    {$($key:expr => $value:expr),*} => {{
        let mut map = HashMap::new();
        $(
            map.insert($key, $value);
        )*
        map
    }};
}

fn sys_spawn_grassland(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut clear_color: ResMut<ClearColor>,
) {
    let mut level = Level::open("grassland");
    level.insert(
        ChunkLocation::new(0, 0),
        ChunkDescriptor {
            bg: ChunkDecoration {
                path: "grassland_0_0_bg.png".into(),
            },
        },
    );
    dbg!(level.get(&ChunkLocation::new(0, 0)));
    *clear_color = ClearColor(Color::rgba(0.9, 0.95, 1., 1.));
    // let level_descriptor = LevelDescriptor {
    //     ident: "grassland",
    //     hitboxes: map! {
    //         ChunkLocation::new(0, 0) => Array::from([
    //             LevelHitboxDescriptor::aabb(72, 134, 119, 80),
    //             LevelHitboxDescriptor::aabb(120, 79, 135, 48),
    //             LevelHitboxDescriptor::aabb(136, 86, 151, 80),
    //             LevelHitboxDescriptor::aabb(120, 142, 127, 135),
    //             LevelHitboxDescriptor::aabb(128, 158, 143, 143),
    //             LevelHitboxDescriptor::aabb(144, 207, 159, 159),
    //             LevelHitboxDescriptor::aabb(160, 199, 175, 128),
    //             LevelHitboxDescriptor::aabb(176, 127, 183, 96),
    //             LevelHitboxDescriptor::aabb(168, 103, 175, 96),
    //             LevelHitboxDescriptor::aabb(152, 95, 167, 88),
    //             LevelHitboxDescriptor::aabb(88, 203, 103, 172),
    //         ])
    //     },
    // };

    // level_descriptor.spawn(&asset_server, &mut commands);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            EguiPlugin,
        ))
        .add_plugins(CameraPlugin)
        .add_plugins(AnimationPlugin)
        // debug plugins
        .add_plugins((
            CameraControlsPlugin,
            CommandLinePlugin,
            DebugUiPlugin,
            RapierDebugRenderPlugin::default(),
            WorldInspectorPlugin::new(),
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(128.0))
        .add_plugins((KinematicObjectPlugin))
        .add_systems(Startup, sys_spawn_grassland) // TODO: remove
        .run();
}

pub mod controller;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{animation::*, animations, bitflags::BitFlags};

use self::controller::sys_player_controller;

use super::KinematicObject;

/// The child of PlayerObjectBundle
#[derive(Bundle)]
struct PlayerDecorationBundle {
    sprite: AnimatedSpriteSheetBundle,
}

#[derive(Bundle)]
struct PlayerObjectBundle {
    /// KinematicVelocityBased
    // rigid_body: RigidBody,
    collider: Collider,
    transform: TransformBundle,
    /// Name::new("Player")
    name: Name,
    visibility: VisibilityBundle,
    velocity: Velocity,
    object: KinematicObject,
    player: Player,
}

#[derive(Component)]
pub struct Player;

impl Player {
    pub fn spawn(commands: &mut Commands, animations: &Animations) {
        let player_decoration = commands
            .spawn(PlayerDecorationBundle {
                sprite: AnimatedSpriteSheetBundle {
                    sprite: SpriteSheetBundle {
                        // TODO: this is a magic number for part of the grassland level
                        transform: Transform::from_xyz(0., 1., 0.),
                        ..default()
                    },
                    animation: animations.get(&animations::PLAYER_IDLE).unwrap().clone(),
                    ..default()
                },
            })
            .id();
        commands
            .spawn(PlayerObjectBundle {
                // rigid_body: RigidBody::KinematicVelocityBased,
                collider: Collider::cuboid(3.5, 7.),
                transform: TransformBundle::from(Transform::from_xyz(49., -4., 0.)),
                name: Name::new("Player"),
                visibility: VisibilityBundle::default(),
                velocity: Velocity::zero(),
                object: KinematicObject::new(),
                player: Player,
            })
            .push_children(&[player_decoration]);
    }
}

pub struct PlayerPlugin;

fn sys_setup_player(
    mut commands: Commands,
    mut animations: ResMut<Animations>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // animations
    let mut builder = AnimationBuilder::new(&asset_server, &mut texture_atlases);
    animations.add(
        builder
            .from_grid(
                animations::PLAYER_IDLE,
                "objects/player/player_idle_16x16_5_1_0x0_1x0.png",
                Vec2::new(16., 16.),
                5,
                1,
                Some(Vec2::new(1., 0.)),
                None,
            )
            .with_flow(AnimationFlow::Looping)
            .with_frame_duration(0.2)
            .with_frames([0, 1, 2, 3, 4]),
    );

    // spawn player
    Player::spawn(&mut commands, &animations);
}

fn sys_player_animation() {}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, sys_setup_player)
            .add_systems(Update, sys_player_animation)
            .add_systems(Update, sys_player_controller)
            .add_systems(
                PostUpdate,
                |mut evr_collisions: EventReader<CollisionEvent>| {
                    for ev in evr_collisions.iter() {
                        dbg!(ev);
                    }
                },
            );
    }
}

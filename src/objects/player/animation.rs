use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::Player;
use crate::{animation::*, animations, objects::{KinematicObject, KinematicVelocity}};

/// Adds jump, land, crouch (for now), fall and float animations for the player
fn setup_jump_animations(animations: &mut Animations, builder: &mut AnimationBuilder) {
    let jump = builder
        .from_grid(
            animations::PLAYER_JUMP,
            "objects/player/player_jump_16x16_6_1_0x0_1x0.png",
            Vec2::new(16., 16.),
            6,
            1,
            Some(Vec2::new(1., 0.)),
            None,
        )
        .with_flow(AnimationFlow::Once)
        .with_frame_duration(0.04);
    animations.add(jump.clone().with_frames([0, 1, 2]));
    animations.add(
        jump.clone()
            .with_key(animations::PLAYER_FLOAT)
            .with_flow(AnimationFlow::Static)
            .with_frames([3]),
    );
    animations.add(
        jump.clone()
            .with_key(animations::PLAYER_FALL)
            .with_flow(AnimationFlow::Static)
            .with_frames([2]),
    );
    let player_land = jump
        .clone()
        .with_key(animations::PLAYER_LAND)
        .with_flow(AnimationFlow::Once)
        .with_frames([4, 5])
        .with_frame_duration(0.05);
    animations.add(
        player_land
            .clone()
            .with_frames(player_land.frames.iter().rev().map(|n| *n))
            .with_key(animations::PLAYER_CROUCH)
            .with_frame_duration(0.01),
    );
    animations.add(player_land.clone());
}

fn setup_idle_animation(animations: &mut Animations, builder: &mut AnimationBuilder) {
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
}

fn setup_run_animation(animations: &mut Animations, builder: &mut AnimationBuilder) {
    animations.add(
        builder
            .from_grid(
                animations::PLAYER_RUN,
                "objects/player/player_run_16x16_6_1_0x0_1x0.png",
                Vec2::new(16., 16.),
                6,
                1,
                Some(Vec2::new(1., 0.)),
                None,
            )
            .with_flow(AnimationFlow::Looping)
            .with_frame_duration(0.1)
            .with_frames([0, 1, 2, 3, 4, 5]),
    );
}

pub(crate) fn sys_setup_player_animations(
    mut animations: ResMut<Animations>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let mut builder = AnimationBuilder::new(&asset_server, &mut texture_atlases);
    setup_idle_animation(&mut animations, &mut builder);
    setup_run_animation(&mut animations, &mut builder);
    setup_jump_animations(&mut animations, &mut builder);
}

/// Control the playing animation for the player
pub(crate) fn sys_player_animation(
    key_in: Res<Input<KeyCode>>,
    animations: Res<Animations>,
    q_player: Query<(&Children, &KinematicVelocity, &KinematicObject), With<Player>>,
    mut q_player_decoration: Query<(&mut Animation, &AnimationState, &mut Transform)>,
) {
    let (children, vel, k_object) = q_player.single();
    let (animation, animation_state, mut trf) = q_player_decoration
        .get_mut(*children.first().unwrap())
        .unwrap();

    if vel.linvel.x.abs() > KinematicObject::PADDING_WIDTH {
        if vel.linvel.x.is_sign_positive() {
            trf.scale.x = 1.;
        } else {
            trf.scale.x = -1.;
        }
    }

    if k_object.touching_floor() {
        if [
            animations::PLAYER_FALL,
            animations::PLAYER_FLOAT,
            animations::PLAYER_JUMP,
        ]
        .contains(&animation.key)
            || key_in.just_released(KeyCode::Down)
        {
            animations.bind_if_different(animation, &animations::PLAYER_LAND);
        } else if key_in.pressed(KeyCode::Down) {
            animations.bind_if_different(animation, &animations::PLAYER_CROUCH);
        } else if key_in.just_pressed(KeyCode::Up) {
            animations.bind_if_different(animation, &animations::PLAYER_JUMP);
        } else if animation_state.finished_or_unfinishable(&animation.flow) {
            if vel.linvel.x.abs() > KinematicObject::PADDING_WIDTH * 2. {
                animations.bind_if_different(animation, &animations::PLAYER_RUN);
            } else {
                animations.bind_if_different(animation, &animations::PLAYER_IDLE);
            }
        }
    } else {
        if animation_state.finished_or_unfinishable(&animation.flow) {
            if vel.linvel.y.abs() < 100. {
                animations.bind_if_different(animation, &animations::PLAYER_FLOAT);
            } else {
                animations.bind_if_different(animation, &animations::PLAYER_FALL);
            }
        }
    }
}

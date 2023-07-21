#![allow(unused)]
use bevy::{prelude::*, utils::HashMap};
use std::sync::Arc;

pub type AnimationKey = usize;

/// Represents an animation (e.g. "jump" or "walk")
#[derive(Component, Clone, Debug)]
pub struct Animation {
    /// A unique identifier for this animation
    pub key: AnimationKey,
    /// The texture atlas from which sprites are acquired (strong handle)
    pub atlas: Handle<TextureAtlas>,
    /// The frames of the animation
    pub frames: Arc<[AnimationKey]>,
    /// How the animation should be played
    pub flow: AnimationFlow,
    /// The duration between frames
    pub frame_duration: f32,
}

impl Animation {
    pub fn with_key(mut self, key: AnimationKey) -> Self {
        self.key = key;
        self
    }

    pub fn with_frames<I: IntoIterator<Item = AnimationKey>>(mut self, frames: I) -> Animation {
        self.frames = frames.into_iter().collect();
        self
    }

    pub fn with_flow(mut self, flow: AnimationFlow) -> Self {
        self.flow = flow;
        self
    }

    pub fn with_frame_duration(mut self, frame_duration: f32) -> Self {
        self.frame_duration = frame_duration;
        self
    }
}

impl Default for Animation {
    fn default() -> Self {
        Self {
            key: 0,
            atlas: Handle::<TextureAtlas>::default(),
            frames: Arc::new([]),
            flow: AnimationFlow::Looping,
            frame_duration: 0.1,
        }
    }
}

pub struct AnimationBuilder<'a> {
    asset_server: &'a AssetServer,
    texture_atlases: &'a mut Assets<TextureAtlas>,
}

impl<'a> AnimationBuilder<'a> {
    pub fn new(
        asset_server: &'a AssetServer,
        texture_atlases: &'a mut Assets<TextureAtlas>,
    ) -> Self {
        Self {
            asset_server,
            texture_atlases,
        }
    }

    pub fn from_grid(
        &mut self,
        key: AnimationKey,
        path: &'static str,
        tile_size: Vec2,
        columns: usize,
        rows: usize,
        padding: Option<Vec2>,
        offset: Option<Vec2>,
    ) -> Animation {
        let atlas = TextureAtlas::from_grid(
            self.asset_server.load(path),
            tile_size,
            columns,
            rows,
            padding,
            offset,
        );

        Animation {
            key,
            atlas: self.texture_atlases.add(atlas),
            frames: (0..(columns * rows)).collect(),
            flow: AnimationFlow::Static,
            frame_duration: 0.,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AnimationFlow {
    Static,
    Once,
    Looping,
}

#[derive(Component, Clone, Debug)]
pub struct AnimationState {
    timer: Timer,
    index: usize,
    /// ```
    /// [0] just started
    /// [1] reserved
    /// [2] finished
    /// [3..] undefined
    /// ```
    flags: u8,
}

impl Default for AnimationState {
    fn default() -> Self {
        Self {
            timer: Timer::default(),
            flags: 0b1000_0000, // just started
            index: 0,
        }
    }
}

impl AnimationState {
    pub fn just_started(&self) -> bool {
        (self.flags & 0b1000_0000) > 0
    }

    fn set_just_started(&mut self, val: bool) {
        self.flags = (self.flags & 0b0111_1111) | ((val as u8) << 7);
    }

    pub fn finished(&self) -> bool {
        (self.flags & 0b0010_0000) > 0
    }

    /// `AnimationFlow::Static` and `AnimationFlow::Looping` cannot actually 'finish', so we return
    /// `true` for these flows as well
    pub fn finished_or_unfinishable(&self, animation_flow: &AnimationFlow) -> bool {
        self.finished()
            || *animation_flow == AnimationFlow::Looping
            || *animation_flow == AnimationFlow::Static
    }

    fn set_finished(&mut self, val: bool) {
        self.flags = (self.flags & 0b1101_1111) | ((val as u8) << 5);
    }

    fn reset(&mut self, animation: &Animation) {
        self.timer = Timer::from_seconds(animation.frame_duration, TimerMode::Repeating);
        self.set_just_started(true);
        self.set_finished(false);
        self.index = 0;
    }
}

/// Keeps track of all animations -- use constants to access named keys e.g.
///
/// ```
/// const PLAYER_RUN: usize = 0;
/// animations.get(PLAYER_RUN).unwrap();
/// ```
#[derive(Resource)]
pub struct Animations(HashMap<AnimationKey, Animation>);

impl Animations {
    pub fn add(&mut self, animation: Animation) {
        self.0.insert(animation.key, animation);
    }

    pub fn get(&self, label: &AnimationKey) -> Option<&Animation> {
        self.0.get(label)
    }

    /// Sets a given `Animation` to the animation with the key `key` (if it exists), otherwise does
    /// nothing.
    pub fn bind(&self, animation: &mut Animation, key: &AnimationKey) {
        if let Some(new_animation) = self.get(key) {
            *animation = new_animation.clone();
        }
    }

    /// perform a `bind` if the `key` is different from the current animation
    pub fn bind_if_different<'a>(&self, mut animation: Mut<'a, Animation>, key: &AnimationKey) {
        if animation.key != *key {
            self.bind(&mut animation, key);
        }
    }
}

#[derive(Bundle, Default)]
pub struct AnimatedSpriteSheetBundle {
    pub sprite: SpriteSheetBundle,
    pub animation: Animation,
    pub animation_state: AnimationState,
}

/// Whenever somebody binds a new `Animation`, we want to reset that animation's state.
fn sys_reset_changed_animations(
    mut query: Query<
        (
            &Animation,
            &mut AnimationState,
            &mut TextureAtlasSprite,
            &mut Handle<TextureAtlas>,
        ),
        Changed<Animation>,
    >,
) {
    for (animation, mut animation_state, mut atlas_sprite, mut atlas) in query.iter_mut() {
        animation_state.reset(&animation);
        atlas_sprite.index = animation.frames[0];
        *atlas = animation.atlas.clone();
    }
}

/// Update the state of each animation, then set the index of the atlas accordingly
fn sys_update_animations(
    mut query: Query<(&Animation, &mut AnimationState, &mut TextureAtlasSprite)>,
    time: Res<Time>,
) {
    for (animation, mut animation_state, mut atlas) in query.iter_mut() {
        if animation_state.just_started() {
            animation_state.set_just_started(false);
        } else {
            if animation.flow == AnimationFlow::Static {
                continue;
            }

            if animation_state.timer.tick(time.delta()).just_finished() {
                match animation.flow {
                    AnimationFlow::Looping => {
                        if animation_state.index == animation.frames.len() - 1 {
                            animation_state.index = 0;
                        } else {
                            animation_state.index += 1;
                        }
                    }
                    AnimationFlow::Once => {
                        if animation_state.index < animation.frames.len() - 1 {
                            animation_state.index += 1;
                        } else {
                            animation_state.set_finished(true);
                        }
                    }
                    AnimationFlow::Static => {
                        unreachable!("should have handled static flow earlier")
                    }
                }
            }

            atlas.index = animation.frames[animation_state.index];
        }
    }
}

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Animations(HashMap::new())).add_systems(
            PostUpdate,
            (
                sys_reset_changed_animations,
                sys_update_animations.after(sys_reset_changed_animations),
            ),
        );
    }
}

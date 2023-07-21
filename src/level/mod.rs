use std::path::Path;

use crate::types::*;
use bevy::{prelude::*, utils::HashMap};
use bevy_rapier2d::prelude::*;

/// The size of a chunk in pixels (the image used for a chunk should be exactly
/// (CHUNK_SIZE_PX x CHUNK_SIZE_PX))
pub const CHUNK_SIZE_PX: usize = 256;

/// Levels should be stored in the special folder assets/levels/
/// They should be stored in the folder with their respective identifier, e.g.
/// assets/levels/grassland/
/// Chunks of the level should be a fixed size of `CHUNK_SIZE_PX` in both
/// dimensions...
#[derive(Default)]
pub struct LevelDescriptor {
    pub ident: &'static str,
    pub hitboxes: HashMap<ChunkLocation, Array<LevelHitboxDescriptor>>,
}

impl LevelDescriptor {
    /// (rust, bevy) style return type
    pub fn create_path_string(&self, x: i32, y: i32, fg_bg: &'static str) -> (String, String) {
        if fg_bg != "fg" && fg_bg != "bg" {
            panic!("fg_bg must be one of \"fg\" or \"bg\"");
        }

        let asset_path = format!(
            "levels/{ident}/{ident}_{x}_{y}_{fg_bg}.png",
            ident = self.ident
        );
        let path = format!("./assets/{}", asset_path);

        (path, asset_path)
    }
}

#[derive(Component)]
pub struct LevelIdent(&'static str);

#[derive(Default, Clone)]
pub struct LoadedChunk {
    /// Guaranteed to be a `ChunkLayerBundle`
    pub background: Option<Entity>,
    /// Guaranteed to be a `ChunkLayerBundle`
    pub foreground: Option<Entity>,
}

#[derive(Component)]
pub struct LevelChunks(Array<Array<LoadedChunk>>);

#[derive(Bundle)]
pub struct LevelBundle {
    pub name: Name,
    pub ident: LevelIdent,
    pub chunks: LevelChunks,
}

#[derive(Bundle, Default)]
pub struct ChunkLayerBundle {
    pub sprite_bundle: SpriteBundle,
}

impl LevelDescriptor {
    fn spawn_chunk_layer(
        &self,
        asset_server: &AssetServer,
        commands: &mut Commands,
        x: i32,
        y: i32,
        z: f32,
        fg_bg: &'static str,
    ) -> Option<Entity> {
        let (path_str, asset_path_str) = self.create_path_string(x, y, fg_bg);
        let path = Path::new(&path_str);
        if path.exists() {
            let bundle = ChunkLayerBundle {
                sprite_bundle: SpriteBundle {
                    texture: asset_server.load(asset_path_str),
                    transform: Transform::from_xyz(
                        x as f32 * CHUNK_SIZE_PX as f32,
                        y as f32 * CHUNK_SIZE_PX as f32,
                        z,
                    ),
                    ..default()
                },
            };
            Some(commands.spawn(bundle).id())
        } else {
            None
        }
    }

    pub fn spawn(&self, asset_server: &AssetServer, commands: &mut Commands) {
        // for now, let's just spawn a 5x5 region, centered at (0, 0)
        // TODO: make this not use vecs (not a big deal though)
        let mut level_chunks = Array::from(vec![Array::from(vec![LoadedChunk::default(); 3]); 3]);

        for (i, x) in (-1..=1).enumerate() {
            for (j, y) in (-1..=1).rev().enumerate() {
                let chunk = &mut level_chunks[j][i];
                chunk.foreground = self.spawn_chunk_layer(asset_server, commands, x, y, 0.5, "fg");
                chunk.background = self.spawn_chunk_layer(asset_server, commands, x, y, -0.5, "bg");
                if let (Some(id), Some(hitbox_descriptors)) = (
                    chunk.background,
                    self.hitboxes.get(&ChunkLocation::new(x, y)),
                ) {
                    let children = hitbox_descriptors
                        .into_iter()
                        .map(|desc| desc.spawn(commands))
                        .collect::<Box<[Entity]>>();
                    let mut ec = commands.get_entity(id).unwrap();
                    ec.push_children(&children);
                }
            }
        }

        commands.spawn(LevelBundle {
            name: Name::new(self.ident),
            ident: LevelIdent(self.ident),
            chunks: LevelChunks(level_chunks),
        });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct ChunkLocation {
    x: i32,
    y: i32,
}

impl ChunkLocation {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[non_exhaustive]
pub enum LevelHitboxDescriptor {
    /// Describes where the aabb should be aligned based on *aseprite pixel offsets*. So y
    /// coordinates are measured from the top of the canvas. Hitboxes encapsulate all pixels that
    /// describe them. so two hitboxes stretching from (0, 5) and (5, 10) would have an overlap 1
    /// pixel wide at the 6th (index 5) pixel.
    ///
    /// Aabbs are SOLID. That means nothing can get inside them. It does not mean nothing can get
    /// out of them. No guarantees are made about interactions with aabbs from the inside.
    /// Therefore, a (nether portal style) donut shape would require 4 aabbs, one for each wall,
    /// not just an inner ring and outer ring.
    Aabb {
        top: usize,
        right: usize,
        bottom: usize,
        left: usize,
    },
}

impl LevelHitboxDescriptor {
    pub fn aabb(top: usize, right: usize, bottom: usize, left: usize) -> Self {
        if top > bottom {
            panic!("cannot create upside-down Aabb (top must be <= bottom)");
        }
        if left > right {
            panic!("cannot create flipped Aabb (left must be <= right)")
        }

        Self::Aabb {
            top,
            right: right + 1,
            bottom: bottom + 1,
            left,
        }
    }

    fn into_collider_with_transform(&self) -> (Collider, TransformBundle) {
        match self {
            Self::Aabb {
                top,
                right,
                bottom,
                left,
            } => {
                let (top, right, bottom, left) =
                    (*top as f32, *right as f32, *bottom as f32, *left as f32);
                let half_width = (right - left) / 2.;
                let half_height = (bottom - top) / 2.;
                let mut center_x = left + half_width;
                let mut center_y = top + half_height;
                // coordinates are (0, 0) at bl of image
                center_y = CHUNK_SIZE_PX as f32 - center_y;
                // and (0, 0) should be -chunk_size / 2.
                center_x -= CHUNK_SIZE_PX as f32 / 2.;
                center_y -= CHUNK_SIZE_PX as f32 / 2.;

                (
                    Collider::cuboid(half_width, half_height),
                    TransformBundle::from(Transform::from_xyz(center_x, center_y, 0.)),
                )
            }
        }
    }

    fn spawn(&self, commands: &mut Commands) -> Entity {
        match self {
            Self::Aabb { .. } => commands
                .spawn((self.into_collider_with_transform(), RigidBody::Fixed))
                .id(),
        }
    }
}

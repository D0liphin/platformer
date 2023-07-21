pub mod player;
use std::{f32::consts::FRAC_PI_2, fmt};

use crate::debug::gizmos_ext::GizmosExt;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct KinematicObject {
    /// [0]: touching the ground
    #[allow(unused)]
    flags: u64,
}

impl KinematicObject {
    fn new() -> Self {
        Self { flags: 0 }
    }

    pub const PADDING_WIDTH: f32 = 1.;

    // pub const TOUCHING_GROUND: u64 = 0b1;

    // pub fn is_touching_ground(&self) -> bool {
    //     self.flags.is_high(Self::TOUCHING_GROUND)
    // }
}

#[derive(Clone, Copy)]
struct GetCollisionCorrectionArgs<'a> {
    rapier_ctx: &'a RapierContext,
    shape_pos: Vec2,
    shape_rot: f32,
    shape_linvel: Vec2,
    shape: &'a Collider,
    max_toi: f32,
    self_id: Entity,
    padding: f32,
}

impl fmt::Debug for GetCollisionCorrectionArgs<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        #[allow(unused)]
        #[derive(Debug)]
        struct DebugArgs {
            shape_pos: Vec2,
            shape_linvel: Vec2,
            max_toi: f32,
            padding: f32,
        }

        let Self {
            shape_pos,
            shape_linvel,
            max_toi,
            padding,
            ..
        } = self.clone();
        write!(
            f,
            "{:#?}",
            DebugArgs {
                shape_pos,
                shape_linvel,
                max_toi,
                padding
            }
        )
    }
}

#[derive(Clone, Debug)]
struct CollisionCorrection {
    /// The rapier collision
    hit: Toi,
    /// The exact location of the collision
    #[allow(unused)]
    hit_pos: Vec2,
    /// The location of the collision, padded according to the normal of the collision. You should
    /// move the object here, as floating point imprecision causes us to phase through blocks if
    /// we don't do this.
    hit_pos_with_padding: Vec2,
    /// The new linear velocity after the collision takes place. Note that this assumes the surface
    /// is completely slippery. So a vel [10, -10], colliding with a floor parallel to the x-axis
    /// will produce a new_linvel of [10, 0] (roughly)
    new_linvel: Vec2,
}

/// Given a movement, defined as moving `shape` (at position `shape_pos`, rotation `shape_rot`, with
/// id `self_id`) for `max_toi` seconds, with linear velocity `shape_linvel`, if there is a
/// collision at some point along this movement, return information about how to correct that
/// collision.
fn get_collision_correction(args: GetCollisionCorrectionArgs) -> Option<CollisionCorrection> {
    let GetCollisionCorrectionArgs {
        rapier_ctx,
        shape_pos,
        shape_rot,
        shape_linvel,
        shape,
        max_toi,
        self_id,
        padding,
    } = args;

    if let Some((_, hit)) = rapier_ctx.cast_shape(
        shape_pos,
        shape_rot,
        shape_linvel,
        shape,
        max_toi,
        QueryFilter::new().predicate(&|e| e != self_id), // ignore ourself
    ) {
        // witness1 is the hitbox that we collide with.
        // fire a ray from the point of collision to the shape to find the delta required to
        // have the shape's edge intersect with the collision point. We can't just use witness1
        // because two surfaces with the same normal (or opposite) colliding will produce
        // indeterminate collision points across that edge.
        let ray_origin = hit.witness1;
        let ray_dir = -shape_linvel.normalize();
        let ray_toi = shape.cast_ray(
            shape_pos,
            shape_rot,
            ray_origin,
            ray_dir,
            f32::INFINITY, // it's guaranteed to hit, so this just makes it simpler for us
            true,          // we always want this to be on the edge of the collider
        );
        if let Some(ray_toi) = ray_toi {
            // The perfect location would just be ray_toi * -ray_dir + shape_pos, but we want
            // to back-shift by a pixel so we don't then phase through the wall on the next
            // iteration
            let hit_pos = ray_toi * -ray_dir + shape_pos;
            // TODO: back-shift this along the velocity vector instead (maybe only if this results
            // in a penetrating collision)
            let hit_pos_with_padding = hit_pos + hit.normal1 * padding;

            // Just getting the components of the velocity that are not affected. TODO
            let new_linvel = shape_linvel.project_onto(Mat2::from_angle(FRAC_PI_2) * hit.normal1);

            return Some(CollisionCorrection {
                hit,
                hit_pos,
                hit_pos_with_padding,
                new_linvel,
            });
        } else {
            println!("\n\n\nPENETRATING\n\n\n");
            // TOIStatus::Penetrating
        }
    }
    None
}

/// Iterate multiple `get_collision_correction` passes, applying residual velocity each time. Does
/// not return a `CollisionCorrection` as it uses the hit_pos_with_padding internally and involves
/// several passes (up to `max_correction_iters`)
///
/// Returns `None` if there is no collision
fn get_collision_correction_multipass(
    mut args: GetCollisionCorrectionArgs,
    max_correction_iters: usize,
) -> Option<Vec2> {
    let mut final_pos: Option<Vec2> = None;

    for i in 1..=max_correction_iters {
        let is_final_pass = i == max_correction_iters;

        let correction = get_collision_correction(args);
        if let Some(mut correction) = correction {
            if is_final_pass {
                correction.new_linvel = Vec2::ZERO;
                final_pos = Some(correction.hit_pos_with_padding);
            } else {
                final_pos = Some(correction.hit_pos_with_padding);
                args.shape_pos = correction.hit_pos_with_padding;
                args.shape_linvel = correction.new_linvel;
                args.max_toi -= correction.hit.toi;
            }
        } else {
            // No collisions! We're all good :)
            // but we still need to update the final_pos with any lingering velocity
            if let Some(mut pos) = final_pos {
                pos += args.shape_linvel * args.max_toi;
            }
            break;
        }
    }

    final_pos
}

/// KinematicVelocityBased objects *want* to move through walls, but we can't let them! This system
/// updates their positions.
fn sys_adjust_objects(
    mut gizmos: Gizmos,
    rapier_ctx: Res<RapierContext>,
    time: Res<Time>,
    mut q_kinematic_objects: Query<(
        Entity,
        &mut KinematicObject,
        &mut Transform,
        &GlobalTransform,
        &mut Velocity,
        &Collider,
    )>,
) {
    for (self_id, mut _k_object, mut trf, global_trf, mut shape_vel, shape) in
        q_kinematic_objects.iter_mut()
    {
        if shape_vel.linvel.length() < 0.01 {
            shape_vel.linvel = Vec2::ZERO;
            continue;
        }

        let shape_pos = Vec2::new(global_trf.translation().x, global_trf.translation().y);
        let shape_linvel = shape_vel.linvel;
        let shape_rot = Quat::from_affine3(&global_trf.affine())
            .to_euler(EulerRot::XYZ)
            .2; // this is the z-coordinate of the euler-rot which is all that matters for 2D
                // velocity of the KinematicObject
        gizmos.arrow_2d(shape_pos, shape_pos + shape_linvel, 4., Color::BLUE);

        if let Some(pos) = get_collision_correction_multipass(
            GetCollisionCorrectionArgs {
                rapier_ctx: &rapier_ctx,
                shape_pos,
                shape_rot,
                shape_linvel,
                shape,
                max_toi: time.delta_seconds(),
                self_id,
                padding: KinematicObject::PADDING_WIDTH,
            },
            5,
        ) {
            trf.translation.x = pos.x;
            trf.translation.y = pos.y;
        } else {
            let delta = shape_vel.linvel * time.delta_seconds();
            trf.translation.x += delta.x;
            trf.translation.y += delta.y;
        }
    }
}

/// Manages universal object behaviour
pub struct KinematicObjectPlugin;

impl Plugin for KinematicObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sys_adjust_objects);
    }
}

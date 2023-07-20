pub mod player;
use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier2d::{parry::bounding_volume::Aabb, prelude::*};

use crate::{bitflags::BitFlags, debug::gizmos_ext::GizmosExt};

use self::player::{controller::sys_print_player_trf, Player};

#[derive(Component)]
pub struct KinematicObject {
    /// [0]: touching the ground
    flags: u64,
}

impl KinematicObject {
    fn new() -> Self {
        Self { flags: 0 }
    }

    pub const PADDING_WIDTH: f32 = 0.01;

    pub const TOUCHING_GROUND: u64 = 0b1;

    pub fn is_touching_ground(&self) -> bool {
        self.flags.is_high(Self::TOUCHING_GROUND)
    }
}

/// KinematicVelocityBased objects *want* to move through walls, but we can't let them! This system
/// updates their positions.
fn sys_adjust_objects(
    mut gizmos: Gizmos,
    rapier_ctx: Res<RapierContext>,
    time: Res<Time>,
    q_ignore: Query<(), With<Player>>,
    mut q_kinematic_objects: Query<(
        &mut KinematicObject,
        &mut Transform,
        &GlobalTransform,
        &mut Velocity,
        &Collider,
    )>,
) {
    for (mut k_object, mut trf, global_trf, mut shape_vel, shape) in q_kinematic_objects.iter_mut()
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

        // cast a shape within the next frame
        if let Some((_, hit)) = rapier_ctx.cast_shape(
            shape_pos,
            shape_rot,
            shape_linvel,
            shape,
            time.delta_seconds(),
            // 1.,
            QueryFilter::new().predicate(&|e| q_ignore.get(e).is_err()), // ignore ourself
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
                let hit_pos_with_padding = hit_pos + hit.normal1 * KinematicObject::PADDING_WIDTH;

                let rect_size = shape.as_cuboid().unwrap().half_extents() * 2.;
                gizmos.rect_2d(hit_pos, shape_rot, rect_size, Color::RED);
                gizmos.rect_2d(hit_pos_with_padding, shape_rot, rect_size, Color::BLUE);

                // Just getting the components of the velocity that are not affected. TODO
                let new_linvel =
                    shape_linvel.project_onto(Mat2::from_angle(FRAC_PI_2) * hit.normal1);
                gizmos.arrow_2d(
                    hit_pos_with_padding,
                    hit_pos_with_padding + new_linvel,
                    2.,
                    Color::RED,
                );

                trf.translation.x = hit_pos_with_padding.x;
                trf.translation.y = hit_pos_with_padding.y;
                shape_vel.linvel = new_linvel;

                // We 'teleport' to the point of collision, and maintain *all* velocity not 
                // directly opposed to the collision normal... Obviously this causes a whole host of
                // problems. Ignoring how this will result in an incorrect simulation. We also have
                // to consider the case where we are moving diagonally into a corner. We can do 
                // one additional pass on the resultant velocity to make sure that there is no
                // collision, not propogating remaining velocity componnents
            }
        }
        let delta = shape_vel.linvel * time.delta_seconds();
        trf.translation.x += delta.x;
        trf.translation.y += delta.y;
    }
}

/// Manages universal object behaviour
pub struct KinematicObjectPlugin;

// use std::time::Instant;

// #[derive(Resource)]
// struct LastFixedUpdateInstant(Instant);

// fn sys_println(mut last_fu: ResMut<LastFixedUpdateInstant>, time: Res<Time>) {
//     let now = Instant::now();
//     println!(
//         "{:?}, time.delta_seconds() = {}",
//         now.duration_since(last_fu.0),
//         time.delta_seconds()
//     );
//     last_fu.0 = now;
// }

impl Plugin for KinematicObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sys_adjust_objects);
        // .insert_resource(LastFixedUpdateInstant(Instant::now()))
        // .add_systems(FixedUpdate, sys_println);
        // .add_systems(PreUpdate, sys_print_player_trf::<PreUpdate>())
        // .add_systems(Update, sys_print_player_trf::<Update>())
        // .add_systems(PostUpdate, sys_print_player_trf::<PostUpdate>())
        // .add_systems(Last, sys_print_player_trf::<Last>());
    }
}

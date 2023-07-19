pub mod player;
use bevy::prelude::*;
use bevy_rapier2d::{parry::bounding_volume::Aabb, prelude::*};

use crate::bitflags::BitFlags;

use self::player::Player;

#[derive(Component)]
pub struct KinematicObject {
    /// [0]: touching the ground
    flags: u64,
}

impl KinematicObject {
    fn new() -> Self {
        Self { flags: 0 }
    }

    pub const TOUCHING_GROUND: u64 = 0b1;

    pub fn is_touching_ground(&self) -> bool {
        self.flags.is_high(Self::TOUCHING_GROUND)
    }
}

#[derive(Component, Default)]
pub struct DeathNote {
    kill_me: bool,
}

fn sys_fulfill_death_notes(
    mut commands: Commands,
    mut q_death_notes: Query<(&mut DeathNote, Entity)>,
) {
    for (mut death_note, entity) in q_death_notes.iter_mut() {
        if death_note.kill_me {
            commands.entity(entity).despawn();
        } else {
            death_note.kill_me = true;
        }
    }
}

/// KinematicVelocityBased objects *want* to move through walls, but we can't let them! This system
/// updates their positions.
fn sys_adjust_objects(
    mut commands: Commands,
    rapier_ctx: Res<RapierContext>,
    time: Res<Time>,
    q_witness: Query<(&Collider, &GlobalTransform), With<RigidBody>>,
    q_ignore: Query<(), Or<(With<Player>, With<DeathNote>)>>,
    mut q_kinematic_objects: Query<
        (&mut Transform, &GlobalTransform, &mut Velocity, &Collider),
        (With<KinematicObject>, With<Player>),
    >,
) {
    for (mut trf, global_trf, mut shape_vel, shape) in q_kinematic_objects.iter_mut() {
        let shape_pos = Vec2::new(global_trf.translation().x, global_trf.translation().y);
        if let Some((e, hit)) = rapier_ctx.cast_shape(
            shape_pos,
            0.,
            Vec2::new(-32., -64.),
            shape,
            1.,
            QueryFilter::new().predicate(&|e| q_ignore.get(e).is_err()),
        ) {
            // witness1 is the hitbox that we collide with.
            if let Ok((w1_collider, w1_trf)) = q_witness.get(e) {
                // ok so what we need to do here is cast a ray backwards from witness1 to the shape
                // in the direction of the normal. Then the point where it collides is the point on
                // the object that collides. Then we can just shift the shape by that much.

                // commands.spawn((
                //     shape.clone(),
                //     DeathNote::default(),
                //     TransformBundle::from(Transform::from_xyz(hit.witness1.x, hit.witness1.y, 0.)),
                // ));
            }
            // dbg!(hit);
            // trf.translation.y += shape_vel.linvel.y * time.delta_seconds();
            // trf.translation.y += hit.normal1.y;
            // shape_vel.linvel.y = 0.;
        }
    }
}

/// Manages universal object behaviour
pub struct KinematicObjectPlugin;

impl Plugin for KinematicObjectPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, sys_adjust_objects)
            .add_systems(PostUpdate, sys_fulfill_death_notes);
    }
}

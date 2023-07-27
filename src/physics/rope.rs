use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;

use crate::real_cursor_pos::RealCursorPosition;

// #[derive(Clone, Copy, PartialEq)]
// pub enum RopeSegmentTy {
//     Fixed,
//     Dynamic,
// }

// #[derive(Clone, Copy)]
// pub struct RopeSegment {
//     pos: Vec2,
//     vel: Vec2,
//     ty: RopeSegmentTy,
// }

// impl RopeSegment {
//     pub fn fixed(pos: Vec2) -> Self {
//         Self {
//             pos,
//             vel: Vec2::ZERO,
//             ty: RopeSegmentTy::Fixed,
//         }
//     }

//     pub fn dynamic(pos: Vec2) -> Self {
//         Self {
//             pos,
//             vel: Vec2::ZERO,
//             ty: RopeSegmentTy::Dynamic,
//         }
//     }
// }

#[derive(Clone, Component)]
pub struct Catenary {
    anchor_1: Vec2,
    anchor_2: Vec2,
}

#[derive(Resource, Default)]
pub struct DraggingRopeSegment(Option<(Entity, usize)>);

fn sys_draw_ropes(
    mut dragging_rope_segment: ResMut<DraggingRopeSegment>,
    mouse_in: Res<Input<MouseButton>>,
    mut gizmos: Gizmos,
    cursor_pos: Res<RealCursorPosition>,
    mut q_ropes: Query<(Entity, &mut Catenary)>,
) {
    for (entity, rope) in q_ropes.iter_mut() {
        for (i, pos) in [rope.anchor_1, rope.anchor_2].into_iter().enumerate() {
            let radius = 2.;
            let mut color = Color::BLUE;
            if let Some(cursor_pos) = cursor_pos.get() {
                if pos.distance(cursor_pos) < radius {
                    color = Color::RED;
                    if mouse_in.just_pressed(MouseButton::Left) {
                        dragging_rope_segment.0 = Some((entity, i));
                    }
                }
            }
            gizmos.circle_2d(pos, radius, color);
        }
        let mut line_strip = vec![];
        let segments = 100;
        for i in 0..segments {

        }
        gizmos.linestrip_2d(line_strip, Color::BLUE);

        // for (i, segment) in rope.segments.iter().enumerate() {
        //     if segment.ty == RopeSegmentTy::Fixed {

        //     }
        // }
    }

    if mouse_in.pressed(MouseButton::Left) {
        if let Some((entity, i)) = dragging_rope_segment.0 {
            let (_, mut rope) = q_ropes.get_mut(entity).unwrap();
            if let Some(cursor_pos) = cursor_pos.get() {
                *if i == 0 {
                    &mut rope.anchor_1
                } else {
                    &mut rope.anchor_2
                } = cursor_pos;
            }
        }
    } else {
        dragging_rope_segment.0 = None;
    }
}

fn tension(fixed: Vec2, displaced: Vec2, spring_constant: f32, spring_length: f32) -> Vec2 {
    let force = displaced - fixed;
    let direction = force.normalize();
    let displacement = force.length() - spring_length;
    -direction * displacement * spring_constant
}

struct Line {
    position: Vec2,
    direction: Vec2,
}

impl Line {
    fn new(position: Vec2, direction: Vec2) -> Self {
        Self {
            position,
            direction,
        }
    }

    fn intersects(&self, other: &Line) -> Option<Vec2> {
        let a = (other.direction.y * (other.position.x - self.position.x)
            + self.position.y * other.direction.x
            - other.position.y * other.direction.x)
            / (self.direction.x * other.direction.y - self.direction.y * other.direction.x);
        if a.is_nan() {
            None
        } else {
            Some(self.position + a * self.direction)
        }
    }
}

// fn equalize_tension(rope_segment: &mut RopeSegment, anchor_1: Vec2, anchor_2: Vec2) {
//     // does nothing on purpose (for now)
//     let d_1 = rope_segment.pos.distance(anchor_1);
//     let d_2 = rope_segment.pos.distance(anchor_2);
//     let (start, end) = if d_1 > d_2 {
//         (anchor_2, anchor_1)
//     } else {
//         (anchor_1, anchor_2)
//     };

//     let p = (end - start) * 0.5 + start;
//     let u = Mat2::from_angle(FRAC_PI_2) * (end - start);

//     let q = rope_segment.pos;
//     let v = end - start;

//     if let Some(intersection) = Line::new(p, u).intersects(&Line::new(q, v)) {
//         rope_segment.pos = intersection;
//     }
// }

// fn update_velocity(rope_segment: RopeSegment, )

fn sys_update_ropes(time: Res<Time>, mut q_ropes: Query<&mut Catenary>) {
    for mut rope in q_ropes.iter_mut() {}
}

pub struct RopePlugin;

impl Plugin for RopePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut commands: Commands| {
            let count = 100;
            commands.spawn(Catenary {
                anchor_1: Vec2::new(50., 50.),
                anchor_2: Vec2::new(150., 50.),
            });
        })
        .insert_resource(DraggingRopeSegment::default())
        .add_systems(Update, sys_update_ropes)
        .add_systems(PostUpdate, sys_draw_ropes);
    }
}

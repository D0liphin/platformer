//! Adds the ability to drag colliders around!
use bevy::prelude::*;
use bevy_egui::egui::Ui;
use bevy_rapier2d::{prelude::*, rapier::prelude::TypedShape};

use crate::real_cursor_pos::RealCursorPosition;

use super::affine_ext::AffineExt;

#[derive(Resource)]
pub struct Dragging {
    entity: Option<Entity>,
}

#[derive(Resource)]
pub struct DraggableCollidersUiState {
    enabled: bool,
}

pub fn show_draggable_colliders_ui(ui: &mut Ui, ui_state: &mut DraggableCollidersUiState) {
    ui.heading("Draggable Colliders");
    ui.checkbox(&mut ui_state.enabled, "enabled");
    ui.separator();
}

pub fn sys_draggable_colliders(
    mut gizmos: Gizmos,
    mut dragging: ResMut<Dragging>,
    mouse_in: Res<Input<MouseButton>>,
    key_in: Res<Input<KeyCode>>,
    cursor_pos: Res<RealCursorPosition>,
    rapier_ctx: Res<RapierContext>,
    mut q_collider: Query<(&Collider, &mut Transform)>,
) {
    let Some(cursor_pos) = cursor_pos.get() else {
        return;
    };
    if let Some(entity) = dragging.entity {
        if key_in.pressed(KeyCode::ControlLeft) && mouse_in.pressed(MouseButton::Left) {
            let (_, mut trf) = q_collider
                .get_mut(entity)
                .expect("should match query as created from query");
            trf.translation.x = cursor_pos.x;
            trf.translation.y = cursor_pos.y;
        } else {
            dragging.entity = None;
        }
        return;
    }

    if key_in.pressed(KeyCode::ControlLeft) {
        if let Some((e, _)) = rapier_ctx.project_point(
            cursor_pos,
            true,
            QueryFilter::default().predicate(&|e| q_collider.get(e).is_ok()),
        ) {
            let (collider, trf) = q_collider.get(e).unwrap();
            // mouse_in.just_pressed(MouseButton::Left)
            match collider.raw.as_typed_shape() {
                TypedShape::Cuboid(cuboid) => gizmos.rect_2d(
                    Vec2::new(trf.translation.x, trf.translation.y),
                    trf.compute_affine().rot_2d(),
                    Vec2::from(cuboid.half_extents) * 2.,
                    Color::ORANGE,
                ),
                _ => unimplemented!(),
            }

            if mouse_in.just_pressed(MouseButton::Left) {
                dragging.entity = Some(e);
            }
        }
    }
}

pub struct DraggableCollidersPlugin;

impl Plugin for DraggableCollidersPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Dragging { entity: None })
            .insert_resource(DraggableCollidersUiState { enabled: true })
            .add_systems(Update, sys_draggable_colliders);
    }
}

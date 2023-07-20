use bevy::prelude::*;

pub fn get_real_cursor_position(
    window: &Window,
    camera_orth_proj: &OrthographicProjection,
    camera_translation: &Vec3,
) -> Option<Vec2> {
    if let Some(phys_pos) = window.physical_cursor_position() {
        Some(
            camera_orth_proj.scale
                * (phys_pos - Vec2::new(window.width() / 2., window.height() / 2.))
                * Vec2::new(1., -1.)
                + Vec2::new(camera_translation.x, camera_translation.y)
        )
    } else {
        None
    }
}

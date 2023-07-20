use bevy::prelude::*;

/// Adds some more gizmos
pub trait GizmosExt {
    fn arrow_2d(&mut self, start: Vec2, end: Vec2, arrow_head_width: f32, color: Color);
}

impl GizmosExt for Gizmos<'_> {
    fn arrow_2d(&mut self, start: Vec2, end: Vec2, arrow_head_width: f32, color: Color) {
        let arrow_half_width = arrow_head_width / 2.;
        let arrow_normal = (end - start).normalize();

        let arrow_head_left_delta =
            Mat2::from_angle(std::f32::consts::FRAC_PI_2) * arrow_normal * arrow_half_width;
        let arrow_head_right_delta = -arrow_head_left_delta;
        let arrow_head_tip_delta = arrow_normal * arrow_half_width * 3f32.sqrt();
        let arrow_head_base = end - (arrow_normal * arrow_half_width * 0.5);

        self.linestrip_2d(
            [
                start,
                end,
                (arrow_head_base) + arrow_head_left_delta,
                (arrow_head_base) + arrow_head_tip_delta,
                (arrow_head_base) + arrow_head_right_delta,
                end,
            ],
            color,
        );
    }
}

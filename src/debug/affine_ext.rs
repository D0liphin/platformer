use bevy::{
    math::Affine3A,
    prelude::{EulerRot, Quat},
};

pub trait AffineExt {
    fn rot_2d(&self) -> f32;
}

impl AffineExt for Affine3A {
    fn rot_2d(&self) -> f32 {
        Quat::from_affine3(self).to_euler(EulerRot::XYZ).2
    }
}

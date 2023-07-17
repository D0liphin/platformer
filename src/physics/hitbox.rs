use bevy::prelude::*;
use bevy_rapier2d::{parry::bounding_volume::Aabb as RapierAabb, rapier::prelude::Point};
use serde::Deserialize;

#[derive(Component)]
pub struct Aabb(RapierAabb);

impl<'de> Deserialize<'de> for Aabb {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct TransparentAabb {
            top: f32,
            right: f32,
            bottom: f32,
            left: f32,
        }

        impl TransparentAabb {
            fn as_aabb(self) -> Aabb {
                Aabb(RapierAabb::new(
                    Point::new(self.left, self.bottom),
                    Point::new(self.right, self.top),
                ))
            }
        }

        TransparentAabb::deserialize(deserializer).map(TransparentAabb::as_aabb)
    }
}

#[non_exhaustive]
#[derive(Component, Deserialize)]
pub enum AabbSpecialization {
    /// Standard hitbox type -- our main character can jump off walls etc. etc. yipee
    Solid,
}

#[derive(Bundle, Deserialize)]
pub struct SpecializedAabb {
    pub aabb: Aabb,
    pub specialization: AabbSpecialization,
}

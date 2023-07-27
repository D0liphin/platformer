use bevy::prelude::*;

#[derive(Clone, Debug, Copy, Component, Reflect)]
pub struct KinematicVelocity {
    prev_linvel: Vec2,
    pub linvel: Vec2,
    prev_angvel: f32,
    angvel: f32,
}

impl KinematicVelocity {
    pub fn zero() -> Self {
        Self {
            prev_linvel: Vec2::ZERO,
            linvel: Vec2::ZERO,
            prev_angvel: 0.,
            angvel: 0.,
        }
    }

    #[inline]
    pub fn effective_linvel(&self, dt: f32) -> Vec2 {
        self.prev_linvel * dt + self.linvel * dt * 0.5
    }

    pub fn halt_linvel(&mut self) {
        self.linvel = Vec2::ZERO;
        self.prev_linvel = Vec2::ZERO;
    }

    pub fn update_prev_linvel(&mut self) {
        self.prev_linvel = self.linvel;
    }

    /// Set the x velocity, effective immediately (rather than in a frame)
    pub fn hard_assign_x(&mut self, value: f32) {
        self.prev_linvel.x = value;
        self.linvel.x = value;
    }

    pub fn hard_add_assign_x(&mut self, diff: f32) {
        self.hard_assign_x(self.linvel.x + diff);
    }

    pub fn hard_assign_y(&mut self, value: f32) {
        self.prev_linvel.y = value;
        self.linvel.y = value;
    }
}
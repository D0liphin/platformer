pub type Vec2F = nalgebra::Vector2<f32>;
pub type Array<T> = Box<[T]>;

pub mod animations {
    use crate::animation::AnimationKey;

    macro_rules! animation_keys {
        {$($KEY:ident = $value:tt);*$(;)?} => {
            $(
                pub const $KEY: AnimationKey = $value;
            )*
        };
    }

    animation_keys! {
        NULL = 0;
        PLAYER_IDLE = 1;
        PLAYER_RUN = 2;
    }
}

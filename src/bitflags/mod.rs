use std::ops::{BitAnd, BitOr, Shl, Shr, Not};
use bytemuck::Zeroable;

pub trait BitFlags:
    Sized
    + Shl<Output = Self>
    + Shr<Output = Self>
    + BitAnd<Output = Self>
    + BitOr<Output = Self>
    + Not<Output = Self>
    + Eq
    + Copy
    + Zeroable
{
    fn set_high(&mut self, flag: Self) {
        *self = *self | flag;
    }

    fn set_low(&mut self, flag: Self) {
        *self = *self & !flag;
    }

    fn is_high(&self, flag: Self) -> bool {
        !((*self & flag) == Self::zeroed())
    }

    fn is_low(&self, flag: Self) -> bool {
        (*self & flag) == Self::zeroed()
    }
}

impl BitFlags for u8 {}
impl BitFlags for u16 {}
impl BitFlags for u32 {}
impl BitFlags for u64 {}
impl BitFlags for u128 {}
impl BitFlags for usize {}
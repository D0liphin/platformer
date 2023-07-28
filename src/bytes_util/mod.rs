use bevy::prelude::*;
use bytemuck::{Pod, Zeroable};
use std::mem::size_of;

#[macro_export]
macro_rules! byte_muckable {
    ($Struct:item) => {
        #[derive(Clone, Copy, Zeroable, Pod)]
        #[repr(C)]
        $Struct
    };
}

pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn write_bytes(&mut self, writer: &impl WriteBytes) {
        writer.write_bytes(self);
    }

    pub fn push(&mut self, byte: u8) {
        self.0.push(byte);
    }

    pub fn extend<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        self.0.extend(bytes);
    }
}

pub trait WriteBytes {
    fn write_bytes(&self, bytes: &mut Bytes);
}

pub struct BytesWindow<'a> {
    bytes: &'a [u8],
    index: usize,
}

impl<'a> BytesWindow<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        Self { bytes, index: 0 }
    }

    pub fn acquire_sized(&mut self, size: usize) -> &[u8] {
        let res = &self.bytes[self.index..self.index + size];
        self.index += size;
        res
    }

    pub fn acquire_unsized(&mut self) -> &[u8] {
        byte_muckable!(
            struct Usize(usize);
        );
        let size = bytemuck::pod_read_unaligned::<Usize>(self.acquire_sized(size_of::<Usize>())).0;
        self.acquire_sized(size)
    }
}

pub trait ConstSlice<T> {
    fn const_slice<const START: usize, const END: usize>(&self) -> [T; END - START];
}

impl<Item: Copy> ConstSlice<Item> for &[Item] {
    fn const_slice<const START: usize, const END: usize>(&self) -> [Item; END - START] {
        self[START..END].try_into().unwrap()
    }
}

pub trait FromBytes {
    fn from_bytes(window: &mut BytesWindow) -> Self;
}

impl WriteBytes for &[u8] {
    fn write_bytes(&self, bytes: &mut Bytes) {
        byte_muckable!(
            struct Usize(usize);
        );
        bytes.extend(bytemuck::cast::<Usize, [u8; size_of::<Usize>()]>(Usize(self.len())));
        bytes.extend(self.iter().map(|n| *n));
    }
}

impl FromBytes for Box<[u8]> {
    fn from_bytes(window: &mut BytesWindow) -> Self {
        window.acquire_unsized().into()
    }
}

impl WriteBytes for &str {
    fn write_bytes(&self, bytes: &mut Bytes) {
        self.as_bytes().write_bytes(bytes);
    }
}

impl FromBytes for Box<str> {
    fn from_bytes(window: &mut BytesWindow) -> Self {
        std::str::from_utf8(window.acquire_unsized())
            .unwrap()
            .into()
    }
}

pub trait IntoVecU8 {
    fn into_vec_u8(&self) -> Vec<u8>;
}

impl<T: WriteBytes> IntoVecU8 for T {
    fn into_vec_u8(&self) -> Vec<u8> {
        let mut bytes = Bytes(Vec::new());
        self.write_bytes(&mut bytes);
        bytes.0
    }
}

pub trait FromU8Slice {
    fn from_u8_slice(slice: &[u8]) -> Self;
}

impl<T: FromBytes> FromU8Slice for T {
    fn from_u8_slice(slice: &[u8]) -> Self {
        Self::from_bytes(&mut BytesWindow::new(slice))
    }
}

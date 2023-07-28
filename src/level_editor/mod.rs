//! levels are stored in assets/levels/level_name
//! levels contain several .png images representing chunks of the level.
//! They also contain a sled db file that contains information about the objects in a level, the
//! hitboxes in a level etc. etc.

use crate::bytes_util::*;
use bevy::{asset::FileAssetIo, prelude::*};
use std::mem::size_of;

pub struct Level {
    /// BTreeMap<ChunkLocation, ChunkDescriptor>
    db: sled::Db,
}

impl Level {
    pub fn open(path: &'static str) -> Self {
        let mut db_path = FileAssetIo::get_base_path();
        db_path.push("levels");
        db_path.push(path);
        Self {
            db: sled::open(db_path).unwrap(),
        }
    }

    pub fn insert(&mut self, loc: ChunkLocation, desc: ChunkDescriptor) {
        _ = self.db.insert(loc.into_vec_u8(), desc.into_vec_u8());
    }

    pub fn get(&self, loc: &ChunkLocation) -> Option<ChunkDescriptor> {
        match self.db.get(loc.into_vec_u8()) {
            Result::Ok(Some(v)) => Some(ChunkDescriptor::from_u8_slice(&v)),
            _ => None,
        }
    }
}

pub type ChunkCoord = i32;

pub struct ChunkLocation {
    pub x: ChunkCoord,
    pub y: ChunkCoord,
}

impl ChunkLocation {
    pub fn new(x: ChunkCoord, y: ChunkCoord) -> Self {
        Self { x, y }
    }
}

impl WriteBytes for ChunkLocation {
    fn write_bytes(&self, bytes: &mut Bytes) {
        bytes.extend(self.x.to_be_bytes());
        bytes.extend(self.y.to_be_bytes());
    }
}

impl FromBytes for ChunkLocation {
    fn from_bytes(window: &mut BytesWindow) -> Self {
        let bytes = window.acquire_sized(size_of::<Self>());
        const SIZE: usize = size_of::<ChunkCoord>();
        let x = ChunkCoord::from_be_bytes(bytes.const_slice::<0, SIZE>());
        let y = ChunkCoord::from_be_bytes(bytes.const_slice::<SIZE, { SIZE * 2 }>());
        Self { x, y }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkDecoration {
    pub path: Box<str>,
}

impl WriteBytes for ChunkDecoration {
    fn write_bytes(&self, bytes: &mut Bytes) {
        (&*self.path).write_bytes(bytes);
    }
}

impl FromBytes for ChunkDecoration {
    fn from_bytes(window: &mut BytesWindow) -> Self {
        Self {
            path: Box::<str>::from_bytes(window),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ChunkDescriptor {
    /// path to the background image for this chunk
    pub bg: ChunkDecoration,
}

impl WriteBytes for ChunkDescriptor {
    fn write_bytes(&self, bytes: &mut Bytes) {
        self.bg.write_bytes(bytes);
    }
}

impl FromBytes for ChunkDescriptor {
    fn from_bytes(window: &mut BytesWindow) -> Self {
        Self {
            bg: ChunkDecoration::from_bytes(window),
        }
    }
}

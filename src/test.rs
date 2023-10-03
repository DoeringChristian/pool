use std::fmt;

use crate::hashpool::HashPool;
use crate::*;

#[derive(thiserror::Error, Debug)]
pub struct BufferError;

impl fmt::Display for BufferError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Buffer Error!")
    }
}

pub struct Buffer {
    data: Vec<i32>,
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub struct BufferInfo {
    size: usize,
}

impl Resource for Buffer {
    type Info = BufferInfo;

    type Context = ();

    type CreateError = BufferError;

    fn create(info: &Self::Info, ctx: &Self::Context) -> Result<Self, Self::CreateError> {
        Ok(Buffer {
            data: Vec::with_capacity(info.size),
        })
    }

    fn clear(&mut self) {
        self.data.clear()
    }
}

#[test]
fn create_buffer() {
    let mut pool = HashPool::<Buffer>::default();
    let buffer = pool.lease(&BufferInfo { size: 100 }, &()).unwrap();
}

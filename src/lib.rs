mod hashpool;
#[cfg(test)]
mod test;

use std::error::Error;

pub trait Pool<R: Resource> {
    type Lease;

    fn lease(&mut self, info: &R::Info, ctx: &R::Context) -> Result<Self::Lease, R::CreateError>;
}

pub trait Resource: Sized {
    type Info: Eq + PartialEq + Clone;
    type Context;
    type CreateError: Error;

    fn create(info: &Self::Info, ctx: &Self::Context) -> Result<Self, Self::CreateError>;
    fn clear(&mut self);
}

pub mod repeat;

use crate::tryable::Tryable;
use core::future::Future;

pub trait AsyncFn<F> {
    fn call(&mut self) -> F;
}

impl<F, Fut, Out> AsyncFn<Fut> for F
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Out>,
{
    fn call(&mut self) -> Fut {
        self()
    }
}

pub trait TryAsyncFn<F> {
    fn call(&mut self) -> F;
}

impl<F, Fut, Out> TryAsyncFn<Fut> for F
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Out>,
    Out: Tryable,
{
    fn call(&mut self) -> Fut {
        self()
    }
}

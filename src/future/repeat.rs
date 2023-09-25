//! Title: Repeater
//!
//! How to use:
//! ```rust
//! use retry::future::*;
//! pub async fn myfunc() {
//!     println!("Hello, world!");
//!     // Other stuff
//! }
//! myfunc.repeat::<3>().await;
//! ```

use core::future::Future;
use core::pin::Pin;
use core::task::Poll;

#[pin_project::pin_project]
#[non_exhaustive]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Repeater<F, Args, Fut> {
    f: F,
    #[pin]
    state: RepeaterStates<Fut>,
    repeat: usize,
    args: Args,
}

impl<F, Args: core::fmt::Debug, Fut> core::fmt::Debug for Repeater<F, Args, Fut> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Repeater")
            .field("args", &self.args)
            .field("repeat", &self.repeat)
            .finish()
    }
}

// impl<F, Fut: Unpin> Unpin for Repeater<F, Fut> {}

#[pin_project::pin_project(project = RepeaterState)]
pub enum RepeaterStates<F> {
    Pending,
    Ready(#[pin] F),
}

impl<F, Fut, Out> Future for Repeater<F, (), Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Out>,
{
    type Output = Out;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let mut tries = *this.repeat;
        match this.state.as_mut().project() {
            RepeaterState::Pending => {
                // Create the future from the function
                let fut = (this.f)();
                this.state.set(RepeaterStates::Ready(fut));
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            RepeaterState::Ready(ref mut fut) => {
                match Pin::new(fut).poll(cx) {
                    // If it's already ready then decrement the repeat counter for 1
                    Poll::Ready(v) => {
                        tries -= 1;
                        if tries == 0 {
                            Poll::Ready(v)
                        } else {
                            // So we need to retry more so create a new future
                            let fut = (this.f)();
                            this.state.set(RepeaterStates::Ready(fut));
                            *this.repeat = tries;
                            cx.waker().wake_by_ref();
                            Poll::Pending
                        }
                    }
                    Poll::Pending => {
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
            }
        }
    }
}

pub trait AsyncRepeat0<Fut>: Sized {
    fn repeat<const N: usize>(self) -> Repeater<Self, (), Fut>;
}

impl<F, Fut, Out> AsyncRepeat0<Fut> for F
where
    F: Fn() -> Fut,
    Fut: Future<Output = Out>,
{
    fn repeat<const N: usize>(self) -> Repeater<Self, (), Fut> {
        Repeater {
            f: self,
            state: RepeaterStates::Pending,
            repeat: N,
            args: (),
        }
    }
}

macro_rules! impl_gen_async_repeat {
    ($name:ident, $($item: ident),*) => {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub trait $name<Fut, $($item),*>: Sized {
            fn repeat<const N: usize>(self, $($item: $item),*) -> Repeater<Self, ($($item),*,), Fut> {
                Repeater {
                    f: self,
                    state: RepeaterStates::Pending,
                    repeat: N,
                    args: ($($item),*,),
                }
            }
        }

        #[allow(non_snake_case)]
        impl<F, Fut, Out, $($item),*> $name<Fut, $($item),*> for F
        where
            F: Fn($($item),*) -> Fut,
            Fut: Future<Output = Out>,
        { }

        #[allow(non_snake_case)]
        impl<F, Fut, Out, $($item: Clone),*> Future for Repeater<F, ($($item),*,), Fut>
            where
                F: Fn($($item),*) -> Fut,
                Fut: Future<Output = Out>,
        {
            type Output = Out;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
                let mut this = self.project();
                let mut tries = *this.repeat;
                match this.state.as_mut().project() {
                    RepeaterState::Pending => {
                        // Create the future from the function
                        let ( $($item),*, ) = this.args.clone();
                        let fut = (this.f)($($item),*,);
                        this.state.set(RepeaterStates::Ready(fut));
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }

                    RepeaterState::Ready(ref mut fut) => {
                        match Pin::new(fut).poll(cx) {
                            // If it's already ready then decrement the repeat counter for 1
                            Poll::Ready(v) => {
                                tries -= 1;
                                if tries == 0 {
                                    Poll::Ready(v)
                                } else {
                                    let ( $($item),*, ) = this.args.clone();
                                    let fut = (this.f)($($item),*,);
                                    this.state.set(RepeaterStates::Ready(fut));
                                    *this.repeat = tries;
                                    cx.waker().wake_by_ref();
                                    Poll::Pending
                                }
                            }
                            Poll::Pending => {
                                cx.waker().wake_by_ref();
                                Poll::Pending
                            }
                        }
                    }
                }
            }

         }
    };
}

impl_gen_async_repeat!(AsyncRepeat1, A1);
impl_gen_async_repeat!(AsyncRepeat2, A1, A2);
impl_gen_async_repeat!(AsyncRepeat3, A1, A2, A3);
impl_gen_async_repeat!(AsyncRepeat4, A1, A2, A3, A4);
impl_gen_async_repeat!(AsyncRepeat5, A1, A2, A3, A4, A5);
impl_gen_async_repeat!(AsyncRepeat6, A1, A2, A3, A4, A5, A6);
impl_gen_async_repeat!(AsyncRepeat7, A1, A2, A3, A4, A5, A6, A7);
impl_gen_async_repeat!(AsyncRepeat8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_gen_async_repeat!(AsyncRepeat9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_gen_async_repeat!(AsyncRepeat10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

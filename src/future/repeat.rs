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
use core::pin::{pin, Pin};
use core::task::Poll;

use super::AsyncFn;

// #[pin_project::pin_project]
pub struct Repeater<F, Fut> {
    f: F,
    state: RepeaterState<Fut>,
    repeat: usize,
}

// impl<F, Fut: Unpin> Unpin for Repeater<F, Fut> {}

pub enum RepeaterState<F> {
    Pending,
    Ready(F),
}

pub trait AsyncRepeat0<Fut>: Sized {
    fn repeat(self, repeat: usize) -> Repeater<Self, Fut>;
}

impl<F, Fut, Out> AsyncRepeat0<Fut> for F
where
    F: AsyncFn<Fut>,
    Fut: Future<Output = Out>,
{
    fn repeat(self, repeat: usize) -> Repeater<Self, Fut> {
        Repeater {
            f: self,
            state: RepeaterState::Pending,
            repeat,
        }
    }
}

impl<F, Fut, Out> Future for Repeater<F, Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Out> + Unpin,
{
    type Output = Out;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self;
        let mut tries = this.repeat;
        match this.state {
            RepeaterState::Pending => {
                // Create the future from the function
                let fut = (this.f)();
                this.state = RepeaterState::Ready(fut);
                Poll::Pending
            }
            RepeaterState::Ready(ref mut fut) => match Pin::new(fut).poll(cx) {
                // If it's already ready then decrement the repeat counter for 1
                Poll::Ready(v) => {
                    tries -= 1;
                    if tries == 0 {
                        Poll::Ready(v)
                    } else {
                        // So we need to retry more so create a new future
                        let fut = (this.f)();
                        this.state = RepeaterState::Ready(fut);
                        this.repeat = tries;
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
                }
                Poll::Pending => Poll::Pending,
            },
        }
    }
}

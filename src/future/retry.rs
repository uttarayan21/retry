use core::future::Future;
use core::pin::Pin;
use core::task::Poll;

#[pin_project::pin_project(project = RetryStates)]
pub enum RetryState<F> {
    Pending,
    Ready(#[pin] F),
}

#[pin_project::pin_project]
pub struct Retrier<F, Args, Fut> {
    retry: usize,
    f: F,
    #[pin]
    state: RetryState<Fut>,
    args: Args,
}

impl<F, Fut, Out> Future for Retrier<F, (), Fut>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Out>,
{
    type Output = Out;
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        let mut tries = *this.retry;
        match this.state.as_mut().project() {
            RetryStates::Pending => {
                // Create the future from the function
                let fut = (this.f)();
                this.state.set(RetryState::Ready(fut));
                cx.waker().wake_by_ref();
                Poll::Pending
            }

            RetryStates::Ready(ref mut fut) => {
                match Pin::new(fut).poll(cx) {
                    // If it's already ready then decrement the repeat counter for 1
                    Poll::Ready(v) => {
                        tries -= 1;
                        if tries == 0 {
                            Poll::Ready(v)
                        } else {
                            // So we need to retry more so create a new future
                            let fut = (this.f)();
                            this.state.set(RetryState::Ready(fut));
                            *this.retry = tries;
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

pub trait AsyncRetry0<Fut>: Sized {
    fn retry<const N: usize>(self) -> Retrier<Self, (), Fut>;
}

impl<F, Fut, Out> AsyncRetry0<Fut> for F
where
    F: Fn() -> Out,
    Fut: Future<Output = Out>,
{
    fn retry<const N: usize>(self) -> Retrier<Self, (), Fut> {
        Retrier {
            retry: N,
            f: self,
            state: RetryState::Pending,
            args: (),
        }
    }
}

macro_rules! impl_gen_async_retry {
    ($name: ident, $($item: ident),*) => {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub trait $name<Fut, $($item),*>: Sized {
            fn retry<const N: usize>(self, $($item: $item),*) -> Retrier<Self, ($($item),*,), Fut> {
                Retrier {
                    retry: N,
                    f: self,
                    state: RetryState::Pending,
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
        impl<F, Fut, Out, $($item: Clone),*> Future for Retrier<F, ($($item),*,), Fut>
            where
                F: Fn($($item),*) -> Fut,
                Fut: Future<Output = Out>,
        {
            type Output = Out;
            fn poll(self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
                let mut this = self.project();
                let mut tries = *this.retry;
                match this.state.as_mut().project() {
                    RetryStates::Pending => {
                        // Create the future from the function
                        let ( $($item),*, ) = this.args.clone();
                        let fut = (this.f)($($item),*,);
                        this.state.set(RetryState::Ready(fut));
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }

                    RetryStates::Ready(ref mut fut) => {
                        match Pin::new(fut).poll(cx) {
                            // If it's already ready then decrement the repeat counter for 1
                            Poll::Ready(v) => {
                                tries -= 1;
                                if tries == 0 {
                                    Poll::Ready(v)
                                } else {
                                    let ( $($item),*, ) = this.args.clone();
                                    let fut = (this.f)($($item),*,);
                                    this.state.set(RetryState::Ready(fut));
                                    *this.retry = tries;
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

    }
}

impl_gen_async_retry!(AsyncRetry1, A1);
impl_gen_async_retry!(AsyncRetry2, A1, A2);
impl_gen_async_retry!(AsyncRetry3, A1, A2, A3);
impl_gen_async_retry!(AsyncRetry4, A1, A2, A3, A4);
impl_gen_async_retry!(AsyncRetry5, A1, A2, A3, A4, A5);
impl_gen_async_retry!(AsyncRetry6, A1, A2, A3, A4, A5, A6);
impl_gen_async_retry!(AsyncRetry7, A1, A2, A3, A4, A5, A6, A7);
impl_gen_async_retry!(AsyncRetry8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_gen_async_retry!(AsyncRetry9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_gen_async_retry!(AsyncRetry10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

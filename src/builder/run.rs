use super::repeat::*;
use super::retry::*;
use crate::tryable::Tryable;

macro_rules! impl_gen_retry {
    ($name: ident, $($item: ident),*) => {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub trait $name<$($item),*, Output> {
            fn run(&mut self, $($item: $item),*) -> Output;
        }
        #[allow(non_snake_case)]
        impl<F, $($item: Clone),*, Output> $name<$($item),*, Output> for Retrier<F>
        where
            F: Fn($($item),*) -> Output,
            Output: Tryable,
        {
            fn run(&mut self, $($item: $item),*) -> Output {
                let mut retry = self.retry;
                loop {
                    let res = (self.func)($($item.clone()),*);
                    if res.negative() {
                        retry -= 1;
                    } else {
                        return res;
                    }
                    if retry == 0 {
                        return res;
                    }
                }
            }
        }

        #[allow(non_snake_case)]
        impl<F, $($item: Clone),*, Output> $name<$($item),*, Output> for Repeater<F>
        where
            F: Fn($($item),*) -> Output,
        {
            fn run(&mut self, $($item: $item),*) -> Output {
                let mut repeat = self.repeat;
                loop {
                    let res = (self.func)($($item.clone()),*);
                    repeat -= 1;
                    if repeat == 0 {
                        return res;
                    }
                }
            }
        }
    };
}

impl_gen_retry!(Run1, A1);
impl_gen_retry!(Run2, A1, A2);
impl_gen_retry!(Run3, A1, A2, A3);
impl_gen_retry!(Run4, A1, A2, A3, A4);
impl_gen_retry!(Run5, A1, A2, A3, A4, A5);
impl_gen_retry!(Run6, A1, A2, A3, A4, A5, A6);
impl_gen_retry!(Run7, A1, A2, A3, A4, A5, A6, A7);
impl_gen_retry!(Run8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_gen_retry!(Run9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_gen_retry!(Run10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

pub trait Run0<Output> {
    fn run(&mut self) -> Output;
}

impl<F, Output> Run0<Output> for Retrier<F>
where
    F: FnMut() -> Output,
    Output: Tryable,
{
    fn run(&mut self) -> Output {
        let mut retry = self.retry;
        loop {
            let res = (self.func)();
            if res.negative() {
                retry -= 1;
            } else {
                return res;
            }
            if retry == 0 {
                return res;
            }
        }
    }
}

impl<F, Output> Run0<Output> for Repeater<F>
where
    F: FnMut() -> Output,
{
    fn run(&mut self) -> Output {
        let mut repeat = self.repeat;
        loop {
            let res = (self.func)();
            repeat -= 1;
            if repeat == 0 {
                return res;
            }
        }
    }
}

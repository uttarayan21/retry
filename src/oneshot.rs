use crate::tryable::Tryable;
macro_rules! impl_gen_retry_for_tuple {
    ($name: ident, $($item: ident),*) => (
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub trait $name<$($item),*, Output>: Sized {
            fn retry<const N: usize>(self, $($item: $item),*) -> Output;
        }
        #[allow(non_snake_case)]
        impl<F, $($item: Clone),*, Output> $name<$($item),*, Output> for F
        where
            F: FnMut($($item),*) -> Output,
            Output: Tryable,
        {
            fn retry<const N: usize>(mut self, $($item: $item),*) -> Output {
                let mut retry = N;
                loop {
                    let result = self($($item.clone()),*);
                    if result.negative() {
                        if retry == 0 {
                            return result;
                        }
                        retry -= 1;
                    } else {
                        return result;
                    }
                }
            }
        }
    )
}

pub trait RetryOneshot0<Output>: Sized {
    fn retry<const N: usize>(self) -> Output;
}

impl<F, Output> RetryOneshot0<Output> for F
where
    F: FnMut() -> Output,
    Output: Tryable,
{
    fn retry<const N: usize>(mut self) -> Output {
        let mut retry = N;
        loop {
            let result = self();
            if result.negative() {
                if retry == 0 {
                    return result;
                }
                retry -= 1;
            } else {
                return result;
            }
        }
    }
}

impl_gen_retry_for_tuple!(RetryOneshot1, A1);
impl_gen_retry_for_tuple!(RetryOneshot2, A1, A2);
impl_gen_retry_for_tuple!(RetryOneshot3, A1, A2, A3);
impl_gen_retry_for_tuple!(RetryOneshot4, A1, A2, A3, A4);
impl_gen_retry_for_tuple!(RetryOneshot5, A1, A2, A3, A4, A5);
impl_gen_retry_for_tuple!(RetryOneshot6, A1, A2, A3, A4, A5, A6);
impl_gen_retry_for_tuple!(RetryOneshot7, A1, A2, A3, A4, A5, A6, A7);
impl_gen_retry_for_tuple!(RetryOneshot8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_gen_retry_for_tuple!(RetryOneshot9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_gen_retry_for_tuple!(RetryOneshot10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

macro_rules! impl_gen_repeat_for_tuple {
    ($name: ident, $($item: ident),*) => {
        #[allow(non_snake_case, clippy::too_many_arguments)]
        pub trait $name<$($item),*, Output>: Sized {
            fn repeat<const N: usize>(&mut self, $($item: $item),*) -> Output;
        }

        #[allow(non_snake_case)]
        impl<F, $($item: Clone),*,Output> $name<$($item),*,Output> for F
        where
            F: FnMut($($item),*) -> Output,
        {
            fn repeat<const N: usize>(&mut self, $($item: $item),*) -> Output {
                let mut repeat = N;
                loop {
                    let res = self($($item.clone()),*);
                    repeat -= 1;
                    if repeat == 0 {
                        break res;
                    }
                }
            }
        }

    };
}

pub trait RepeatOneshot0<Output>: Sized {
    fn repeat<const N: usize>(&mut self) -> Output;
}

impl<F, Output> RepeatOneshot0<Output> for F
where
    F: FnMut() -> Output,
{
    fn repeat<const N: usize>(&mut self) -> Output {
        let mut repeat = N;
        loop {
            let res = self();
            repeat -= 1;
            if repeat == 0 {
                break res;
            }
        }
    }
}

impl_gen_repeat_for_tuple!(RepeatOneshot1, A1);
impl_gen_repeat_for_tuple!(RepeatOneshot2, A1, A2);
impl_gen_repeat_for_tuple!(RepeatOneshot3, A1, A2, A3);
impl_gen_repeat_for_tuple!(RepeatOneshot4, A1, A2, A3, A4);
impl_gen_repeat_for_tuple!(RepeatOneshot5, A1, A2, A3, A4, A5);
impl_gen_repeat_for_tuple!(RepeatOneshot6, A1, A2, A3, A4, A5, A6);
impl_gen_repeat_for_tuple!(RepeatOneshot7, A1, A2, A3, A4, A5, A6, A7);
impl_gen_repeat_for_tuple!(RepeatOneshot8, A1, A2, A3, A4, A5, A6, A7, A8);
impl_gen_repeat_for_tuple!(RepeatOneshot9, A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_gen_repeat_for_tuple!(RepeatOneshot10, A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

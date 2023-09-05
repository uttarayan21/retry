#[derive(Debug, Clone)]
#[must_use = "retry() does nothing unless you `.run(..)` it"]
pub struct Retrier<T> {
    pub(crate) retry: usize,
    pub(crate) func: T,
}

pub trait Retry<Args, Output>: Sized {
    fn retry(self, times: usize) -> Retrier<Self>;
}

impl<F, O> Retry<(), O> for F
where
    F: FnMut() -> O,
{
    fn retry(self, times: usize) -> Retrier<Self> {
        Retrier {
            retry: times,
            func: self,
        }
    }
}

macro_rules! impl_for_tuple {
    ($($tup: ident),*) => (
        impl<F, $($tup),*, O> Retry<($($tup),*,), O> for F
        where
            F: FnMut($($tup),*) -> O,
        {
            fn retry(self, times: usize) -> Retrier<Self> {
                Retrier {
                    retry: times,
                    func: self,
                }
            }
        }
        )
}

impl_for_tuple!(A1);
impl_for_tuple!(A1, A2);
impl_for_tuple!(A1, A2, A3);
impl_for_tuple!(A1, A2, A3, A4);
impl_for_tuple!(A1, A2, A3, A4, A5);
impl_for_tuple!(A1, A2, A3, A4, A5, A6);
impl_for_tuple!(A1, A2, A3, A4, A5, A6, A7);
impl_for_tuple!(A1, A2, A3, A4, A5, A6, A7, A8);
impl_for_tuple!(A1, A2, A3, A4, A5, A6, A7, A8, A9);
impl_for_tuple!(A1, A2, A3, A4, A5, A6, A7, A8, A9, A10);

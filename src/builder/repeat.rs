#[must_use = "repeat() does nothing unless you `.run(..)` it"]
#[derive(Debug, Clone)]
pub struct Repeater<T> {
    pub(crate) repeat: usize,
    pub(crate) func: T,
}

pub trait Repeat<Args, Output>: Sized {
    fn repeat(self, times: usize) -> Repeater<Self>;
}

impl<F, Output> Repeat<(), Output> for F
where
    F: FnMut() -> Output,
{
    fn repeat(self, times: usize) -> Repeater<Self> {
        Repeater {
            repeat: times,
            func: self,
        }
    }
}

macro_rules! impl_for_tuple {
    ($($tup: ident),*) => (
        impl<F, $($tup),*, O> Repeat<($($tup),*,), O> for F
        where
            F: FnMut($($tup),*) -> O,
        {
            fn repeat(self, times: usize) -> Repeater<Self> {
                Repeater {
                    repeat: times,
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

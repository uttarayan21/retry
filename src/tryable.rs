pub trait Tryable: seal::Sealed {
    type Ok;
    type Error;
    fn negative(&self) -> bool;
}

impl<Ok, Error> Tryable for Result<Ok, Error> {
    type Ok = Ok;
    type Error = Error;
    fn negative(&self) -> bool {
        self.is_err()
    }
}

impl<T> Tryable for Option<T> {
    type Ok = T;
    type Error = ();
    fn negative(&self) -> bool {
        self.is_none()
    }
}

mod seal {
    pub trait Sealed {}
    impl<T> Sealed for Option<T> {}
    impl<T, E> Sealed for Result<T, E> {}
}

pub trait Tryable {
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

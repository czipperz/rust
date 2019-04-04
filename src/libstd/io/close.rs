pub trait Close {
    type Error;

    fn close(self) -> Result<(), Error>;
}

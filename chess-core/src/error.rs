#[derive(Debug)]
pub struct OutOfBounds;

#[derive(Debug)]
pub enum Error {
    OutOfBounds,
    NoPiece,
}

impl From<OutOfBounds> for Error {
    fn from(_: OutOfBounds) -> Self {
        Error::OutOfBounds
    }
}

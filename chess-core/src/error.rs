#[derive(Debug)]
pub struct OutOfBounds;

#[derive(Debug)]
pub enum InvalidDiff {
    /// Tried to capture, when type of move is MoveType::Move
    CaptureOnMoveTy,
    /// Failed to capture, when type of move is MoveType::Capture
    MoveOnCaptureTy,
    /// Tried to promote a non-pawn piece
    InvalidPromotionPiece,
    /// Tried to promote from the wrong row
    InvalidPromotionRow,
}

#[derive(Debug)]
pub enum Error {
    InvalidDiff(InvalidDiff),
    OutOfBounds,
    NoPiece,
}

impl From<OutOfBounds> for Error {
    fn from(_: OutOfBounds) -> Self {
        Error::OutOfBounds
    }
}

impl From<InvalidDiff> for Error {
    fn from(d: InvalidDiff) -> Self {
        Error::InvalidDiff(d)
    }
}

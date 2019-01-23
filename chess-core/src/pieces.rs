use crate::math::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

pub struct Piece {
    pub ty: PieceType,
    pub pos: Vector
}

use crate::math::Vector;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveType {
    Move,
    Capture,
    MoveCapture,
}

/**
 * Represents a virtual move, a possible legal move
 * (a move that is legal if there are no other rules to prevent the move)
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VMove(
    pub(crate) Piece,
    pub(crate) Vector,
    pub(crate) MoveType,
    pub(crate) u32,
);

impl Piece {
    pub fn get_ident(self) -> char {
        match self {
            Piece::Pawn => 'P',
            Piece::Knight => 'N',
            Piece::Bishop => 'B',
            Piece::Rook => 'R',
            Piece::Queen => 'Q',
            Piece::King => 'K',
        }
    }

    pub fn get_moves(self) -> &'static [VMove] {
        macro_rules! moves {
            ($name: ident
             $piece: ident
            $((
                $($rest:tt)*
            ))*) => {
                static $name: &[VMove] = &[
                    $(
                        moves!(@parse Piece::$piece, $($rest)*)
                    ),*
                ];
            };

            (@parse $piece:expr, $x:literal, $y:literal, $dist:literal, $mt:ident) => {
                VMove($piece, Vector { x: $x, y: $y }, MoveType::$mt, $dist)
            };
            (@parse $piece:expr, $x:literal, $y:literal, $dist:literal) => {
                moves!(@parse $piece, $x, $y, $dist, MoveCapture)
            };
            (@parse $piece:expr, $x:literal, $y:literal) => {
                moves!(@parse $piece, $x, $y, 8)
            };
        }

        moves! {
            PAWN_MOVES
            Pawn
            ( 0, 1, 1, Move)
            ( 0, 2, 1, Move)
            ( 1, 1, 1, Capture)
            (-1, 1, 1, Capture)
        }

        moves! {
            KNIGHT_MOVES
            Knight
            ( 1, -2, 1)
            ( 1,  2, 1)
            (-1, -2, 1)
            (-1,  2, 1)
            ( 2, -1, 1)
            ( 2,  1, 1)
            (-2, -1, 1)
            (-2,  1, 1)
        }

        moves! {
            BISHOP_MOVES
            Bishop
            ( 1,  1)
            ( 1, -1)
            (-1,  1)
            (-1, -1)
        }

        moves! {
            ROOK_MOVES
            Rook
            ( 1,  0)
            (-1,  0)
            ( 0,  1)
            ( 0, -1)
        }

        moves! {
            QUEEN_MOVES
            Queen
            ( 1,  0)
            (-1,  0)
            ( 0,  1)
            ( 0, -1)
            ( 1,  1)
            ( 1, -1)
            (-1,  1)
            (-1, -1)
        }

        moves! {
            KING_MOVES
            King
            ( 1,  0, 1)
            (-1,  0, 1)
            ( 0,  1, 1)
            ( 0, -1, 1)
            ( 1,  1, 1)
            ( 1, -1, 1)
            (-1,  1, 1)
            (-1, -1, 1)
        }

        match self {
            Piece::Pawn => PAWN_MOVES,
            Piece::Knight => KNIGHT_MOVES,
            Piece::Bishop => BISHOP_MOVES,
            Piece::Rook => ROOK_MOVES,
            Piece::Queen => QUEEN_MOVES,
            Piece::King => KING_MOVES,
        }
    }
}

impl Color {
    pub fn get_ident(self) -> char {
        match self {
            Color::White => 'W',
            Color::Black => 'B',
        }
    }

    pub fn dir(self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

impl MoveType {
    pub fn is_capture(self) -> bool {
        match self {
            MoveType::Move => false,
            _ => true,
        }
    }

    pub fn is_normal(self) -> bool {
        match self {
            MoveType::Capture => false,
            _ => true,
        }
    }
}

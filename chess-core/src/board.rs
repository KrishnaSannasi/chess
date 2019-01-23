use crate::pieces::{Piece as PieceType, Color};
use crate::math::Vector;
use crate::error::*;

type Piece = (PieceType, Color);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RawBoard {
    data: [[Option<Piece>; 8]; 8]
}

pub enum Diff {
    Promote(Pos, Piece),
    Add(Pos, Piece),
    Move(Pos, Pos),
    Rem(Pos),
}

impl Pos {
    pub fn new_unchecked(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn new(x: usize, y: usize) -> Result<Self, OutOfBounds> {
        if x < 8 &&
            y < 8 {
            Ok(Self(x, y))
        } else {
            Err(OutOfBounds)
        }
    }

    pub fn try_from(Vector { x, y }: Vector) -> Result<Self, OutOfBounds> {
        if x >= 0 &&
            y >= 0 &&
            x < 8 &&
            y < 8 {
            Ok(Self(x as usize, y as usize))
        } else {
            Err(OutOfBounds)
        }
    }

    pub fn into(self) -> Vector {
        Vector {
            x: self.0 as i32,
            y: self.1 as i32
        }
    }
}

impl RawBoard {
    fn set(&mut self, Pos(x, y): Pos, piece: PieceType, color: Color) {
        self.data[x][y] = Some((piece, color));
    }

    fn replace(&mut self, Pos(x, y): Pos, piece: Option<Piece>) -> Option<Piece> {
        std::mem::replace(&mut self.data[x][y], piece)
    }
    
    fn remove(&mut self, Pos(x, y): Pos) -> Option<Piece> {
        self.data[x][y].take()
    }

    fn get(&self, Pos(x, y): Pos) -> Result<Piece, Error> {
        self.data[x][y].ok_or(Error::NoPiece)
    }

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Pos, PieceType, Color)> {
        self.data.iter()
                 .enumerate()
                 .flat_map(move |(x, col)| {
                     col.iter()
                        .enumerate()
                        .flat_map(move |(y, &piece)| Some((Pos(x, y), piece?)))
                        .map(move |(pos, (pt, color))| (pos, pt, color))
                 })
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl 'a + Iterator<Item = (Pos, &mut PieceType, &mut Color)> {
        self.data.iter_mut()
                 .enumerate()
                 .flat_map(move |(x, col)| {
                     col.iter_mut()
                        .enumerate()
                        .flat_map(move |(y, piece)| Some((Pos(x, y), piece.as_mut()?)))
                        .map(move |(pos, (pt, color))| (pos, pt, color))
                 })
    }
}

pub struct Board {
    board: RawBoard
}

impl Board {
    pub fn new() -> Self {
        let mut board = RawBoard::default();

        board.set(Pos(0, 0), PieceType::Rook, Color::White);
        board.set(Pos(7, 0), PieceType::Rook, Color::White);

        board.set(Pos(1, 0), PieceType::Knight, Color::White);
        board.set(Pos(6, 0), PieceType::Knight, Color::White);

        board.set(Pos(2, 0), PieceType::Bishop, Color::White);
        board.set(Pos(5, 0), PieceType::Bishop, Color::White);

        board.set(Pos(3, 0), PieceType::Queen, Color::White);
        board.set(Pos(4, 0), PieceType::King, Color::White);


        board.set(Pos(0, 7), PieceType::Rook, Color::Black);
        board.set(Pos(7, 7), PieceType::Rook, Color::Black);

        board.set(Pos(1, 7), PieceType::Knight, Color::Black);
        board.set(Pos(6, 7), PieceType::Knight, Color::Black);

        board.set(Pos(2, 7), PieceType::Bishop, Color::Black);
        board.set(Pos(5, 7), PieceType::Bishop, Color::Black);

        board.set(Pos(3, 7), PieceType::Queen, Color::Black);
        board.set(Pos(4, 7), PieceType::King, Color::Black);

        Self { board }
    }
}
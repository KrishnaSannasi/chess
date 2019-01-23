use crate::error::*;
use crate::math::Vector;
use crate::pieces::{Color, Piece as PieceType, VMove};

type Piece = (PieceType, Color);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos(usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RawBoard {
    data: [[Option<Piece>; 8]; 8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Diff {
    Promote(Pos, Piece),
    Move { from: Pos, to: Pos },
    Capture { from: Pos, to: Pos, cap: Pos },
}

impl Pos {
    pub fn new_unchecked(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    pub fn new(x: usize, y: usize) -> Result<Self, OutOfBounds> {
        if x < 8 && y < 8 {
            Ok(Self(x, y))
        } else {
            Err(OutOfBounds)
        }
    }

    pub fn try_from(Vector { x, y }: Vector) -> Result<Self, OutOfBounds> {
        if x >= 0 && y >= 0 && x < 8 && y < 8 {
            Ok(Self(x as usize, y as usize))
        } else {
            Err(OutOfBounds)
        }
    }

    pub fn into(self) -> Vector {
        Vector {
            x: self.0 as i32,
            y: self.1 as i32,
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
        self.data.iter().enumerate().flat_map(move |(x, col)| {
            col.iter()
                .enumerate()
                .flat_map(move |(y, &piece)| Some((Pos(x, y), piece?)))
                .map(move |(pos, (pt, color))| (pos, pt, color))
        })
    }

    pub fn iter_mut<'a>(
        &'a mut self,
    ) -> impl 'a + Iterator<Item = (Pos, &mut PieceType, &mut Color)> {
        self.data.iter_mut().enumerate().flat_map(move |(x, col)| {
            col.iter_mut()
                .enumerate()
                .flat_map(move |(y, piece)| Some((Pos(x, y), piece.as_mut()?)))
                .map(move |(pos, (pt, color))| (pos, pt, color))
        })
    }
}

pub struct Board {
    board: RawBoard,
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

    pub fn get_possible_moves(&self, pos: Pos) -> Option<Vec<Diff>> {
        let (_, color) = self.board.get(pos).ok()?;

        let mut diffs = self.get_possible_moves_unchecked(pos);

        if let Some(diffs) = &mut diffs {
            diffs.retain(move |&x| {
                let mut temp = Self { board: self.board };
                temp.apply(x);
                !temp.is_king_check(color)
            });
        }

        diffs
    }

    /**
     * gets all possible moves, uses a closure to handle the case of a `King`
     */
    #[allow(clippy::single_match)]
    pub fn get_possible_moves_unchecked(&self, pos: Pos) -> Option<Vec<Diff>> {
        let mut moves = Vec::new();

        let (pt, color) = self.board.get(pos).ok()?;
        let old_pos = pos;
        let pos = pos.into();
        let dir = color.dir();

        for &VMove(_, del, ty, dist) in pt.get_moves() {
            let del = del * dir;
            let mut captures = false;

            moves.extend(
                (1..dist as i32)
                    .flat_map(move |dist| Pos::try_from(pos + del * dist))
                    .map(move |pos| {
                        let victim = self.board.get(pos).ok();

                        let diff = if let Some((_, v_color)) = victim {
                            if ty.is_capture() && v_color != color {
                                Some(Diff::Capture {
                                    from: old_pos,
                                    to: pos,
                                    cap: pos,
                                })
                            } else {
                                None
                            }
                        } else if ty.is_normal() {
                            Some(Diff::Move {
                                from: old_pos,
                                to: pos,
                            })
                        } else {
                            None
                        };

                        (diff, victim)
                    })
                    .take_while(move |(_, victim)| {
                        let cap = captures;
                        captures = victim.is_some();
                        cap
                    })
                    .flat_map(move |(diff, _)| diff)
                    .fuse(),
            )
        }

        match pt {
            PieceType::Pawn => {}
            PieceType::King => {}
            _ => (), // intentionally unimplemented, other pieces don't need special casing
        }

        Some(moves)
    }

    pub fn apply(&mut self, diff: Diff) {}

    fn is_king_check(&self, color: Color) -> bool {
        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

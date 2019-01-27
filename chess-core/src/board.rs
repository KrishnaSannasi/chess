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
pub enum DiffType {
    Promote { piece: PieceType },
    Capture { cap: Pos },
    Move,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Diff {
    ty: DiffType,
    from: Pos,
    to: Pos
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameCondition {
    Safe,
    Stale,
    Check,
    Mate
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
        self.data[y][x] = Some((piece, color));
    }

    fn replace(&mut self, Pos(x, y): Pos, piece: Option<Piece>) -> Option<Piece> {
        std::mem::replace(&mut self.data[y][x], piece)
    }

    fn remove(&mut self, Pos(x, y): Pos) -> Option<Piece> {
        self.data[y][x].take()
    }

    fn get(&self, Pos(x, y): Pos) -> Result<Piece, Error> {
        self.data[y][x].ok_or(Error::NoPiece)
    }

    pub fn iter<'a>(&'a self) -> impl 'a + Iterator<Item = (Pos, PieceType, Color)> {
        self.data.iter().enumerate().flat_map(move |(x, col)| {
            col.iter()
                .enumerate()
                .flat_map(move |(y, &piece)| Some((Pos(y, x), piece?)))
                .map(move |(pos, (pt, color))| (pos, pt, color))
        })
    }

    pub fn iter_mut<'a>(
        &'a mut self,
    ) -> impl 'a + Iterator<Item = (Pos, &mut PieceType, &mut Color)> {
        self.data.iter_mut().enumerate().flat_map(move |(x, col)| {
            col.iter_mut()
                .enumerate()
                .flat_map(move |(y, piece)| Some((Pos(y, x), piece.as_mut()?)))
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

        for i in 0..8 {
            board.set(Pos(i, 1), PieceType::Pawn, Color::White);
            board.set(Pos(i, 6), PieceType::Pawn, Color::Black);
        }

        Self { board }
    }

    pub fn with(board: RawBoard) -> Self {
        Self { board }
    }

    pub fn get(&self, pos: Pos) -> Result<Piece, Error> {
        self.board.get(pos)
    }

    /**
     * gets all possible moves for the selected piece, check if
     * the king will be put in check and if so, that move will be skipped
     */
    pub fn get_possible_moves<'a>(&'a self, pos: Pos) -> Option<impl 'a + Iterator<Item = Diff>> {
        let (_, color) = self.board.get(pos).ok()?;

        let diffs = self.get_possible_moves_unchecked(pos);

        diffs.map(move |diffs| {
            diffs.filter(move |&x| {
                let mut temp = Self { board: self.board };
                temp.apply(x).unwrap();
                !temp.is_king_check(color)
            })
        })
    }

    /**
     * gets all possible moves, don't check if the king will be put in check
     */
    #[allow(clippy::single_match)]
    pub fn get_possible_moves_unchecked<'a>(&'a self, pos: Pos) -> Option<impl 'a + Iterator<Item = Diff>> {
        let (pt, color) = self.board.get(pos).ok()?;
        let old_pos = pos;
        let pos = pos.into();
        let dir = color.dir();

        let moves = pt.get_moves();
        let moves = moves.iter()
            .map(move |&VMove(_, del, ty, dist)| {
                let del = del * dir;
                (del, ty, dist as i32)
            })
            .flat_map(move |(del, ty, dist)| {
                let mut no_captures = true;

                (1..=dist)
                    .flat_map(move |dist| Pos::try_from(pos + del * dist))
                    .map(move |pos| {
                        let victim = self.board.get(pos).ok();

                        let diff = if let Some((_, v_color)) = victim {
                            if ty.is_capture() && v_color != color {
                                Some(Diff {
                                    from: old_pos,
                                    to: pos,
                                    ty: DiffType::Capture {
                                        cap: pos
                                    }
                                })
                            } else {
                                None
                            }
                        } else if ty.is_normal() {
                            Some(Diff {
                                ty: DiffType::Move,
                                from: old_pos,
                                to: pos,
                            })
                        } else {
                            None
                        };

                        (diff, victim)
                    })
                    .take_while(move |(_, victim)| {
                        let no_cap = no_captures;
                        no_captures = victim.is_none();
                        no_cap
                    })
                    .flat_map(move |(diff, _)| diff)
                    .fuse()
            });

        match pt {
            PieceType::Pawn => {}
            PieceType::King => {}
            _ => (), // intentionally unimplemented, other pieces don't need special casing
        }

        Some(moves)
    }

    /**
     * Checks and applies a Diff to the current state of the Board
     */
    pub fn apply(&mut self, Diff { ty, from, to }: Diff) -> Result<(), Error> {
        match ty {
            DiffType::Move => {
                let (piece, color) = self.board.remove(from).ok_or(Error::NoPiece)?;
                
                if self.board.get(to).is_ok() {
                    Err(InvalidDiff::CaptureOnMoveTy)?;
                }

                self.board.set(to, piece, color);
            }
            DiffType::Capture { cap } => {
                let (piece, color) = self.board.remove(from).ok_or(Error::NoPiece)?;
                
                if self.board.replace(cap, None).is_none() {
                    Err(InvalidDiff::MoveOnCaptureTy)?;
                }
                
                self.board.set(to, piece, color);
            }
            DiffType::Promote { piece } => {
                match self.board.replace(from, None) {
                    Some((PieceType::Pawn, color)) => {
                        let row = (1 - color.dir()) / 2 * 5 + 1; // choose 1 and 6
                        let prom = (1 - color.dir()) / 2 * 8; // choose 0 and 8

                        let from = from.into();
                        let v_to = to.into();
                        if from.y == row && v_to.y == prom {
                            self.board.set(to, piece, color)
                        } else {
                            Err(InvalidDiff::InvalidPromotionRow)?
                        }
                    },
                    Some(_) => Err(InvalidDiff::InvalidPromotionPiece)?,
                    None => Err(Error::NoPiece)?
                }
            }
        }

        Ok(())
    }

    /**
     * This checks if the king of the given color is in check,
     * i.e. is being attacked by an enemy piece
     */
    fn is_king_check(&self, color: Color) -> bool {
        self.board.iter()
            .filter(move |(_, _, c)| c != &color)
            .flat_map(move |(pos, _, _)| {
                self.get_possible_moves_unchecked(pos).unwrap()
                    .flat_map(move |Diff { to, .. }| self.get(to))
            })
            .any(move |(pt, c)| pt == PieceType::King && c == color)
    }

    /**
     * This checks the condition of the game
     * 
     * Check => King is being attacked, but can escape or remove the attacker
     * Mate => King is being attacked with no way to stop it
     * Safe => King is not being attacked, and some piece of the given color can move
     * Stale => King is not being attacked, and no piece of the given color can move
     */
    pub fn game_condition(&self, color: Color) -> GameCondition {
        let has_moves = self.board.iter()
                .filter(move |(_, _, c)| c == &color)
                .flat_map(move |(pos, _, _)| {
                    self.get_possible_moves(pos).unwrap()
                })
                .any(move |_| true);
        
        let is_king_check = self.is_king_check(color);

        match (is_king_check, has_moves) {
            (true, true) => GameCondition::Check,
            (true, false) => GameCondition::Mate,
            (false, true) => GameCondition::Safe,
            (false, false) => GameCondition::Stale,
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

mod fmt {
    use super::*;
    use std::fmt;

    impl fmt::Debug for Board {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            for col in self.board.data.iter().rev() {
                for &tile in col {
                    match tile {
                        Some((pt, _)) => write!(f, "{}. ", pt.get_ident())?,
                        None => write!(f, "__ ")?
                    }
                }
                
                writeln!(f)?;

                for &tile in col {
                    match tile {
                        Some((_, color)) => write!(f, ".{} ", color.get_ident())?,
                        None => write!(f, "__ ")?
                    }
                }

                writeln!(f)?;
            }

            Ok(())
        }
    }
}

#[cfg(test)]
mod test {
    use crate::board::*;

    macro_rules! pos {
        ($x:expr, $y:expr) => { Pos::new_unchecked($x, $y) };
    }

    macro_rules! poss_move_u {
        ($board:expr, $x:expr, $y:expr) => { $board.get_possible_moves_unchecked(pos!($x, $y)).unwrap().collect::<Vec<_>>() };
    }

    macro_rules! make_board {
        (
            $(($($rest:tt)*))*
        ) => {{
            #[allow(unused_mut)]
            let mut board = RawBoard { data: [[None; 8]; 8] };

            $(
                make_board!(@internal board $($rest)*);
            )*

            dbg!(Board::with(board))
        }};

        (@internal $board:ident ($x:expr, $y:expr) $color:ident $piece:ident) => {
            $board.set(Pos($x, $y), PieceType::$piece, Color::$color);
        };
    }

    #[test]
    fn gpmu_pass_1() {
        let board = Board::new();

        let moves = poss_move_u!(board, 0, 1);

        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Diff { ty: DiffType::Move, from: pos!(0, 1), to: pos!(0, 2) }));
        assert!(moves.contains(&Diff { ty: DiffType::Move, from: pos!(0, 1), to: pos!(0, 3) }));
    }

    #[test]
    fn gpmu_pass_2() {
        let board = Board::new();

        let moves = poss_move_u!(board, 1, 0);

        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&Diff { ty: DiffType::Move, from: pos!(1, 0), to: pos!(0, 2) }));
        assert!(moves.contains(&Diff { ty: DiffType::Move, from: pos!(1, 0), to: pos!(2, 2) }));
    }

    #[test]
    fn gpmu_pass_3() {
        let board = Board::new();

        let moves = poss_move_u!(board, 0, 0);
        assert!(moves.is_empty());

        let moves = poss_move_u!(board, 2, 0);
        assert!(moves.is_empty());

        let moves = poss_move_u!(board, 3, 0);
        assert!(moves.is_empty());

        let moves = poss_move_u!(board, 4, 0);
        assert!(moves.is_empty());
    }

    

    #[test]
    fn gc_pass_1() {
        let board = make_board!();

        assert!(!board.is_king_check(Color::White));
        assert!(!board.is_king_check(Color::Black));
    }

    #[test]
    fn gc_pass_2() {
        let board = make_board!(
            ((0, 0) White King)
            ((0, 2) Black Rook)
        );

        assert!(board.is_king_check(Color::White));
        assert!(!board.is_king_check(Color::Black));
    }

    #[test]
    fn gc_pass_3() {
        let mut board = RawBoard { data: [[None; 8]; 8] };

        board.data[0][0] = Some((PieceType::King, Color::White));
        board.data[1][0] = Some((PieceType::Pawn, Color::Black));
        board.data[7][0] = Some((PieceType::Rook, Color::Black));

        let board = make_board!(
            ((0, 0) White King)
            ((0, 1) Black Pawn)
            ((0, 7) Black Rook)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Safe);
    }

    #[test]
    fn gc_pass_4() {
        let board = make_board!(
            ((0, 0) White King)
            ((0, 1) White Pawn)
            ((0, 7) Black Rook)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Safe);
    }

    #[test]
    fn gc_pass_5() {
        let board = make_board!(
            ((0, 0) White King)
            ((0, 1) White Pawn)
            ((0, 7) Black Rook)
            ((1, 7) Black Queen)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Safe);
    }

    #[test]
    fn gc_pass_6() {
        let board = make_board!(
            ((0, 0) White King)
            ((0, 7) Black Rook)
            ((1, 7) Black Queen)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Mate);
    }

    #[test]
    fn gc_pass_7() {
        let board = make_board!(
            ((1, 0) White King)
            ((0, 7) Black Rook)
            ((1, 7) Black Queen)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Check);
    }

    #[test]
    fn gc_pass_8() {
        let board = make_board!(
            ((0, 0) White King)
            ((0, 7) White Queen)
            ((0, 6) Black Rook)
            ((1, 6) Black Rook)
        );

        assert_eq!(board.game_condition(Color::White), GameCondition::Check);
    }
}

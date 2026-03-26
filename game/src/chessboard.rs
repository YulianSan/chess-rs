#![allow(warnings)]

use std::array;

use crate::pieces::{ChessPiece, Piece};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Color {
    White,
    Black,
}

pub type Position = (u8, u8);

pub struct Place {
    pub position: Position,
    pub color: Color,
    pub chess_piece: Option<ChessPiece>,
}

impl Place {
    pub fn get_name(&self) -> &'static str {
        self.chess_piece.as_ref().unwrap().piece.get_name()
    }
}

pub struct Chessboard {
    board: [[Place; 8]; 8],
}

impl Chessboard {
    pub fn new() -> Self {
        Self {
            board: array::from_fn(|i| {
                array::from_fn(|j| Place {
                    position: (i as u8, j as u8),
                    color: if (i + j) % 2 == 0 {
                        Color::Black
                    } else {
                        Color::White
                    },
                    chess_piece: ChessPiece::has_piece_in_position((i as u8, j as u8)),
                })
            }),
        }
    }

    pub fn has_piece(&self, position: Position, color: Color) -> bool {
        self.board[position.0 as usize][position.1 as usize]
            .chess_piece
            .is_some()
            && self.board[position.0 as usize][position.1 as usize]
                .chess_piece
                .as_ref()
                .unwrap()
                .color
                == color
    }

    pub fn possible_moves(&self, position: Position) -> Vec<Position> {
        let place = &self.board[position.0 as usize][position.1 as usize];

        if place.chess_piece.is_none() {
            return vec![];
        }

        place
            .chess_piece
            .as_ref()
            .unwrap()
            .piece
            .generate_move(place, self)
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::*;

    use super::*;

    macro_rules! piece {
        (Kb) => {
            Some(ChessPiece {
                piece: Box::new(King),
                color: Color::Black,
            })
        };
        (qb) => {
            Some(ChessPiece {
                piece: Box::new(Queen),
                color: Color::Black,
            })
        };
        (bb) => {
            Some(ChessPiece {
                piece: Box::new(Bishop),
                color: Color::Black,
            })
        };
        (kb) => {
            Some(ChessPiece {
                piece: Box::new(Knight),
                color: Color::Black,
            })
        };
        (rb) => {
            Some(ChessPiece {
                piece: Box::new(Rook),
                color: Color::Black,
            })
        };
        (pb) => {
            Some(ChessPiece {
                piece: Box::new(Pawn),
                color: Color::Black,
            })
        };
        (Kb) => {
            Some(ChessPiece {
                piece: Box::new(King),
                color: Color::Black,
            })
        };
        (qw) => {
            Some(ChessPiece {
                piece: Box::new(Queen),
                color: Color::White,
            })
        };
        (bw) => {
            Some(ChessPiece {
                piece: Box::new(Bishop),
                color: Color::White,
            })
        };
        (kw) => {
            Some(ChessPiece {
                piece: Box::new(Knight),
                color: Color::White,
            })
        };
        (rw) => {
            Some(ChessPiece {
                piece: Box::new(Rook),
                color: Color::White,
            })
        };
        (pw) => {
            Some(ChessPiece {
                piece: Box::new(Pawn),
                color: Color::White,
            })
        };
        (.) => {
            None
        };
    }

    macro_rules! row {
        ([$p0:tt, $p1:tt, $p2:tt, $p3:tt, $p4:tt, $p5:tt, $p6:tt, $p7:tt], $row:expr) => {
            [
                Place {
                    position: ($row, 0),
                    color: Color::Black,
                    chess_piece: piece!($p0),
                },
                Place {
                    position: ($row, 1),
                    color: Color::Black,
                    chess_piece: piece!($p1),
                },
                Place {
                    position: ($row, 2),
                    color: Color::Black,
                    chess_piece: piece!($p2),
                },
                Place {
                    position: ($row, 3),
                    color: Color::Black,
                    chess_piece: piece!($p3),
                },
                Place {
                    position: ($row, 4),
                    color: Color::Black,
                    chess_piece: piece!($p4),
                },
                Place {
                    position: ($row, 5),
                    color: Color::Black,
                    chess_piece: piece!($p5),
                },
                Place {
                    position: ($row, 6),
                    color: Color::Black,
                    chess_piece: piece!($p6),
                },
                Place {
                    position: ($row, 7),
                    color: Color::Black,
                    chess_piece: piece!($p7),
                },
            ]
        };
    }

    macro_rules! chessboard {
        (
            $r0:tt,
            $r1:tt,
            $r2:tt,
            $r3:tt,
            $r4:tt,
            $r5:tt,
            $r6:tt,
            $r7:tt
        ) => {
            Chessboard {
                board: [
                    row!($r0, 0),
                    row!($r1, 1),
                    row!($r2, 2),
                    row!($r3, 3),
                    row!($r4, 4),
                    row!($r5, 5),
                    row!($r6, 6),
                    row!($r7, 7),
                ],
            }
        };
    }

    #[test]
    fn should_create_new_chessboard() {
        let chessboard = Chessboard::new();
        assert_eq!(chessboard.board[0][0].color, Color::Black);
        assert_eq!(chessboard.board[0][0].get_name(), "rook");
        assert_eq!(chessboard.board[0][0].position, (0, 0));

        assert_eq!(chessboard.board[0][1].color, Color::White);
        assert_eq!(chessboard.board[0][1].get_name(), "knight");
        assert_eq!(chessboard.board[0][1].position, (0, 1));

        assert_eq!(chessboard.board[0][2].color, Color::Black);
        assert_eq!(chessboard.board[0][2].get_name(), "bishop");
        assert_eq!(chessboard.board[0][2].position, (0, 2));

        assert_eq!(chessboard.board[0][3].color, Color::White);
        assert_eq!(chessboard.board[0][3].get_name(), "queen");
        assert_eq!(chessboard.board[0][3].position, (0, 3));

        assert_eq!(chessboard.board[0][4].color, Color::Black);
        assert_eq!(chessboard.board[0][4].get_name(), "king");
        assert_eq!(chessboard.board[0][4].position, (0, 4));

        assert_eq!(chessboard.board[0][5].color, Color::White);
        assert_eq!(chessboard.board[0][5].get_name(), "bishop");
        assert_eq!(chessboard.board[0][5].position, (0, 5));

        assert_eq!(chessboard.board[0][6].color, Color::Black);
        assert_eq!(chessboard.board[0][6].get_name(), "knight");
        assert_eq!(chessboard.board[0][6].position, (0, 6));

        assert_eq!(chessboard.board[0][7].color, Color::White);
        assert_eq!(chessboard.board[0][7].get_name(), "rook");
        assert_eq!(chessboard.board[0][7].position, (0, 7));

        assert_eq!(chessboard.board[1][0].color, Color::White);
        assert_eq!(chessboard.board[1][0].get_name(), "pawn");

        assert_eq!(chessboard.board[1][1].color, Color::Black);
        assert_eq!(chessboard.board[1][1].get_name(), "pawn");

        assert_eq!(chessboard.board[1][2].color, Color::White);
        assert_eq!(chessboard.board[1][2].get_name(), "pawn");

        assert_eq!(chessboard.board[1][3].color, Color::Black);
        assert_eq!(chessboard.board[1][3].get_name(), "pawn");

        assert_eq!(chessboard.board[1][4].color, Color::White);
        assert_eq!(chessboard.board[1][4].get_name(), "pawn");

        assert_eq!(chessboard.board[1][5].color, Color::Black);
        assert_eq!(chessboard.board[1][5].get_name(), "pawn");

        assert_eq!(chessboard.board[1][6].color, Color::White);
        assert_eq!(chessboard.board[1][6].get_name(), "pawn");

        assert_eq!(chessboard.board[1][7].color, Color::Black);
        assert_eq!(chessboard.board[1][7].get_name(), "pawn");

        assert_eq!(chessboard.board[2][0].color, Color::Black);
        assert_eq!(chessboard.board[2][1].color, Color::White);
        assert_eq!(chessboard.board[2][2].color, Color::Black);
        assert_eq!(chessboard.board[2][3].color, Color::White);
        assert_eq!(chessboard.board[2][4].color, Color::Black);
        assert_eq!(chessboard.board[2][5].color, Color::White);
        assert_eq!(chessboard.board[2][6].color, Color::Black);
        assert_eq!(chessboard.board[2][7].color, Color::White);

        assert_eq!(chessboard.board[3][0].color, Color::White);
        assert_eq!(chessboard.board[3][1].color, Color::Black);
        assert_eq!(chessboard.board[3][2].color, Color::White);
        assert_eq!(chessboard.board[3][3].color, Color::Black);
        assert_eq!(chessboard.board[3][4].color, Color::White);
        assert_eq!(chessboard.board[3][5].color, Color::Black);
        assert_eq!(chessboard.board[3][6].color, Color::White);
        assert_eq!(chessboard.board[3][7].color, Color::Black);

        assert_eq!(chessboard.board[4][0].color, Color::Black);
        assert_eq!(chessboard.board[4][1].color, Color::White);
        assert_eq!(chessboard.board[4][2].color, Color::Black);
        assert_eq!(chessboard.board[4][3].color, Color::White);
        assert_eq!(chessboard.board[4][4].color, Color::Black);
        assert_eq!(chessboard.board[4][5].color, Color::White);
        assert_eq!(chessboard.board[4][6].color, Color::Black);
        assert_eq!(chessboard.board[4][7].color, Color::White);

        assert_eq!(chessboard.board[5][0].color, Color::White);
        assert_eq!(chessboard.board[5][1].color, Color::Black);
        assert_eq!(chessboard.board[5][2].color, Color::White);
        assert_eq!(chessboard.board[5][3].color, Color::Black);
        assert_eq!(chessboard.board[5][4].color, Color::White);
        assert_eq!(chessboard.board[5][5].color, Color::Black);
        assert_eq!(chessboard.board[5][6].color, Color::White);
        assert_eq!(chessboard.board[5][7].color, Color::Black);

        assert_eq!(chessboard.board[6][0].color, Color::Black);
        assert_eq!(chessboard.board[6][0].get_name(), "pawn");

        assert_eq!(chessboard.board[6][1].color, Color::White);
        assert_eq!(chessboard.board[6][1].get_name(), "pawn");

        assert_eq!(chessboard.board[6][2].color, Color::Black);
        assert_eq!(chessboard.board[6][2].get_name(), "pawn");

        assert_eq!(chessboard.board[6][3].color, Color::White);
        assert_eq!(chessboard.board[6][3].get_name(), "pawn");

        assert_eq!(chessboard.board[6][4].color, Color::Black);
        assert_eq!(chessboard.board[6][4].get_name(), "pawn");

        assert_eq!(chessboard.board[6][5].color, Color::White);
        assert_eq!(chessboard.board[6][5].get_name(), "pawn");

        assert_eq!(chessboard.board[6][6].color, Color::Black);
        assert_eq!(chessboard.board[6][6].get_name(), "pawn");

        assert_eq!(chessboard.board[6][7].color, Color::White);
        assert_eq!(chessboard.board[6][7].get_name(), "pawn");

        assert_eq!(chessboard.board[7][0].color, Color::White);
        assert_eq!(chessboard.board[7][0].get_name(), "rook");
        assert_eq!(chessboard.board[7][0].position, (7, 0));

        assert_eq!(chessboard.board[7][1].color, Color::Black);
        assert_eq!(chessboard.board[7][1].get_name(), "knight");
        assert_eq!(chessboard.board[7][1].position, (7, 1));

        assert_eq!(chessboard.board[7][2].color, Color::White);
        assert_eq!(chessboard.board[7][2].get_name(), "bishop");
        assert_eq!(chessboard.board[7][2].position, (7, 2));

        assert_eq!(chessboard.board[7][3].color, Color::Black);
        assert_eq!(chessboard.board[7][3].get_name(), "queen");
        assert_eq!(chessboard.board[7][3].position, (7, 3));

        assert_eq!(chessboard.board[7][4].color, Color::White);
        assert_eq!(chessboard.board[7][4].get_name(), "king");
        assert_eq!(chessboard.board[7][4].position, (7, 4));

        assert_eq!(chessboard.board[7][5].color, Color::Black);
        assert_eq!(chessboard.board[7][5].get_name(), "bishop");
        assert_eq!(chessboard.board[7][5].position, (7, 5));

        assert_eq!(chessboard.board[7][6].color, Color::White);
        assert_eq!(chessboard.board[7][6].get_name(), "knight");
        assert_eq!(chessboard.board[7][6].position, (7, 6));

        assert_eq!(chessboard.board[7][7].color, Color::Black);
        assert_eq!(chessboard.board[7][7].get_name(), "rook");
        assert_eq!(chessboard.board[7][7].position, (7, 7));
    }

    #[test]
    fn has_piece_by_position_and_color() {
        let chessboard = Chessboard::new();

        assert!(chessboard.has_piece((0, 0), Color::White));
        assert!(!chessboard.has_piece((0, 0), Color::Black));

        assert!(!chessboard.has_piece((4, 4), Color::White));
        assert!(!chessboard.has_piece((4, 4), Color::Black));

        assert!(!chessboard.has_piece((7, 7), Color::White));
        assert!(chessboard.has_piece((7, 7), Color::Black));
    }

    #[test]
    fn possible_moves_king() {
        let chessboard = Chessboard::new();
        let possible_moves = chessboard.possible_moves((0, 4));
        assert_eq!(possible_moves, vec![]);

        let chessboard = chessboard!(
            [Kb, ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .]
        );
        let possible_moves = chessboard.possible_moves((0, 0));
        assert_eq!(possible_moves, vec![(1, 1), (1, 0), (0, 1)]);

        let chessboard = chessboard!(
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., Kb, ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .]
        );

        let possible_moves = chessboard.possible_moves((3, 1));
        assert_eq!(
            possible_moves,
            vec![
                (4, 2),
                (4, 1),
                (3, 2),
                (2, 0),
                (2, 1),
                (3, 0),
                (4, 0),
                (2, 2)
            ]
        );

        let chessboard = chessboard!(
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [kb, kb, pb, ., ., ., ., .],
            [kb, Kb, pb, ., ., ., ., .],
            [., kb, pb, ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .]
        );

        let possible_moves = chessboard.possible_moves((3, 1));
        assert_eq!(possible_moves, vec![(4, 0)]);
    }

    #[test]
    fn possible_moves_queen() {
        let chessboard = Chessboard::new();
        let possible_moves = chessboard.possible_moves((0, 4));
        assert_eq!(possible_moves, vec![]);

        let chessboard = chessboard!(
            [., qb, ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .]
        );
        let possible_moves = chessboard.possible_moves((0, 1));
        assert_eq!(
            possible_moves,
            vec![
                (1, 2),
                (1, 1),
                (0, 2),
                (0, 0),
                (1, 0),
                (2, 3),
                (2, 1),
                (0, 3),
                (3, 4),
                (3, 1),
                (0, 4),
                (4, 5),
                (4, 1),
                (0, 5),
                (5, 6),
                (5, 1),
                (0, 6),
                (6, 7),
                (6, 1),
                (0, 7),
                (7, 1)
            ]
        );

        let chessboard = chessboard!(
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [kb, kb, pb, ., ., ., ., .],
            [kb, qb, pb, ., ., ., ., .],
            [., kb, pb, ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .],
            [., ., ., ., ., ., ., .]
        );

        let possible_moves = chessboard.possible_moves((3, 1));
        assert_eq!(possible_moves, vec![(4, 0)]);

        // let chessboard = chessboard!(
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., Kb, ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .]
        // );
        //
        // let possible_moves = chessboard.possible_moves((3, 1));
        // assert_eq!(possible_moves, vec![(4, 2), (4, 1), (3, 2), (2, 0), (2, 1), (3, 0), (4, 0), (2, 2)]);
        //
        // let chessboard = chessboard!(
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [kb, kb, pb, ., ., ., ., .],
        //     [kb, Kb, pb, ., ., ., ., .],
        //     [., kb, pb, ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .],
        //     [., ., ., ., ., ., ., .]
        // );
        //
        // let possible_moves = chessboard.possible_moves((3, 1));
        // assert_eq!(possible_moves, vec![(4, 0)]);
    }
}

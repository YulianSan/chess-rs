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

    pub fn possible_moves(&self, position: Position) -> Vec<Position> {
        &self.board[position.0 as usize][position.1 as usize].chess_piece.as_ref().unwrap();
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use crate::pieces::*;

    use super::*;

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
}

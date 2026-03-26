#![allow(warnings)]

use std::fmt::Debug;

use crate::chessboard::{Chessboard, Color, Place, Position};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct King;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Queen;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Bishop;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Knight;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rook;
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pawn;

pub trait Piece {
    fn is_initial_position(&self, position: Position) -> bool;
    fn get_name(&self) -> &'static str;
    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position>;
}

impl Piece for King {
    fn is_initial_position(&self, position: Position) -> bool {
        if position == (0, 4) || position == (7, 4) {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "king"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        if place.chess_piece.is_none() {
            return vec![];
        }

        let position = place.position;
        let color = place.chess_piece.as_ref().unwrap().color;

        vec![
            (position.0 + 1, position.1 + 1),
            (position.0 + 1, position.1),
            (position.0, position.1 + 1),
            (
                position.0.checked_sub(1).unwrap_or(8),
                position.1.checked_sub(1).unwrap_or(8),
            ),
            (position.0.checked_sub(1).unwrap_or(8), position.1),
            (position.0, position.1.checked_sub(1).unwrap_or(8)),
            (position.0 + 1, position.1.checked_sub(1).unwrap_or(8)),
            (position.0.checked_sub(1).unwrap_or(8), position.1 + 1),
        ]
        .iter()
        .filter(|p| p.0 <= 7 && p.1 <= 7 && !chessboard.has_piece(**p, color))
        .map(|f| *f)
        .collect::<Vec<Position>>()
    }
}

impl Piece for Queen {
    fn is_initial_position(&self, position: Position) -> bool {
        if position == (0, 3) || position == (7, 3) {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "queen"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        if place.chess_piece.is_none() {
            return vec![];
        }

        let position = place.position;
        let color = place.chess_piece.as_ref().unwrap().color;
        let mut moves = vec![];

        for i in 0..=7 {
            moves.extend(
                vec![
                    (position.0 + i, position.1 + i),
                    (position.0 + i, position.1),
                    (position.0, position.1 + i),
                    (position.0.checked_sub(i).unwrap_or(8), position.1),
                    (position.0, position.1.checked_sub(i).unwrap_or(8)),
                    (position.0 + i, position.1.checked_sub(i).unwrap_or(8)),
                    (position.0.checked_sub(i).unwrap_or(8), position.1 + i),
                ]
                .iter()
                .filter(|p| p.0 <= 7 && p.1 <= 7 && !chessboard.has_piece(**p, color))
                .map(|f| *f),
            );
        }

        moves
    }
}

impl Piece for Bishop {
    fn is_initial_position(&self, position: Position) -> bool {
        if position == (0, 2) || position == (7, 5) || position == (0, 5) || position == (7, 2) {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "bishop"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        todo!()
    }
}

impl Piece for Knight {
    fn is_initial_position(&self, position: Position) -> bool {
        if position == (0, 1) || position == (7, 6) || position == (0, 6) || position == (7, 1) {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "knight"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        todo!()
    }
}

impl Piece for Rook {
    fn is_initial_position(&self, position: Position) -> bool {
        if position == (0, 0) || position == (7, 7) || position == (0, 7) || position == (7, 0) {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "rook"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        todo!()
    }
}

impl Piece for Pawn {
    fn is_initial_position(&self, position: Position) -> bool {
        if position.0 == 1 || position.0 == 6 {
            return true;
        }

        false
    }

    fn get_name(&self) -> &'static str {
        "pawn"
    }

    fn generate_move(&self, place: &Place, chessboard: &Chessboard) -> Vec<Position> {
        todo!()
    }
}

pub struct ChessPiece {
    pub piece: Box<dyn Piece>,
    pub color: Color,
}

impl ChessPiece {
    pub fn has_piece_in_position(position: Position) -> Option<ChessPiece> {
        let color = if position.0 < 4 {
            Color::White
        } else {
            Color::Black
        };

        let piece = Box::new(Pawn);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        let piece = Box::new(Rook);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        let piece = Box::new(Knight);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        let piece = Box::new(Bishop);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        let piece = Box::new(Queen);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        let piece = Box::new(King);
        if piece.is_initial_position(position) {
            return Some(ChessPiece { piece, color });
        }

        None
    }
}

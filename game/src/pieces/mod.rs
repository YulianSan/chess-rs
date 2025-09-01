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

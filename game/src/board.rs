#![allow(warnings)]
use std::marker::PhantomData;

use crate::moves::Move;

trait Color {}

struct White;
struct Black;

pub struct Board<c: Color> {
    white: Side,
    black: Side,
    _marker: PhantomData<c>,
}

pub struct Side {
    pawns: BitBoard,
    knights: BitBoard,
    bishops: BitBoard,
    rooks: BitBoard,
    queens: BitBoard,
    kings: BitBoard,
}

impl Side {
    pub fn all_pieces(&self) -> BitBoard {
        BitBoard(
            self.pawns.0
                | self.knights.0
                | self.bishops.0
                | self.rooks.0
                | self.queens.0
                | self.kings.0,
        )
    }
}

pub struct BitBoard(u64);

impl<C: Color> Board<C> {
    fn new() -> Self {
        Board {
            white: Side {
                pawns: BitBoard(0x00FF00000000FF00),
                rooks: BitBoard(0x8100000000000081),
                knights: BitBoard(0x4200000000000042),
                bishops: BitBoard(0x2400000000000024),
                queens: BitBoard(0x0800000000000008),
                kings: BitBoard(0x1000000000000010),
            },
            black: Side {
                pawns: BitBoard(0x00FF000000000000),
                rooks: BitBoard(0x8100000000000000),
                knights: BitBoard(0x4200000000000000),
                bishops: BitBoard(0x2400000000000000),
                queens: BitBoard(0x0800000000000000),
                kings: BitBoard(0x1000000000000000),
            },
            _marker: PhantomData,
        }
    }
}

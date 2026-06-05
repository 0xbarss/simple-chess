use std::array::from_fn;

use crate::pieces::{Color, PieceKind, Piece, Square};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub squares: [Square; 64],
    pub en_passant: i8,
    pub turn: Color
}

impl Board {
    pub fn new() -> Self {
        let squares: [Square; 64] = from_fn(
            |i| {
                let color = if i < 32 { Color::White } else { Color::Black };
                if (i > 7 && i < 16) || (i > 47 && i < 56) {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::Pawn
                    });
                }
                else if i == 0 || i == 7 || i == 56 || i == 63 {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::Rook
                    });
                }
                else if i == 1 || i == 6 || i == 57 || i == 62 {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::Knight
                    });
                }
                else if i == 2 || i == 5 || i == 58 || i == 61 {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::Bishop
                    });
                }
                else if i == 3 || i == 59 {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::Queen
                    });
                }
                else if i == 4 || i == 60 {
                    return Square::Occupied(Piece {
                        color: color,
                        kind: PieceKind::King
                    });
                }

                Square::Empty(i)
            }
        );

        Board { squares, en_passant: -1, turn: Color::White }
    }
}

impl Board {
    pub fn empty() -> Self {
        let squares = from_fn(|i| Square::Empty(i));
        Board { squares, en_passant: -1, turn: Color::White }
    }

    pub fn place_piece(
        &mut self,
        index: u8,
        color: Color,
        kind: PieceKind,
    ) {
        self.squares[index as usize] = Square::Occupied(Piece {
            color,
            kind,
        });
    }

    pub fn rotate(&mut self) {
        let squares = self.squares.clone();

        for index in 0..64 {
            let square = squares[index];
            let new_rank = 7 - index / 8;
            let new_file = index % 8;
            let new_index = new_file + new_rank * 8;
            self.squares[new_index] = square;
        }

        if self.en_passant >= 0 {
            let ep_index = self.en_passant as usize;
            let new_rank = 7 - ep_index / 8;
            let new_file = ep_index % 8;
            self.en_passant = (new_file + new_rank * 8) as i8;
        }

        self.turn = self.turn.opposite();
    }
}

use std::array::from_fn;

use crate::pieces::{Color, PieceKind, Piece, Square};

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub squares: [Square; 64],
    pub en_passant: i8, // -1: None | pos: <Square>
    pub turn: Color,
    pub castle_rights: [bool; 4], // [CurrentKingside, CurrentQueenside, OpponentKingside, OpponentQueenside]
}

impl Default for Board {
    fn default() -> Self {
        let squares: [Square; 64] = from_fn(
            |i| {
                let color = if i < 32 { Color::White } else { Color::Black };
                if (i > 7 && i < 16) || (i > 47 && i < 56) {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::Pawn
                    });
                }
                else if i == 0 || i == 7 || i == 56 || i == 63 {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::Rook
                    });
                }
                else if i == 1 || i == 6 || i == 57 || i == 62 {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::Knight
                    });
                }
                else if i == 2 || i == 5 || i == 58 || i == 61 {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::Bishop
                    });
                }
                else if i == 3 || i == 59 {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::Queen
                    });
                }
                else if i == 4 || i == 60 {
                    return Square::Occupied(Piece {
                        color,
                        kind: PieceKind::King
                    });
                }

                Square::Empty(i)
            }
        );

        Board {
            squares,
            en_passant: -1,
            turn: Color::White,
            castle_rights: [true, true, true, true],
        }
    }
}

impl Board {
    pub fn empty() -> Board {
        let squares = from_fn(Square::Empty);
        Board {
            squares,
            en_passant: -1,
            turn: Color::White,
            castle_rights: [false, false, false, false],
        }
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
        let squares = self.squares;

        for (index, square) in squares.iter().enumerate() {
            let new_rank = 7 - index / 8;
            let new_file = index % 8;
            let new_index = new_file + new_rank * 8;
            self.squares[new_index] = *square;
        }

        if self.en_passant >= 0 {
            let ep_index = self.en_passant as usize;
            let new_rank = 7 - ep_index / 8;
            let new_file = ep_index % 8;
            self.en_passant = (new_file + new_rank * 8) as i8;
        }

        let current_k = self.castle_rights[0];
        let current_q = self.castle_rights[1];
        let opp_k = self.castle_rights[2];
        let opp_q = self.castle_rights[3];
        self.castle_rights = [opp_k, opp_q, current_k, current_q];

        self.turn = self.turn.opposite();
    }
}

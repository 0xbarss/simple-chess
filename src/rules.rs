use crate::board::Board;
use crate::pieces::{Color, Square, Piece, PieceKind};
use crate::moves::{generate_pseudo_moves, Move, MoveFlag};

pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let moves = generate_pseudo_moves(board);
    let mut legal_moves = vec![];

    for mv in moves {
        let played_square = board.squares[mv.from];
        if let Square::Occupied(square) = played_square {
            if matches!(mv.flag, MoveFlag::CastleKingside | MoveFlag::CastleQueenside) {
                if board.is_in_check(square.color) {
                    continue;
                }

                if matches!(mv.flag, MoveFlag::CastleKingside) {
                    if board.square_attacked(5, square.color.opposite()) {
                        continue;
                    }
                } else {
                    if board.square_attacked(3, square.color.opposite()) {
                        continue;
                    }
                }
            }

            let mut board_test = *board;
            board_test.make_move(&mv);
            if !board_test.is_in_check(square.color) {
                legal_moves.push(mv);
            }
        }
    }

    legal_moves
}

impl Board {
    pub fn make_move(&mut self, mv: &Move) {
        let from = mv.from;
        let to = mv.to;

        let old_en_passant = self.en_passant;
        self.en_passant = -1;

        if from == 4 {
            self.castle_rights[0] = false;
            self.castle_rights[1] = false;
        } else if from == 0 {
            self.castle_rights[1] = false;
        } else if from == 7 {
            self.castle_rights[0] = false;
        }

        if to == 56 {
            self.castle_rights[3] = false;
        } else if to == 63 {
            self.castle_rights[2] = false;
        }

        match mv.flag {
            MoveFlag::Quiet => {
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
            }
            MoveFlag::DoublePawnPush => {
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
                self.en_passant = to as i8;
            }
            MoveFlag::EnPassant => {
                let en_passant_index = old_en_passant as usize;
                self.squares[en_passant_index] = Square::Empty(en_passant_index);
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
            }
            MoveFlag::Promotion(kind) => {
                if let Square::Occupied(from_square) = self.squares[from] {
                    self.squares[to] = self.squares[from];
                    self.squares[from] = Square::Empty(from);
                    self.squares[to] = Square::Occupied(Piece::new(from_square.color, kind))
                }
            }
            MoveFlag::Capture => {
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
            }
            MoveFlag::CastleKingside => {
                let king = self.squares[from];
                let rook = self.squares[to + 1];

                self.squares[to] = king;
                self.squares[from] = Square::Empty(from);

                self.squares[to - 1] = rook;
                self.squares[to + 1] = Square::Empty(to + 1);
            }
            MoveFlag::CastleQueenside => {
                let king = self.squares[from];
                let rook = self.squares[to - 2];

                self.squares[to] = king;
                self.squares[from] = Square::Empty(from);

                self.squares[to + 1] = rook;
                self.squares[to - 2] = Square::Empty(to - 2);
            }
        };
    }

    pub fn is_in_check(&self, color: Color) -> bool {
        let (_, index) = self.king_square(color);
        self.square_attacked(index, color.opposite())
    }

    pub fn king_square(&self, color: Color) -> (Square, usize) {
        for index in 0..64 {
            let square = self.squares[index];
            if let Square::Occupied(piece) = square &&
               piece.kind == PieceKind::King && piece.color == color {
                   return (square, index);
            }
        }
        panic!("King is not found!")
    }

    pub fn square_attacked(
        &self,
        target_index: usize,
        attacker: Color,
    ) -> bool {
        let is_bottom = self.turn == attacker;
        let pawn_attack_dirs = if is_bottom { [7, 9] } else { [-7, -9] };

        for dir in pawn_attack_dirs {
            if let Some(pos) = target_index.checked_add_signed(-(dir as isize)) && pos < 64 {
                let current_rank = pos / 8;
                let target_rank = target_index / 8;
                let current_file = pos % 8;
                let target_file = target_index % 8;

                if current_rank.abs_diff(target_rank) == 1 &&
                   current_file.abs_diff(target_file) == 1 &&
                   let Square::Occupied(piece) = self.squares[pos] &&
                   piece.kind == PieceKind::Pawn && piece.color == attacker {
                        return true;
                }
            }
        }

        generate_pseudo_moves(self)
            .into_iter()
            .any(|mv| {
                mv.to == target_index &&
                matches!(
                    self.squares[mv.from],
                    Square::Occupied(piece) if piece.color == attacker
                )
            })
    }
}

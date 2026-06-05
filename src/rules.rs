use crate::board::Board;
use crate::pieces::{Color, Square, Piece, PieceKind};
use crate::moves::{generate_pseudo_moves, Move, MoveFlag};

pub fn generate_legal_moves(board: &Board) -> Vec<Move> {
    let moves = generate_pseudo_moves(board);
    let mut legal_moves = vec![];

    for mv in moves {
        let played_square = board.squares[mv.from as usize];
        if let Square::Occupied(square) = played_square {
            let mut board_test = board.clone();
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

        match mv.flag {
            MoveFlag::Quiet => {
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
            }
            MoveFlag::DoublePawnPush => {
                self.squares[to] = self.squares[from];
                self.squares[from] = Square::Empty(from);
            }
            MoveFlag::EnPassant => {
                let en_passant_index = self.en_passant as usize;
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
        let (king, index) = self.king_square(color);
        self.square_attacked(king, index, color.opposite())
    }

    pub fn king_square(&self, color: Color) -> (Square, usize) {
        for index in 0..64 {
            let square = self.squares[index];
            if let Square::Occupied(piece) = square {
                if piece.kind == PieceKind::King && piece.color == color {
                    return (square, index);
                }
            }
        }
        panic!("King is not found!")
    }

    fn square_attacked(
        &self,
        _target: Square,
        target_index: usize,
        attacker: Color,
    ) -> bool {
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

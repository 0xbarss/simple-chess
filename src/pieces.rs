#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    White,
    Black
}

impl Color {
    pub fn opposite(&self) -> Color {
        if self == &Color::White { Color::Black } else { Color::White }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Piece {
    pub color: Color,
    pub kind: PieceKind
}

impl Piece {
    pub fn new(color: Color, kind: PieceKind) -> Piece {
        Piece { color, kind }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Square {
    Occupied(Piece),
    Empty(usize)
}

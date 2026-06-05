use macroquad::math::{Rect, vec2};
use macroquad::color::WHITE;
use macroquad::texture::{DrawTextureParams, draw_texture_ex};

use super::Renderer;
use crate::board::Board;
use crate::pieces::{Color, PieceKind, Square};
use crate::render::board::{SQUARE_SIZE, PIECE_SIZE};

const SPRITE_WIDTH: f32 = 64.0;
const SPRITE_HEIGHT: f32 = 64.0;

impl Renderer<'_> {
    pub fn draw_pieces(&self, board: &Board) {
        for i in 0..64 {
            let square = board.squares[i];
            if let Square::Occupied(piece) = square {
                let offset = (SQUARE_SIZE - PIECE_SIZE) / 2.0;

                let rank = i / 8;
                let file = i % 8;

                let x = file as f32 * SQUARE_SIZE + offset;
                let y = (7 - rank) as f32 * SQUARE_SIZE + offset;

                let is_white = piece.color == Color::White;
                let row = if is_white { 1 } else { 0 };

                match piece.kind {
                    PieceKind::Rook => self.draw_piece_sprite(x, y, 0, row),
                    PieceKind::Knight => self.draw_piece_sprite(x, y, 1, row),
                    PieceKind::Bishop => self.draw_piece_sprite(x, y, 2, row),
                    PieceKind::Queen => self.draw_piece_sprite(x, y, 3, row),
                    PieceKind::King => self.draw_piece_sprite(x, y, 4, row),
                    PieceKind::Pawn => self.draw_piece_sprite(x, y - offset, 5, row),
                }
            }
        }
    }

    fn draw_piece_sprite(
        &self,
        x: f32,
        y: f32,
        col: u32,
        row: u32,
    ) {
        draw_texture_ex(
            self.texture,
            x,
            y,
            WHITE,
            DrawTextureParams {
                source: Some(Rect::new(
                    col as f32 * SPRITE_WIDTH,
                    row as f32 * SPRITE_HEIGHT,
                    SPRITE_WIDTH,
                    SPRITE_HEIGHT,
                )),
                dest_size: Some(vec2(PIECE_SIZE, PIECE_SIZE)),
                ..Default::default()
            },
        );
    }
}

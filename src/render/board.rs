use macroquad::color::Color;
use macroquad::shapes::draw_rectangle;
use macroquad::text::draw_text;

use crate::pieces::Color as PieceColor;

use super::Renderer;

pub const SQUARE_SIZE: f32 = 100.0;
pub const PIECE_SIZE: f32 = 80.0;
pub const SCREEN_WIDTH: f32 = SQUARE_SIZE * 8.0;
pub const SCREEN_HEIGHT: f32 = SQUARE_SIZE * 8.0;
const LIGHT_COLOR: Color = Color::from_rgba(240, 217, 181, 255);
const DARK_COLOR: Color = Color::from_rgba(181, 136, 99, 255);

impl Renderer<'_> {
    pub fn draw_board(&self, turn: PieceColor) {
        for x in 0..8 {
            for y in 0..8 {
                let is_light = if turn == PieceColor::White { (x + y) % 2 == 0 } else { (x + y) % 2 == 1 };
                let color = if is_light { LIGHT_COLOR } else { DARK_COLOR };

                draw_rectangle(
                    x as f32 * SQUARE_SIZE,
                    y as f32 * SQUARE_SIZE,
                    SQUARE_SIZE,
                    SQUARE_SIZE,
                    color
                );
            }
        }

        let ranks = ["a", "b", "c", "d", "e", "f", "g", "h"];
        for index in 0..8 {
            let color = if index % 2 == 0 { LIGHT_COLOR } else { DARK_COLOR };

            draw_text(
                ranks[index],
                SQUARE_SIZE / 16.0 + SQUARE_SIZE * index as f32,
                SCREEN_HEIGHT - SQUARE_SIZE / 16.0,
                16.0,
                color
            );
        }

        let files = ["1", "2", "3", "4", "5", "6", "7", "8"];
        for index in 0..8 {
            let color = if index % 2 == 1 { LIGHT_COLOR } else { DARK_COLOR };

            draw_text(
                files[index],
                SCREEN_WIDTH - SQUARE_SIZE / 8.0,
                SCREEN_HEIGHT - 7.0 * SQUARE_SIZE / 8.0 - SQUARE_SIZE * index as f32,
                16.0,
                color
            );
        }
    }
}

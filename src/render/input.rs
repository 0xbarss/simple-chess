use macroquad::color::{Color, WHITE, YELLOW};
use macroquad::shapes::{draw_circle, draw_rectangle_lines};
use macroquad::text::draw_text;

use crate::moves::Move;
use crate::pieces::{Color as PieceColor, Square};
use crate::render::board::{SCREEN_HEIGHT, SCREEN_WIDTH, SQUARE_SIZE};

#[derive(Clone)]
pub struct InputState {
    pub position: Option<usize>,
    pub legal_moves: Vec<Move>
}

impl InputState {
    pub fn empty() -> InputState {
        InputState {
            position: None,
            legal_moves: vec![]
        }
    }

    pub fn new(position: Option<usize>, legal_moves: Vec<Move>) -> InputState {
        InputState {
            position,
            legal_moves
        }
    }
}

pub fn draw_highlights(rank: u32, file: u32, selected: Option<Square>, legal_moves: &Vec<Move>, color: Color) {
    if selected.is_some() {
        let x = file as f32 * SQUARE_SIZE;
        let y = (7.0 - rank as f32) * SQUARE_SIZE;
        draw_rectangle_lines(
            x,
            y,
            SQUARE_SIZE,
            SQUARE_SIZE,
            10.0,
            color
        );
    }

    for mv in legal_moves {
        if mv.from == (file as usize + rank as usize * 8) {
            let x = mv.to % 8;
            let y = 7 - mv.to / 8;
            draw_circle(
                x as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
                y as f32 * SQUARE_SIZE + SQUARE_SIZE / 2.0,
                SQUARE_SIZE / 12.0,
                WHITE
            );
        }
    }
}

pub fn draw_win(color: PieceColor) {
    let text = if color == PieceColor::White { "WHITE WIN!" } else { "BLACK WIN!" };
    draw_text(
        text,
        SCREEN_WIDTH / 8.0,
        SCREEN_HEIGHT / 2.0,
        128.0,
        YELLOW
    );
}

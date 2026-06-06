pub mod pieces;
pub mod board;
pub mod moves;
pub mod rules;
pub mod render;

use crate::board::Board;
use crate::pieces::Square;
use crate::render::Renderer;
use crate::render::board::{SCREEN_HEIGHT, SCREEN_WIDTH, SQUARE_SIZE};
use crate::render::input::{InputState, draw_highlights, draw_win};
use crate::rules::generate_legal_moves;

use macroquad::main;
use macroquad::color::{LIGHTGRAY, RED, WHITE};
use macroquad::texture::load_texture;
use macroquad::input::{is_mouse_button_pressed, mouse_position, MouseButton};
use macroquad::window::{clear_background, next_frame, request_new_screen_size};

#[main("Game")]
async fn main() {
    request_new_screen_size(SCREEN_WIDTH, SCREEN_HEIGHT);

    let texture = load_texture("assets/tileset_64.png").await.unwrap();
    let renderer = Renderer::new(&texture);

    let mut board = Board::new();
    let mut state = InputState::empty();

    loop {
        clear_background(LIGHTGRAY);

        renderer.draw_board(board.turn);
        renderer.draw_pieces(&board);

        if is_mouse_button_pressed(MouseButton::Left) {
            let (x, y) = mouse_position();
            let rank = 7 - y as u32 / SQUARE_SIZE as u32;
            let file = x as u32 / SQUARE_SIZE as u32;
            let position = file as usize + rank as usize * 8;

            match state.position {
                None => {
                    if let Square::Occupied(piece) = board.squares[position] &&
                       piece.color == board.turn
                    {
                        let legal_moves = generate_legal_moves(&board)
                            .into_iter()
                            .filter(|m| m.from == position)
                            .collect();

                        state = InputState::new(Some(position), legal_moves);
                    }
                },
                Some(_) => {
                    if let Some(mv) = state.legal_moves
                                           .iter()
                                           .find(|m| m.to == position)
                                           .cloned()
                    {
                        board.make_move(&mv);
                        board.rotate();
                        state = InputState::empty();
                    } else {
                        match board.squares[position] {
                            Square::Occupied(piece) if piece.color == board.turn => {
                                let legal_moves = generate_legal_moves(&board)
                                                  .into_iter()
                                                  .filter(|m| m.from == position)
                                                  .collect();

                                state = InputState::new(
                                    Some(position),
                                    legal_moves
                                );
                            },
                            _ => {
                                state = InputState::empty();
                            }
                        }
                    }
                }
            }
        }

        if let Some(selected_pos) = state.position {
            let rank = selected_pos / 8;
            let file = selected_pos % 8;

            draw_highlights(
                rank as u32,
                file as u32,
                Some(board.squares[selected_pos]),
                &state.legal_moves,
                WHITE
            );
        }

        if board.is_in_check(board.turn) {
            let (king_square, pos) = board.king_square(board.turn);
            let rank = pos / 8;
            let file = pos % 8;

            draw_highlights(
                rank as u32,
                file as u32,
                Some(king_square),
                &state.legal_moves,
                RED
            );

            let king_has_move = generate_legal_moves(&board)
                                       .into_iter()
                                       .any(|m| m.from == pos);

            if !king_has_move {
                draw_win(board.turn.opposite());
            }
        }

        next_frame().await;
    }
}

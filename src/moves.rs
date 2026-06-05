use crate::{board::Board, pieces::{PieceKind, Square}};

#[derive(Debug, Clone)]
pub enum MoveFlag {
    Quiet,
    DoublePawnPush,
    Capture,
    EnPassant,
    CastleKingside,
    CastleQueenside,
    Promotion(PieceKind)
}

#[derive(Debug, Clone)]
pub struct Move {
    pub from: usize,
    pub to: usize,
    pub flag: MoveFlag
}

pub fn generate_pseudo_moves(board: &Board) -> Vec<Move> {
    let mut moves: Vec<Move> = vec![];
    let squares = board.squares;
    for index in 0..64 {
        let square = squares[index];
        if let Square::Occupied(piece) = square {
            match piece.kind {
                PieceKind::Pawn => {
                    let current_color = piece.color;
                    let is_bottom = board.turn == current_color;

                    let pawn_dirs: [i32; 4] = if is_bottom { [7, 8, 9, 16] } else { [-7, -8, -9, -16] };

                    for dir in pawn_dirs {
                        let Some(pos) = index.checked_add_signed(dir as isize) else { continue};
                        if pos >= 64 {
                            continue;
                        }

                        let current_rank = index / 8;
                        let target_rank = pos / 8;

                        match squares[pos as usize] {
                            Square::Empty(_) => {
                                // DoublePawnPush
                                if dir.abs() == 16 && ((is_bottom && current_rank == 1) || (!is_bottom && current_rank == 6)) {
                                    if let Some(half) = index.checked_add_signed(dir as isize) &&
                                        let Square::Empty(_) = squares[half as usize]
                                    {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::DoublePawnPush
                                        });
                                    }
                                }
                                // Promotion
                                else if (dir == 8 && target_rank == 7) || (dir == -8 && target_rank == 0) {
                                    if let Square::Empty(_) = squares[pos as usize] {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Promotion(PieceKind::Queen)
                                        });
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Promotion(PieceKind::Rook)
                                        });
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Promotion(PieceKind::Bishop)
                                        });
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Promotion(PieceKind::Knight)
                                        });
                                    }
                                }
                                // Quiet
                                else if dir.abs() == 8 {
                                    if let Square::Empty(_) = squares[pos as usize] {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Quiet
                                        });
                                    }
                                }
                                // En Passant
                                else if dir.abs() % 8 != 0 && board.en_passant >= 0 && current_rank == 4 {
                                    let en_passant_index = board.en_passant as usize;
                                    if let Some(prev_pos) = index.checked_add_signed(-1) &&
                                        en_passant_index == prev_pos &&
                                        pos == en_passant_index + 8
                                    {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::EnPassant
                                        });
                                    }
                                    else if en_passant_index == index + 1 &&
                                        pos == en_passant_index + 8
                                    {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::EnPassant
                                        });
                                    }
                                }
                            },
                            Square::Occupied(piece) => {
                                if dir % 8 == 0 || piece.color == current_color { continue };
                                // Promotion
                                if target_rank == 7 {
                                    moves.push(Move {
                                        from: index,
                                        to: pos,
                                        flag: MoveFlag::Promotion(PieceKind::Queen)
                                    });
                                    moves.push(Move {
                                        from: index,
                                        to: pos,
                                        flag: MoveFlag::Promotion(PieceKind::Rook)
                                    });
                                    moves.push(Move {
                                        from: index,
                                        to: pos,
                                        flag: MoveFlag::Promotion(PieceKind::Bishop)
                                    });
                                    moves.push(Move {
                                        from: index,
                                        to: pos,
                                        flag: MoveFlag::Promotion(PieceKind::Knight)
                                    });
                                }
                                // Capture
                                else {
                                    moves.push(Move {
                                        from: index,
                                        to: pos,
                                        flag: MoveFlag::Capture
                                    });
                                };
                            }
                        }
                    }
                },

                PieceKind::Rook => {
                    let current_color = piece.color;

                    let rook_dirs = [8, -8, 1, -1];

                    for dir in rook_dirs {
                        let mut pos = index;

                        while let Some(next) = pos.checked_add_signed(dir) {
                            if next >= 64 {
                                break;
                            }
                            if (dir == 1 || dir == -1) && (pos / 8 != next / 8) {
                                break;
                            }
                            match squares[next as usize] {
                                Square::Empty(_) => {
                                    moves.push(Move {
                                        from: index,
                                        to: next,
                                        flag: MoveFlag::Quiet
                                    });
                                    pos = next;
                                },
                                Square::Occupied(piece) => {
                                    if piece.color != current_color {
                                        moves.push(Move {
                                            from: index,
                                            to: next,
                                            flag: MoveFlag::Capture
                                        });
                                    }
                                    break;
                                }
                            }
                        }
                    }
                },

                PieceKind::Knight => {
                     let current_color = piece.color;

                     let knight_dirs = [-6, 6, -10, 10, -15, 15, -17, 17];

                     for dir in knight_dirs {
                         if let Some(pos) = index.checked_add_signed(dir) {
                             if pos >= 64 {
                                 continue;
                             }

                             let current_rank = index / 8;
                             let current_file = index % 8;

                             let target_rank = pos / 8;
                             let target_file = pos % 8;

                             let rank_diff = current_rank.abs_diff(target_rank);
                             let file_diff = current_file.abs_diff(target_file);

                             if !(
                                 (rank_diff == 2 && file_diff == 1) ||
                                 (rank_diff == 1 && file_diff == 2)
                             ) {
                                 continue;
                             }

                             match squares[pos as usize] {
                                 Square::Empty(_) => {
                                     moves.push(Move {
                                         from: index,
                                         to: pos,
                                         flag: MoveFlag::Quiet
                                     });
                                 },
                                 Square::Occupied(piece) => {
                                     if piece.color != current_color {
                                         moves.push(Move {
                                             from: index,
                                             to: pos,
                                             flag: MoveFlag::Capture
                                         });
                                     }
                                 }
                             }
                         }
                     }
                },

                PieceKind::Bishop => {
                    let current_color = piece.color;

                    let bishop_dirs = [9, 7, -7, -9];

                    for dir in bishop_dirs {
                        let mut pos = index;

                        while let Some(next) = pos.checked_add_signed(dir) {
                            if next >= 64 {
                                break;
                            }
                            if (pos / 8).abs_diff(next / 8) != 1 || // Rank Diff
                                (pos % 8).abs_diff(next % 8) != 1   // File Diff
                            {
                                break;
                            }
                            match squares[next as usize] {
                                Square::Empty(_) => {
                                    moves.push(Move {
                                        from: index,
                                        to: next,
                                        flag: MoveFlag::Quiet
                                    });
                                    pos = next;
                                },
                                Square::Occupied(piece) => {
                                    if piece.color != current_color {
                                        moves.push(Move {
                                            from: index,
                                            to: next,
                                            flag: MoveFlag::Capture
                                        });
                                    }
                                    break;
                                }
                            }
                        }
                    }
                },

                PieceKind::Queen => {
                    let current_color = piece.color;

                    let rook_dirs = [8, -8, 1, -1];
                    let bishop_dirs = [9, 7, -7, -9];

                    for dir in rook_dirs {
                        let mut pos = index;

                        while let Some(next) = pos.checked_add_signed(dir) {
                            if next >= 64 {
                                break;
                            }
                            if (dir == 1 || dir == -1) && (pos / 8 != next / 8) {
                                break;
                            }
                            match squares[next as usize] {
                                Square::Empty(_) => {
                                    moves.push(Move {
                                        from: index,
                                        to: next,
                                        flag: MoveFlag::Quiet
                                    });
                                    pos = next;
                                },
                                Square::Occupied(piece) => {
                                    if piece.color != current_color {
                                        moves.push(Move {
                                            from: index,
                                            to: next,
                                            flag: MoveFlag::Capture
                                        });
                                    }
                                    break;
                                }
                            }
                        }
                    }

                    for dir in bishop_dirs {
                        let mut pos = index;

                        while let Some(next) = pos.checked_add_signed(dir) {
                            if next >= 64 {
                                break;
                            }
                            if (pos / 8).abs_diff(next / 8) != 1 || // Rank Diff
                                (pos % 8).abs_diff(next % 8) != 1   // File Diff
                            {
                                break;
                            }
                            match squares[next as usize] {
                                Square::Empty(_) => {
                                    moves.push(Move {
                                        from: index,
                                        to: next,
                                        flag: MoveFlag::Quiet
                                    });
                                    pos = next;
                                },
                                Square::Occupied(piece) => {
                                    if piece.color != current_color {
                                        moves.push(Move {
                                            from: index,
                                            to: next,
                                            flag: MoveFlag::Capture
                                        });
                                    }
                                    break;
                                }
                            }
                        }
                    }
                },

                PieceKind::King => {
                    let current_color = piece.color;

                    let king_dirs = [7, 8, 9, -7, -8, -9, 1, -1, 2, -2];

                    for dir in king_dirs {
                        if let Some(pos) = index.checked_add_signed(dir) {
                            if pos >= 64 {
                                continue;
                            }

                            let current_rank = index / 8;
                            let current_file = index % 8;

                            let target_rank = pos / 8;
                            let target_file = pos % 8;

                            if (current_rank.abs_diff(target_rank) > 1) ||
                               (current_file.abs_diff(target_file) > 1 && dir.abs() != 2)
                            {
                                continue;
                            }

                            match squares[pos as usize] {
                                Square::Empty(_) => {
                                    // Castling
                                    // These are not covered:
                                    // King or Rook not played before
                                    // No Check / No Threats to areas between king and rook
                                    if dir == 2 && index == 4 {
                                        if let Square::Occupied(piece) = squares[7] &&
                                           let Square::Empty(_) = squares[5] &&
                                           piece.kind == PieceKind::Rook
                                        {
                                            moves.push(Move {
                                                from: index,
                                                to: pos,
                                                flag: MoveFlag::CastleKingside
                                            });
                                        }
                                    }
                                    else if dir == -2 && index == 4 {
                                            if let Square::Occupied(piece) = squares[0] &&
                                               let Square::Empty(_) = squares[1] &&
                                               let Square::Empty(_) = squares[3] &&
                                               piece.kind == PieceKind::Rook
                                            {
                                                moves.push(Move {
                                                    from: index,
                                                    to: pos,
                                                    flag: MoveFlag::CastleQueenside
                                                });
                                            }
                                    }
                                    else {
                                        // Quiet
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Quiet
                                        });
                                    }
                                },
                                Square::Occupied(piece) => {
                                    if piece.color != current_color {
                                        moves.push(Move {
                                            from: index,
                                            to: pos,
                                            flag: MoveFlag::Capture
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    moves
}

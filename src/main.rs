mod board;
mod point;
use std::collections::HashSet;

use crate::board::*;

fn main() {
    let solutions = find_all_8x8_n_queens_solutions();

    for solution in &solutions {
        println!("{}\n----\n", Board::to_string(solution));
    }

    println!("Solution Count: {}", solutions.len());
}

fn next(
    board_size: usize,
    row: usize,
    board: &mut Board,
    solutions: &mut Vec<Board>,
    unique_solutions: &mut HashSet<String>,
) {
    for col in 0..board_size {
        let proposed_position = &board.position(col, row);

        if board.under_attack_queen(proposed_position) {
            continue;
        } else {
            board.set_queen_at(proposed_position);

            // println!("\n\n{}", Board::to_string(board));
            // std::thread::sleep(std::time::Duration::from_millis(500));

            if board.queen_count() == board_size {
                let solved_board = board.clone();
                let rot90 = solved_board.rotate90();
                let rot180 = rot90.rotate90();
                let rot270 = rot180.rotate90();
                let mirror = solved_board.mirror();
                let m_rot90 = rot90.mirror();
                let m_rot180 = rot180.mirror();
                let m_rot270 = rot270.mirror();

                for s in vec![
                    solved_board,
                    rot90,
                    rot180,
                    rot270,
                    mirror,
                    m_rot90,
                    m_rot180,
                    m_rot270,
                ] {
                    let solution = Board::to_string(&s);
                    if unique_solutions.get(&solution).is_none() {
                        unique_solutions.insert(solution.clone());
                        solutions.push(s);
                    }
                }

                board.clear_at(proposed_position);
            } else {
                next(board_size, row + 1, board, solutions, unique_solutions);
                board.clear_at(proposed_position);
            }
        }
    }
}
/// Given an n x n board attempt to put n queens on the board so
/// they do not threaten each other. If the position of one queen
/// is given, then attempt to find all the solutions for the b
fn find_all_8x8_n_queens_solutions() -> Vec<Board> {
    let board_size = 8;
    let mut solutions = vec![];
    let mut unique_solutions: HashSet<String> = HashSet::new();
    let mut board = Board::new(board_size);

    next(
        board_size,
        0,
        &mut board,
        &mut solutions,
        &mut unique_solutions,
    );

    solutions
}

mod board;
mod point;
use std::collections::HashSet;

use crate::board::*;

fn main() {
    let a: Vec<usize> = (0..8).collect();
    let mut unique_solutions: HashSet<String> = HashSet::new();

    for x in &a {
        for y in &a {
            let mut board = Board::new(8);

            board.set_queen_at(&point::Point {
                col: *x,
                row: *y,
                n: 8,
            });

            // let original_board_str = Board::to_string(&board);
            if let Some(solved_board) = solve_n_queens_f(board, Some((*x, *y))) {
                let rot90 = solved_board.rotate90();
                let rot180 = rot90.rotate90();
                let rot270 = rot180.rotate90();
                let m = solved_board.mirror();
                let m_rot90 = rot90.mirror();
                let m_rot180 = rot180.mirror();
                let m_rot270 = rot270.mirror();

                for s in vec![
                    solved_board,
                    rot90,
                    rot180,
                    rot270,
                    m,
                    m_rot90,
                    m_rot180,
                    m_rot270,
                ] {
                    let solution = Board::to_string(&s);
                    if unique_solutions.get(&solution).is_none() {
                        unique_solutions.insert(solution.clone());
                        println!("Solution: ");
                        println!("{}", solution);
                        println!("\n----------\n");
                    }
                }
            }
        }
    }

    println!("Solution Count: {}", unique_solutions.len());
}

// println!("\n\n{}", Board::to_string(&result.1));
// std::thread::sleep(std::time::Duration::from_millis(1));

fn level(col: usize, queen_count: usize, board: Board) -> (bool, Board) {
    let mut result = (false, board);

    if result.1.size == 1 && result.1.col_has_queen(col) {
        result = (true, result.1);
    } else if result.1.col_has_queen(col) {
        result = level(col + 1, queen_count, result.1);
    } else {
        let rows: Vec<usize> = (0..result.1.size).collect();

        for row in rows {
            if result.1.has_taken_row(row) {
                continue;
            }

            let proposed_position = result.1.position(col, row);
            let ok = !result.1.under_attack_queen(&proposed_position);

            if ok {
                result.1.set_queen_at(&proposed_position);

                if queen_count + 1 == result.1.size {
                    // Found Solution
                    result.0 = true;
                    break;
                } else {
                    if col == result.1.size - 1 {
                        break;
                    }

                    result = level(col + 1, queen_count + 1, result.1);

                    match result.0 {
                        true => {
                            break;
                        }
                        false => {
                            result.1.clear_at(&result.1.position(col, row));
                            continue;
                        }
                    }
                }
            } else {
                continue;
            }
        }
    }

    result
}

/// Given an n x n board attempt to put n queens on the board so
/// they do not threaten each other. If the position of one queen
/// is given, then attempt to find the solution for the remaining
/// n-1 queens
fn solve_n_queens_f(mut board: Board, queen_coords: Option<(usize, usize)>) -> Option<Board> {
    let mut queen_count = 0;

    if let Some(x) = queen_coords {
        let (mcol, mrow) = x;
        board.set_queen_at(&board.position(mcol, mrow));
        queen_count = 1;
    }

    let (ok, board) = level(0, queen_count, board);
    match ok {
        true => Some(board.clone()),
        false => None,
    }
}

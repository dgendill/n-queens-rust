mod board;
mod point;
use crate::board::*;

fn main() {
    let board = Board::new(8);

    let board = solve_n_queens_f(board, Some((3, 0)));
    match board {
        Some(x) => {
            println!("{}", x);
        }
        None => {
            println!("No solution");
        }
    }
}

fn level(col: usize, queen_count: usize, board: Board) -> (bool, Board) {
    let mut result = (false, board);

    if result.1.n == 1 && result.1.col_has_queen(col) {
        result = (true, result.1);
    } else if result.1.col_has_queen(col) {
        result = level(col + 1, queen_count, result.1);
    } else {
        let free_rows = result.1.free_rows.clone();

        for r in free_rows {
            let row = r.1;

            let p = result.1.position(col, row);
            let ok = !result.1.under_attack_queen(&p);

            if ok {
                // println!("\n\n{}", Board::to_string(&result.1));
                // std::thread::sleep(std::time::Duration::from_millis(1));

                result.1.set_queen_at(&result.1.position(col, row));

                // println!("\n\n{}", Board::to_string(&result.1));
                // std::thread::sleep(std::time::Duration::from_millis(1));

                if queen_count + 1 == result.1.n {
                    result.0 = true;
                    break;
                } else {
                    if col == result.1.n - 1 {
                        break;
                    }

                    result = level(col + 1, queen_count + 1, result.1);

                    match result.0 {
                        true => {
                            break;
                        }
                        false => {
                            result.1.clear_at(&result.1.position(col, row));

                            // println!("\n\n{}", Board::to_string(&result.1));
                            // std::thread::sleep(std::time::Duration::from_millis(40));

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

pub fn solve_n_queens(n: usize, mandatory_coords: (usize, usize)) -> Option<String> {
    let board = Board::new(n);
    solve_n_queens_f(board, Some(mandatory_coords))
}

/// Given an n x n board attempt to put n queens on the board so
/// they do not threaten each other. If the position of one queen
/// is given, then attempt to find the solution for the remaining
/// n-1 queens
fn solve_n_queens_f(mut board: Board, queen_coords: Option<(usize, usize)>) -> Option<String> {
    let mut queen_count = 0;

    if let Some(x) = queen_coords {
        let (mcol, mrow) = x;
        board.set_queen_at(&board.position(mcol, mrow));
        queen_count = 1;
    }

    let (ok, board) = level(0, queen_count, board);
    match ok {
        true => Some(Board::to_string(&board)),
        false => None,
    }
}

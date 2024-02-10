use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
 
    let size = 4;
    let board = make_board(size);
    let board = solve_n_queens_f(size, board, None);

    match board {
        Some(x) => {
            println!("{}", x);
        },
        None => {
            println!("No Solution");
        }
    }

}

fn guessing_game() {
    
    let secret = rand::thread_rng().gen_range(1, 101);   
    
    loop {
        
        println!("Make Guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };
    
        //let guess: u32 = guess.trim().parse().expect("Please enter a number");
    
        println!("You guessed {}", guess);
    
        match guess.cmp(&secret) {
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!")
        }

    }
    
    // println!("The secret is {}", secret);    

}

pub fn row_has_queen(n: usize, board: &str, row: usize) -> bool {
    let mut has_queen = false;
    for col in 0..n {
        if has_queen_at(n, board, (col, row)) {
            has_queen = true;
            break;
        }
        
    }
    has_queen
}

pub fn col_has_queen(n: usize, board: &str, col: usize) -> bool {
    let mut has_queen = false;
    for row in 0..n {
        if has_queen_at(n, board, (col, row)) {
            has_queen = true;
            break;
        }
        
    }
    has_queen
}

pub fn diag_has_queen(n: usize, board: &str, intersection: (usize, usize)) -> bool {
    let mut has_queen = false;
    let (col, row) = intersection;

    // Below the intersection...
    for (index, r) in (row..n).enumerate() {
        if index == 0 { continue; }
        let mut skip_right = false;
        let mut skip_left = false;

        let ubound = if col + index > n-1 {
            skip_right = true;
            0
        } else {
            col + index
        };

        let lbound = if index > col {
            skip_left = true;
            0
        } else {
            col - index
        };

        let has_right = !skip_right && has_queen_at(n, board, (ubound, r));
        let has_left = !skip_left && has_queen_at(n, board, (lbound, r));
        
        // println!("({},{}) ({},{})", ubound, r, lbound, r);
        if has_right || has_left  {
            if has_right {
                // println!("({},{})", ubound, r);
            } else {
                // println!("({},{})", lbound, r);
            }
            // println!("{}", board);
            has_queen = true;
            break;
        }
    }

    // Above the intersection...
    if !has_queen {

        // println!("from ({},{}) up", col, row);

        for (index, rr) in (0..row+1).enumerate() {
            let r = row - rr;
            if index == 0 { continue; }
            let mut skip_right = false;
            let mut skip_left = false;

            let ubound = if col + index > n-1 {
                skip_right = true;
                0
            } else {
                col + index
            };

            let lbound = if index > col {
                skip_left = true;
                0
            } else {
                col - index
            };

            let has_right = !skip_right && has_queen_at(n, board, (ubound, r));
            let has_left = !skip_left && has_queen_at(n, board, (lbound, r));
            // println!("({},{}) ({},{})", ubound, r, lbound, r);
    
            if has_right || has_left  {
                if has_right {
                    // println!("({},{})", ubound, r);
                } else {
                    // println!("({},{})", lbound, r);
                }
                // println!("{}", board);
                has_queen = true;
                break;
            }
        }
    }

    has_queen
}

pub fn can_place_at(n: usize, board: &String, intersection: (usize, usize)) -> bool {
    let (col, row) = intersection;

    !(
    has_queen_at(n, &board[..], intersection) ||
    row_has_queen(n, &board[..], row) ||
    col_has_queen(n, &board[..], col) ||
    diag_has_queen(n, &board[..], intersection))
}

pub fn has_queen_at(n: usize, board: &str, intersection: (usize, usize)) -> bool {
    let (col, row) = intersection;
    let start = (row * n) + col + (1*row);
    let end = start + 1;

    if board[start..end].to_string() == "Q" {
        true
    } else {
        false
    }
}

pub fn set_queen_at(n: usize, mut board: String, intersection: (usize, usize)) -> String {
    let (col, row) = intersection;
    let start = (row * n) + col + (1*row);
    let end = start + 1;

    board.replace_range(start..end, "Q");
    board
}

pub fn clear_at(n: usize, mut board: String, intersection: (usize, usize)) -> String {
    let (col, row) = intersection;
    let start = (row * n) + col + (1*row);
    let end = start + 1;

    board.replace_range(start..end, ".");
    board
}

pub fn make_board(n: usize) -> String {
    let mut board = String::from("");

    for _row in 0..n {
        for _col in 0..n {
            board.push_str(".");
        }
        board.push_str("\n");
    }

    board
}

pub fn level(i: usize, n: usize, c: usize, mut oboard: String) -> Option<String> {

    let mut result = None;

    if n == 1 && col_has_queen(n, &oboard[..], i) {
        result = Some(oboard);
    } else if col_has_queen(n, &oboard[..], i) {
        println!("chq {}\n{}", i, oboard);
        result = level(i+1, n, c, oboard);
    } else {

        for q1 in 0..n {
            let ok = can_place_at(n, &oboard, (i, q1));
            
            // println!("l{} q{}\n{}", i, c + 1, oboard);
    
            if ok {
                oboard = set_queen_at(n, oboard, (i, q1));
    
                if c + 1 == n {
                    result = Some(oboard);
                    break;
                } else {
                    println!("l{} q{}\n{}", i, c, oboard);
                    let nboard = oboard.to_string();
                    result = level(i+1, n, c+1, nboard);
                    
                    match result {
                        Some(_) => {
                            break;
                        },
                        None => {
                            oboard = clear_at(n, oboard, (i, q1));
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
    let board = make_board(n);
    solve_n_queens_f(n, board, Some(mandatory_coords))
}

pub fn solve_n_queens_f(n: usize, mut board: String, mandatory_coords: Option<(usize, usize)>) -> Option<String> {

    let mut queen_count = 0;

    match mandatory_coords {
        Some(x) => {
            let (mcol, mrow) = x;    
            board = set_queen_at(n, board, (mcol, mrow));
            queen_count = 1;
        },
        None => {}
    }
    
    let result = level(0, n, queen_count, board);

    result
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_row_check() {

      let n = 10;
      let board = make_board(n);  
      let board = set_queen_at(n, board, (5,0));
      let board = set_queen_at(n, board, (5,1));
      let board = set_queen_at(n, board, (6,2));
      let board = set_queen_at(n, board, (n-1,n-1));
      let board = set_queen_at(n, board, (0,0));


      let qat = has_queen_at(n, &board, (0, 0));
      assert_eq!(qat, true, "There is a queen at (0,0)");

      let qat = row_has_queen(n, &board, 0);
      assert_eq!(qat, true, "Row 0 has a queen");

      let qat = row_has_queen(n, &board, 1);
      assert_eq!(qat, true, "Row 1 has a queen");

      let qat = row_has_queen(n, &board, 2);
      assert_eq!(qat, true, "Row 2 has a queen");

      let qat = row_has_queen(n, &board, 3);
      assert_eq!(qat, false, "Row 3 does not have a queen");
      
    }

    #[test]
    fn test_col_check() {

        let n = 10;
        let board = make_board(n);  
        let board = set_queen_at(n, board, (5,0));
        let board = set_queen_at(n, board, (5,1));
        let board = set_queen_at(n, board, (6,2));
        let board = set_queen_at(n, board, (n-1,n-1));
        let board = set_queen_at(n, board, (0,0));
  
        let qat = col_has_queen(n, &board, 5);
        assert_eq!(qat, true, "Col 5 has a queen");
  
        let qat = col_has_queen(n, &board, 6);
        assert_eq!(qat, true, "Col 6 has a queen");

        let qat = col_has_queen(n, &board, 4);
        assert_eq!(qat, false, "Col 4 does not have a queen");
        
      }

      #[test]
      fn diag_has_queen_test() {

        let n = 10;
        let board = make_board(n);  
        let board = set_queen_at(n, board, (5,0));
        let board = set_queen_at(n, board, (5,1));
        let board = set_queen_at(n, board, (6,2));
        let board = set_queen_at(n, board, (n-1,n-1));
        let board = set_queen_at(n, board, (0,0));
  
        let qat = diag_has_queen(n, &board, (6,2));
        assert_eq!(qat, true, "Diag from (6,2) has a queen");

        let qat = diag_has_queen(n, &board, (6,6));
        assert_eq!(qat, true, "Diag from (6,6) has a queen");

        let qat = diag_has_queen(n, &board, (6,5));
        assert_eq!(qat, false, "Diag from (6,5) is clear");

        let board = make_board(n);  
        let board = set_queen_at(n, board, (0,0));

        let qat = diag_has_queen(n, &board, (1,1));
        assert_eq!(qat, true, "Diag from (1,1) has a queen");

      }

}
// use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

use std::collections::HashMap;

#[derive(Debug)]
#[derive(Hash)]
struct Point {
    col: usize,
    row: usize,
    n: usize
}

impl Point {
    fn index(&self) -> usize {
        let col = if self.col >= self.n {
            self.n - 1
        } else {
            self.col
        };

        let row = if self.row >= self.n {
            self.n - 1
        } else {
            self.row
        };

        col + row * self.n
    }

    fn from_index(index: usize, n: usize) -> Point {
        let col = index % n;
        let row = ((index / n) as f32).floor() as usize;
        Point { col, row, n }
    }

    fn t(&self) -> Option<usize> {
        let index = &self.index();
        if index < &self.n {
            None
        } else {
            Some(index - &self.n)
        }
    }

    fn b(&self) -> Option<usize> {
        let index = &self.index();
        let bottom_row = &self.n * &self.n - &self.n;
        if index >= &bottom_row {
            None
        } else {
            Some(index + &self.n)
        }
    }

    fn r(&self) -> Option<usize> {
        let index = &self.index();
        if index == &(0 as usize) {
            Some(1)
        } else if (index + 1) % &self.n == 0 {
            None
        } else {
            Some(index + 1)
        }
    }

    fn l(&self) -> Option<usize> {
        let index = &self.index();
        if index == &(0 as usize) {
            None
        } else if (index + 1) % &self.n == 1 {
            None
        } else {
            Some(index - 1)
        }
    }

}

impl Eq for Point {}
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.n == other.n && self.col == other.col && self.row == other.row
    }
}


#[derive(Debug)]
struct Board {
    n: usize,
    grid: Vec<bool>,
    queens: HashMap<Point, String>,
    free_rows: HashMap<usize, usize>,
    free_cols: HashMap<usize, usize>,
}

impl Board {
    fn new(n: usize) -> Board {
        let length = n * n;
        let grid = vec![false; length];
        
        let mut free_rows = HashMap::new();
        for n in 0..n { free_rows.insert(n, n); }
        let mut free_cols = HashMap::new();
        for n in 0..n { free_cols.insert(n, n); }

        let queens = HashMap::new();
        
        Board {
            n,
            grid,
            queens,
            free_rows,
            free_cols
        }
    }

    fn to_string(board: &Board) -> String {
        let mut s = String::new();

        for (index, val) in board.grid.iter().enumerate() {
            let i = index + 1;
            // if (index, val) == (0, &true) {
            //     s.push_str("Q");
            // } else if (index, val) == (0, &false) {
            //     s.push_str(".");
            if val == &true && ((i % board.n) == 0) {
                //s.push_str("🐶\n"); 
                s.push_str("Q\n");
            } else if val == &false && ((i % board.n) == 0) {
                //s.push_str("⬛\n");
                s.push_str(".\n");
            } else if val == &true {
                //s.push_str("🐶");
                s.push_str("Q");
            } else if val == &false {
                //s.push_str("⬛");
                s.push_str(".");
            }
                
        }

        s.to_string()
    }

    fn position(&self, col: usize, row: usize) -> Point {
        Point { col, row, n: self.n }
    }

    fn has_queen_at(&self, p: &Point) -> bool {
        self.grid[p.index()]
    }

    fn set_queen_at(&mut self, p: &Point) {
        self.grid[p.index()] = true;
        
        let p1 = Point {
            col: p.col,
            row: p.row,
            n: self.n
        };

        self.queens.insert(p1, String::new());
        self.free_rows.remove(&p.row);
        self.free_cols.remove(&p.col);
    }

    fn clear_at(&mut self, p: &Point) {
        self.grid[p.index()] = false;
        self.queens.remove(p);
        self.free_rows.insert(p.row, p.row);
        self.free_cols.insert(p.col, p.col);
    }

    fn row_has_queen(&self, row: usize) -> bool {
        !self.free_rows.contains_key(&row)
    }

    fn col_has_queen(&self, col: usize) -> bool {
        !self.free_cols.contains_key(&col)
    }

    pub fn diag_has_queen(&self, intersection: &Point) -> bool {
        let mut has_queen = false;

        for (_, qp) in self.queens.iter().enumerate() {
            let (queen_position, _) = qp;
            let delta_row = (queen_position.row as isize) - (intersection.row as isize);
            let delta_column = (queen_position.col as isize) - (intersection.col as isize);

            if delta_row == delta_column || delta_row == -delta_column {
                has_queen = true;
                break;
            }

        }

        has_queen
       
    }

    pub fn under_attack_queen(&self, intersection: &Point) -> bool {
        let mut yes = false;

        for (_, qp) in self.queens.iter().enumerate() {
            let (queen_position, _) = qp;

            if intersection.row == queen_position.row {
                yes = true;
                break;
            } else if intersection.col == queen_position.col {
                yes = true;
                break;
            } else {
                let delta_row = (queen_position.row as isize) - (intersection.row as isize);
                let delta_column = (queen_position.col as isize) - (intersection.col as isize);

                if delta_row == delta_column || delta_row == -delta_column {
                    yes = true;
                    break;
                }
            }
        }

        yes
       
    }

}

fn main() {
    let mut board = Board::new(150);
    // println!("{}", Board::to_string(&board));

    let board = solve_n_queens_f(board, Some((3,0))); //mandatory_coords: Option<(usize, usize)>)
    match board {
       Some(x) => {
            println!("{}", x);
       },
       None => {
            println!("No solution");
       }
   }
}

fn level(col: usize, queen_count: usize, board: Board) -> (bool, Board) {

    let mut result = (false, board);

    if result.1.n == 1 && result.1.col_has_queen(col) {
        result = (true , result.1);
    } else if result.1.col_has_queen(col) {
        result = level(col+1, queen_count, result.1);
    } else {

        let free_rows = result.1.free_rows.clone();

        for r in free_rows {
            
            let row = r.1.clone();

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
                        true => { break; },
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
    let mut board = Board::new(n);
    solve_n_queens_f(board, Some(mandatory_coords))
}

fn solve_n_queens_f(mut board: Board, mandatory_coords: Option<(usize, usize)>) -> Option<String> {

    let mut queen_count = 0;

    match mandatory_coords {
        Some(x) => {
            let (mcol, mrow) = x;    
            board.set_queen_at(&board.position(mcol, mrow));
            queen_count = 1;
        },
        None => {}
    }
    
    let (ok, board) = level(0, queen_count, board);
    // board
    match ok {
        true => { Some(Board::to_string(&board)) },
        false => None
    }
    
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    
    use super::*;    

    #[test]
    fn test_has_queen() {

        let mut board = Board::new(10);
        let p1 = board.position(0,0);
        let p2 = board.position(0,5);
        let p3 = board.position(3,0);

        board.set_queen_at(&p1);
        board.set_queen_at(&p2);
        board.set_queen_at(&p3);

        let qat = board.has_queen_at(&p1);
        assert_eq!(qat, true, "There is a queen at {:?}", p1);

        let qat = board.has_queen_at(&p3);
        assert_eq!(qat, true, "There is a queen at {:?}", p3);

        board.clear_at(&p3);

        let qat = board.has_queen_at(&p3);
        assert_eq!(qat, false, "The queen at {:?} should be removed", p3);
      
    }

    #[test]
    fn test_row_and_col_checks() {

        let mut board = Board::new(10);
        let p1 = board.position(0,0);
        let p2 = board.position(0,5);
        let p3 = board.position(3,0);

        board.set_queen_at(&p1);
        board.set_queen_at(&p2);
        board.set_queen_at(&p3);

        let qat = board.row_has_queen(0);
        assert_eq!(qat, true, "Row 0 has a queen");

        let qat = board.row_has_queen(2);
        assert_eq!(qat, false, "Row 2 does not have a queen");

        let qat = board.row_has_queen(5);
        assert_eq!(qat, true, "Row 5 has a queen");

        let qat = board.col_has_queen(0);
        assert_eq!(qat, true, "Col 0 has a queen");

        let qat = board.col_has_queen(3);
        assert_eq!(qat, true, "Col 3 does not have a queen");

        let qat = board.col_has_queen(5);
        assert_eq!(qat, false, "Col 5 does not has a queen");

        let mut board2 = Board::new(1);
        let p4 = board2.position(0,0);
        board2.set_queen_at(&p4);

        let qat = board2.col_has_queen(0);
        assert_eq!(qat, true, "Col 0 does have a queen");
        let qat = board2.row_has_queen(0);
        assert_eq!(qat, true, "Row 0 does have a queen");

    }

    #[test]
    fn test_positions() {

        let mut board = Board::new(10);
        assert_eq!(board.position(9,8).t().is_some(), true, "top fail");
        assert_eq!(board.position(9,8).r().is_none(), true, "right fail");
        assert_eq!(board.position(9,8).b().is_some(), true, "bottom fail");
        assert_eq!(board.position(9,8).l().is_some(), true, "left fail");

        assert_eq!(board.position(9,9).r().is_none(), true, "right fail");
        assert_eq!(board.position(9,9).l().is_some(), true, "left fail");
        assert_eq!(board.position(10,10).r().is_none(), true, "ob test fail");

        assert_eq!(board.position(0,0).l().is_none(), true, "left fail");
        assert_eq!(board.position(1,0).l().is_some(), true, "left fail");
        assert_eq!(board.position(1,0).r().is_some(), true, "right fail");
        assert_eq!(board.position(1,0).t().is_none(), true, "top fail");
        assert_eq!(board.position(1,0).b().is_some(), true, "bottom fail");

        assert_eq!(board.position(0,9).l().is_none(), true, "left fail");

    }

    #[test]
    fn test_points() {
        let n = 10;
        let p = Point { col: 1, row: 1, n };
        assert_eq!(p, Point::from_index(p.index(), n), "point conversion fail");

        let n = 1;
        let p = Point { col: 0, row: 0, n };
        assert_eq!(p, Point::from_index(p.index(), n), "point conversion fail");

        let n = 7;
        let p = Point { col: 6, row: 6, n };
        assert_eq!(p, Point::from_index(p.index(), n), "point conversion fail");

    }

    #[test]
    fn diag_has_queen_test() {

        let n = 10;
        let mut board = Board::new(n);
        board.set_queen_at(&board.position(5,0));
        board.set_queen_at(&board.position(5,1));
        board.set_queen_at(&board.position(6,2));
        board.set_queen_at(&board.position(n-1,n-1));
        board.set_queen_at(&board.position(0,0));
  
        let qat = board.diag_has_queen(&board.position(6,2));
        assert_eq!(qat, true, "Diag from (6,2) has a queen");

        let qat = board.diag_has_queen(&board.position(6,6));
        assert_eq!(qat, true, "Diag from (6,6) has a queen");

        let qat = board.diag_has_queen(&board.position(6,5));
        assert_eq!(qat, false, "Diag from (6,5) is clear");

        let mut board = Board::new(n);  
        board.set_queen_at(&board.position(0,0));

        let qat = board.diag_has_queen(&board.position(1,1));
        assert_eq!(qat, true, "Diag from (1,1) has a queen");

    }

    
    #[test]
    fn perf() {
        
        let board = Board::new(20);    
        // b.iter(|| solve_n_queens_f(board, None));
        let board = solve_n_queens_f(board, None); //Some((1,1))); //mandatory_coords: Option<(usize, usize)>)
        //assert_eq!(board.is_some(), true, "");

    }

}
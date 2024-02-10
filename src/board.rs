// mod point;

use super::point::Point;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Board {
    pub size: usize,
    pub grid: Vec<bool>,
    pub queens: HashMap<Point, String>,
    pub taken_rows: HashSet<usize>,
    pub taken_cols: HashSet<usize>,
}

impl Board {
    /// Create a new n x n chess board
    pub fn new(n: usize) -> Board {
        let length = n * n;
        let grid = vec![false; length];

        let mut taken_rows = HashSet::new();
        let mut taken_cols = HashSet::new();
        let queens = HashMap::new();

        Board {
            size: n,
            grid,
            queens,
            taken_rows,
            taken_cols,
        }
    }

    #[allow(dead_code)]
    pub fn has_queen_at(&self, p: &Point) -> bool {
        self.grid[p.index()]
    }

    pub fn to_string(board: &Board) -> String {
        let mut s = String::new();

        for (index, value) in board.grid.iter().enumerate() {
            let has_queen = *value;
            let at_edge = ((index + 1) % board.size) == 0;

            if has_queen {
                s.push('🟥');
            } else {
                s.push('⬛');
            }

            if at_edge {
                s.push('\n');
            }
        }

        s.to_string()
    }

    pub fn position(&self, col: usize, row: usize) -> Point {
        Point {
            col,
            row,
            n: self.size,
        }
    }

    pub fn set_queen_at(&mut self, p: &Point) {
        self.grid[p.index()] = true;

        let p1 = Point {
            col: p.col,
            row: p.row,
            n: self.size,
        };

        self.queens.insert(p1, String::new());
        self.taken_rows.insert(p.row);
        self.taken_cols.insert(p.col);
    }

    #[allow(dead_code)]
    pub fn row_has_queen(&self, row: usize) -> bool {
        self.taken_rows.contains(&row)
    }

    pub fn clear_at(&mut self, p: &Point) {
        self.grid[p.index()] = false;
        self.queens.remove(p);
        self.taken_rows.remove(&p.row);
        self.taken_cols.remove(&p.col);
    }

    pub fn col_has_queen(&self, col: usize) -> bool {
        self.taken_cols.contains(&col)
    }

    pub fn under_attack_queen(&self, intersection: &Point) -> bool {
        let mut yes = false;

        for qp in self.queens.iter() {
            let (queen_position, _) = qp;

            if intersection.row == queen_position.row || intersection.col == queen_position.col {
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.

    use super::*;

    #[test]
    fn test_has_queen() {
        let mut board = Board::new(10);
        let p1 = board.position(0, 0);
        let p2 = board.position(0, 5);
        let p3 = board.position(3, 0);

        board.set_queen_at(&p1);
        board.set_queen_at(&p2);
        board.set_queen_at(&p3);

        let qat = board.has_queen_at(&p1);
        assert!(qat, "There is a queen at {:?}", p1);

        let qat = board.has_queen_at(&p3);
        assert!(qat, "There is a queen at {:?}", p3);

        board.clear_at(&p3);

        let qat = board.has_queen_at(&p3);
        assert!(!qat, "The queen at {:?} should be removed", p3);
    }

    #[test]
    fn test_row_and_col_checks() {
        let mut board = Board::new(10);
        let p1 = board.position(0, 0);
        let p2 = board.position(0, 5);
        let p3 = board.position(3, 0);

        board.set_queen_at(&p1);
        board.set_queen_at(&p2);
        board.set_queen_at(&p3);

        let qat = board.row_has_queen(0);
        assert!(qat, "Row 0 has a queen");

        let qat = board.row_has_queen(2);
        assert!(!qat, "Row 2 does not have a queen");

        let qat = board.row_has_queen(5);
        assert!(qat, "Row 5 has a queen");

        let qat = board.col_has_queen(0);
        assert!(qat, "Col 0 has a queen");

        let qat = board.col_has_queen(3);
        assert!(qat, "Col 3 does not have a queen");

        let qat = board.col_has_queen(5);
        assert!(!qat, "Col 5 does not has a queen");

        let mut board2 = Board::new(1);
        let p4 = board2.position(0, 0);
        board2.set_queen_at(&p4);

        let qat = board2.col_has_queen(0);
        assert!(qat, "Col 0 does have a queen");
        let qat = board2.row_has_queen(0);
        assert!(qat, "Row 0 does have a queen");
    }
}

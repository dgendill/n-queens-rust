use std::fmt;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
pub struct Point {
    /// The zero-index column position
    pub col: usize,
    /// The zero-index row position
    pub row: usize,
    /// The width and height of the board
    pub n: usize,
}

impl Point {
    pub fn index(&self) -> usize {
        let col = if self.col >= self.n { self.n } else { self.col };
        let row = if self.row >= self.n { self.n } else { self.row };
        col + row * self.n
    }

    #[allow(dead_code)]
    pub fn from_index(index: usize, n: usize) -> Point {
        let col = index % n;
        let row = ((index / n) as f32).floor() as usize;
        Point { col, row, n }
    }

    /// Rotate 90 degress clockwise
    pub fn rotate90(&self) -> Point {
        // column becomes row. new row = column
        // row becomes column. new column = n - row

        let new_row = self.col;
        let new_col = self.n - 1 - self.row;

        Point {
            col: new_col,
            row: new_row,
            n: self.n,
        }
    }

    pub fn mirror_x(&self) -> Point {
        let new_row = self.row;
        let new_col = (self.col as i8 - self.n as i8 + 1).unsigned_abs() as usize;
        Point {
            col: new_col,
            row: new_row,
            n: self.n,
        }
    }
}

impl fmt::Display for Point {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "({}, {})", self.col, self.row)
    }
}

#[test]
fn test_point_rotation() {
    let mapping = vec![
        ((1, 1), (6, 1)),
        ((0, 0), (7, 0)),
        ((7, 0), (7, 7)),
        ((7, 7), (0, 7)),
    ];

    for map in mapping {
        let from = map.0;
        let to = map.1;

        let p1 = Point {
            col: from.0,
            row: from.1,
            n: 8,
        };

        let p2 = p1.rotate90();

        println!("{} to {}", p1, p2);

        assert_eq!(
            Point {
                col: to.0,
                row: to.1,
                n: 8
            },
            p2
        )
    }
}

#[test]
fn test_mirror() {
    let mapping = vec![
        ((0, 0), (2, 0)),
        ((0, 1), (2, 1)),
        ((0, 2), (2, 2)),
        ((1, 0), (1, 0)),
        ((1, 1), (1, 1)),
        ((1, 2), (1, 2)),
        ((2, 0), (0, 0)),
        ((2, 1), (0, 1)),
        ((2, 2), (0, 2)),
    ];

    for map in mapping {
        let from = map.0;
        let to = map.1;

        let p1 = Point {
            col: from.0,
            row: from.1,
            n: 3,
        };

        let p2 = p1.mirror_x();

        println!("{} to {}", p1, p2);

        assert_eq!(
            Point {
                col: to.0,
                row: to.1,
                n: 3
            },
            p2
        )
    }
}

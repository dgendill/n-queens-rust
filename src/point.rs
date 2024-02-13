use std::{f64::consts::PI, fmt};

#[derive(Debug, Hash, PartialEq, Eq)]
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
        let radians = 90.0_f64.to_radians(); //  * PI / 180.0;
        let t: f64 = radians;
        let sin = t.sin();
        let cos = t.cos();

        let x: f64 = self.col as f64;
        let y: f64 = self.row as f64;

        // let new_col = (cos * x + sin * y).round();
        // let new_row = ((-sin) * x + cos * y).round();
        let mut new_col = (cos * x - sin * y);
        let mut new_row = (sin * x + cos * y);

        println!("{} {} {}", radians, new_col, new_row);

        if new_col <= 0.0 {
            new_col += (self.n as f64) - 1.0;
        }

        if new_row <= 0.0 {
            new_row += (self.n as f64) - 1.0;
        }

        println!("{} {} {}", radians, new_col, new_row);

        Point {
            col: new_col as usize,
            row: new_row as usize,
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
    // let mapping = vec![((1, 1), (6, 1)), ((0, 0), (7, 0)), ((7, 0), (7, 7)), ((7,7),(0,7))];
    // let mapping = vec![((1, 1), (6, 1))];
    let mapping = vec![((0, 0), (7, 0))];
    // let mapping = vec![((7, 0), (7, 7))];

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

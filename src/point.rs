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
}

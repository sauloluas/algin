use std::fmt;
use crate::Vector;

#[derive(Debug, Clone)]
pub struct Matrix {
    data: Vec<f64>,
    stride: usize,
}

impl Matrix {
    pub fn from_row(row: &Vector) -> Self {
        let data = row.data();
        let stride = row.len();

        Matrix { data, stride }
    }

    pub fn add_row(mut self, row: &Vector) -> Self {
        self.assert_row_size(&row);

        self.data.extend(row.data().into_iter());

        self
    }

    pub fn add_col(mut self, col: &Vector) -> Self {
        self.assert_col_size(&col);

        let stride = self.stride;
        for (i, e) in col.iter().enumerate() {
            self.data.insert((i+1)*stride + i, *e)
        }

        self.stride = stride + 1;

        self

    }

    // pub fn from_rows(rows: &[&Vector]) -> Self {
    // 	let data = rows.into_iter().map(|v| v.into_iter()).flatten().collect();
    // 	let stride = rows[0].len();

    // 	Matrix { data, stride }
    // }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn row_len(&self) -> usize {
        self.stride
    }

    pub fn col_len(&self) -> usize {
        self.size() / self.stride
    }

    pub fn assert_row_size(&self, row: &Vector) {
        if self.stride != row.len() {
            panic!("Incompatible row lengths");
        }
    }

    pub fn assert_col_size(&self, col: &Vector) {
        if self.col_len() != col.len() {
            panic!("Incompatible col lengths");
        }
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let row_len = self.row_len();
        let col_len = self.col_len();
        writeln!(f, "")?;
        writeln!(f, "[")?;
        for r in 0..col_len {
            write!(f, "    ")?;
            for e in 0..row_len {
                write!(f, "{} ", self.data[e+r*row_len])?;
            }
            writeln!(f, "")?;
        }
        write!(f, "]")
    }
}

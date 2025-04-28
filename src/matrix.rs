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
}

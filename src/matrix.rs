use std::{fmt, ops::{AddAssign, Div, Mul}};
use crate::{ops::Dot, SquareMatrix, Vector, Zero};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    data: Vec<T>,
    stride: usize,
}

impl<T: Clone> Matrix<T> {
    pub fn from_row(row: &Vector<T>) -> Self {
        let data = row.data();
        let stride = row.len();

        Matrix { data, stride }
    }

    pub fn push_row(mut self, row: &Vector<T>) -> Self {
        self.assert_row_size(&row);

        self.data.extend(row.data().into_iter());

        self
    }
    
}

impl<T: Zero + Clone> Matrix<T> {

    pub fn zeros(cl: usize, rl: usize) -> Self {
        let size = cl * rl;
        let data = vec![T::zero(); size];
        let stride = rl;

        Matrix { data, stride }
    }
}

impl<T: Clone + Copy> Matrix<T> {

    pub fn push_col(mut self, col: &Vector<T>) -> Self {
        self.assert_col_size(&col);

        let stride = self.stride;
        for (i, e) in col.iter().enumerate() {
            self.data.insert((i+1)*stride + i, *e)
        }

        self.stride = stride + 1;

        self

    }


} 

impl<T> Matrix<T> {
    

    pub fn set_data(mut self, dptr: Vec<T>) -> Self {
        if dptr.len() == self.size() {
            self.data = dptr;
            self
        } else {
            panic!("Incompatible data length")
        }
    }

    // pub fn from_rows(rows: &[&Vector]) -> Self {
    // 	let data = rows.into_iter().map(|v| v.into_iter()).flatten().collect();
    // 	let stride = rows[0].len();

    // 	Matrix { data, stride }
    // }

    pub fn at(&self, i: usize, j: usize) -> &T {
        let stride = self.stride;
        &self.data[i*stride + j]
    }

    pub fn mut_at(&mut self, i: usize, j: usize) -> &mut T {
        let stride = self.stride;
        &mut self.data[i*stride + j]
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn row_len(&self) -> usize {
        self.stride
    }

    pub fn col_len(&self) -> usize {
        self.size() / self.stride
    }

    pub fn assert_row_size(&self, row: &Vector<T>) {
        if self.stride != row.len() {
            panic!("Incompatible row lengths");
        }
    }

    pub fn assert_col_size(&self, col: &Vector<T>) {
        if self.col_len() != col.len() {
            panic!("Incompatible col lengths");
        }
    }

    pub fn is_square(&self) -> bool {
        if self.row_len() == self.col_len() {
            true
        } else {
            false
        }
    }

    pub fn to_sqrmat(self) -> SquareMatrix<T> {
        SquareMatrix::from(self)    
    }
}

impl<T: fmt::Display> fmt::Display for Matrix<T> {
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

impl<T> Dot<&Matrix<T>> for &Matrix<T> 
where T: 
    Mul<Output = T> 
    + Zero 
    + AddAssign
    + Clone
    + Copy
{
    type Output = Matrix<T>;

    fn dot(self, mat: &Matrix<T>) -> Matrix<T> {
        if self.row_len() != mat.col_len() {
            panic!("Incompatible matrix lengths");
        }

        let rlt_size = self.col_len() * mat.row_len();
        let dptr = vec![T::zero(); rlt_size];

        let mut rlt = Matrix::zeros(self.col_len(), mat.row_len())
            .set_data(dptr);

        for y in 0..mat.row_len() {
            for x in 0..self.col_len() {
                let mut acc = T::zero();
                for i in 0..self.row_len() {
                    acc += *self.at(x, i) * *mat.at(i, y);
                }
                *rlt.mut_at(x, y) = acc;
            }
        }

        rlt

    }
}

impl<T> Matrix<T> {
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Clone> Mul<&T> for &Matrix<T> 
where 
    for<'a> &'a T: Mul<&'a T, Output = T>,
{
    type Output = Matrix<T>; 

    fn mul(self, scalar: &T) -> Matrix<T> {

        let dptr = vec![scalar; self.size()]
            .iter()
            .zip(self.data())
            .map(|(sc, e)| e * sc)
            .collect();

        self.clone().set_data(dptr)

    }
}
use crate::{Matrix, Zero, One};
use std::fmt;

pub type SqrMat<T> = SquareMatrix<T>;
pub type FSqrMat = SqrMat<f64>;

pub struct SquareMatrix<T> {
	mat: Matrix<T>
}

impl<T: Zero + Clone> SquareMatrix<T> {

	pub fn zeros(order: usize) -> Self {
		Matrix::zeros(order, order).to_sqrmat()
	}

}

impl<T: Zero + One + Clone> SqrMat<T> {

	pub fn id(order: usize) -> Self {
		if order > 0 {
			let mut m = Self::zeros(order);
			for i in 0..order {
				*m.mut_at(i, i) = T::one();
			}
			m
		} else {
			panic!("Order of identity matrix can't be zero")
		}
	}

}

impl<T> SquareMatrix<T> {
	pub fn from(mat: Matrix<T>) -> Self {
		if mat.is_square() {
			Self { mat }
		} else {
			panic!("Matrix is not square")
		}
	}

    pub fn at(&self, i: usize, j: usize) -> &T {
        self.mat.at(i, j)
    }

    pub fn mut_at(&mut self, i: usize, j: usize) -> &mut T {
        self.mat.mut_at(i, j)
    }

	pub fn order(&self) -> usize {
		self.mat.row_len()
	}

	pub fn size(&self) -> usize {
		self.mat.size()
	}

	pub fn as_mat(self) -> Matrix<T> {
		self.mat
	}
}

impl<T: fmt::Display> fmt::Display for SquareMatrix<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let m = &self.mat;
		write!(f, "{m}")
	}
}
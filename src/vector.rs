use std::{
    fmt, iter::Sum, ops::{Add, AddAssign, Index, IndexMut, Mul, Sub}
};

use crate::{ops::Dot, Matrix, Zero};

#[derive(Debug, Clone)]
pub struct Vector<T> {
    data: Vec<T>,
}

impl<T: Clone> Vector<T> {
    pub fn from(data: &[T]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn data(&self) -> Vec<T> {
        self.data.clone()
    }
}

impl<T: Copy> Vector<T> {

    pub fn elemwise<F>(&self, other: &Self, mut op: F) -> Self
    where
        F: FnMut(T, T) -> T,
    {
        self.assert_size(&other);

        let data = self
            .iter()
            .zip(other.iter())
            .map(|(&a, &b)| op(a, b))
            .collect();

        Vector { data }
    }

}

impl<T> Vector<T> {

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn at(&self, i: usize) -> &T {
        &self.data[i]
    }

    pub fn set_at(&mut self, i: usize, new: T) {
        self.data[i] = new;
    }

    pub fn mut_at(&mut self, i: usize) -> &mut T {
        &mut self.data[i]
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.data.iter()
    }

    pub fn assert_size(&self, other: &Self) {
        if self.len() != other.len() {
            panic!("Incompatible vector lengths");
        }
    }

}

impl<T: Sub<Output = T> + Copy> Sub for &Vector<T> {
    type Output = Vector<T>;

    fn sub(self, other: &Vector<T>) -> Self::Output {
        let op = |a, b| a - b;
        self.elemwise(&other, op)
    }
}

impl<T: Add<Output = T> + Copy> Add for &Vector<T> {
    type Output = Vector<T>;

    fn add(self, other: Self) -> Self::Output {
        let op = |a, b| a + b;

        self.elemwise(&other, op)
    }
}

impl<T> Index<usize> for Vector<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.at(index)
    }
}

impl<T> IndexMut<usize> for Vector<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        self.mut_at(index)
    }
}

impl<T: fmt::Display> fmt::Display for Vector<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for s in self.iter() {
            write!(f, "{s} ")?;
        }
        write!(f, "]")
    }
}

impl<T: Mul<Output = T> + Sum + Copy> Dot<&Vector<T>> for &Vector<T> {
    type Output = T;

    fn dot(self, other: &Vector<T>) -> Self::Output {
        self.assert_size(&other);

        self.iter().zip(other.iter()).map(|(a, b)| *a * *b).sum()

    }
}

impl<T> Dot<&Matrix<T>> for &Vector<T>
where 
    T: Mul<Output = T> 
    + Copy 
    + AddAssign 
    + Zero
{
    type Output = Vector<T>;

    fn dot(self, mat: &Matrix<T>) -> Self::Output {
        if self.len() != mat.col_len() {
            panic!("Incompatible dimensions");
        } 

        let mut data = vec![T::zero(); mat.row_len()];

        for h in 0..mat.row_len() {
            let mut acc = T::zero();
            for i in 0..self.len() {
                acc += *self.at(i) * *mat.at(i, h);
            }
            data[h] = acc;
        }

        Vector::from(&data)

    }
}

impl<T: Zero + Clone> Vector<T> {

    pub fn zeros(size: usize) -> Self {
        let data = vec![T::zero(); size];

        Vector { data }
    }
}

impl<T> FromIterator<T> for Vector<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let data: Vec<T> = iter.into_iter().collect();
        Vector { data }
    }
}
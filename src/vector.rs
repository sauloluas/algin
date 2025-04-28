use std::{
    fmt,
    ops::{Index, IndexMut, Sub},
};

#[derive(Debug, Clone)]
pub struct Vector {
    data: Vec<f64>,
}

impl Vector {
    pub fn new(data: &[f64]) -> Self {
        Self {
            data: data.to_vec(),
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn at(&self, i: usize) -> &f64 {
        &self.data[i]
    }

    pub fn set_at(&mut self, i: usize, new: f64) {
        self.data[i] = new;
    }

    pub fn mut_at(&mut self, i: usize) -> &mut f64 {
        &mut self.data[i]
    }

    pub fn data(&self) -> Vec<f64> {
        self.data.clone()
    }

    pub fn iter(&self) -> impl Iterator<Item = &f64> {
        self.data.iter()
    }

    pub fn assert_size(&self, other: &Self) {
        if self.len() != other.len() {
            panic!("Incompatible vector lengths");
        }
    }

    pub fn elemwise<F>(&self, other: &Self, mut op: F) -> Vector
    where
        F: FnMut(f64, f64) -> f64,
    {
        self.assert_size(&other);

        let data = self
            .iter()
            .zip(other.iter())
            .map(|(&a, &b)| op(a, b))
            .collect();

        Vector { data }
    }

    pub fn dot(&self, other: &Self) -> f64 {
        self.assert_size(&other);

        self.iter().zip(other.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn sub(&self, other: &Self) -> Vector {
        let op = |a, b| a - b;

        self.elemwise(&other, op)
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, other: &Vector) -> Self::Output {
        self.sub(&other)
    }
}

impl Index<usize> for Vector {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        self.at(index)
    }
}

impl IndexMut<usize> for Vector {
    fn index_mut(&mut self, index: usize) -> &mut f64 {
        self.mut_at(index)
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[ ")?;
        for s in self.iter() {
            write!(f, "{s} ")?;
        }
        write!(f, "]")
    }
}

pub mod matrix;
pub mod vector;
pub mod sqrmat;
pub mod ops;
pub mod row;
pub mod num;

pub use matrix::Matrix;
pub use vector::Vector;
pub use sqrmat::SquareMatrix;
pub use sqrmat::SqrMat;
pub use ops::Dot;
pub use row::Row;
pub use num::Zero;
pub use num::One;
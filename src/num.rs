pub trait Zero {
	fn zero() -> Self;
}

impl Zero for f64 {
	fn zero() -> Self { 0.0 }
}

impl Zero for i64 {
	fn zero() -> Self { 0 }
}

impl Zero for u64 {
	fn zero() -> Self { 0 }
}

impl Zero for i32 {
	fn zero() -> Self { 0 }
}

pub trait One {
	fn one() -> Self;
}

impl One for f64 {
	fn one() -> Self { 1.0 }
}

impl One for i64 {
	fn one() -> Self { 1 }
}

impl One for u64 {
	fn one() -> Self { 1 }
}

impl One for i32 {
	fn one() -> Self { 1 }
}


pub struct Row<'a> {
	data: &'a [f64],
}

impl<'a> Row<'a> {
	pub fn from(data: &'a [f64]) -> Self {
		Row { data }
	}

	pub fn data(&self) -> &[f64] {
        self.data
    }
}


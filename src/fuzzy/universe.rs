use crate::fuzzy::floatiterator::FloatIterator;

pub struct Universe {
    pub start: f64,
    pub stop: f64,
    pub nstep: u64,
    pub term: String
}
impl Universe {
	pub fn linspace(&self) -> FloatIterator {
		FloatIterator::new(self.start, self.stop, self.nstep)
	}
}

use crate::fuzzy::norms::min;
use crate::fuzzy::sets::Set;

pub fn intersection<T1: Set, T2: Set>(seta: &T1, setb: &T2, x: &f64) -> f64 {
    min(seta.mu(x), setb.mu(x))
}

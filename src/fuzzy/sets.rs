use crate::fuzzy::floatiterator::FloatIterator;
pub trait Set {
    fn mu(&self, x: &f64) -> f64;
}
pub fn get_triangular(a: f64, m: f64, b: f64) -> Triangular {
    Triangular { a, m, b, h: 1. }
}
pub struct Triangular {
    pub a: f64,
    pub m: f64,
    pub b: f64,
    pub h: f64,
}
impl Set for Triangular {
    fn mu(&self, x: &f64) -> f64 {
        if *x > self.a && *x <= self.m {
            self.h * (x - self.a) / (self.m - self.a)
        } else if *x > self.m && *x <= self.b {
            self.h * (self.b - x) / (self.b - self.m)
        } else {
            0.0
        }
    }
}

pub struct Trapezoidal {
    pub a: f64,
    pub m: f64,
    pub n: f64,
    pub b: f64,
    pub h: f64,
}
impl Set for Trapezoidal {
    fn mu(&self, x: &f64) -> f64 {
        if *x > self.a && *x <= self.m {
            self.h * (x - self.a) / (self.m - self.a)
        } else if *x > self.m && *x <= self.n {
            self.h
        } else if *x > self.m && *x <= self.b {
            self.h * (self.b - x) / (self.b - self.m)
        } else {
            0.0
        }
    }
}

pub struct Gaussian {
    pub m: f64,
    pub sigma: f64,
    pub h: f64,
}
impl Set for Gaussian {
    fn mu(&self, x: &f64) -> f64 {
        self.h * (-self.sigma * (*x - self.m).powf(2.0)).exp()
    }
}

pub struct Singleton {
    pub m: f64,
    pub h: f64,
}
impl Set for Singleton {
    fn mu(&self, x: &f64) -> f64 {
        if *x == self.m {
            self.h
        } else {
            0.0
        }
    }
}

pub struct Universe {
    pub start: f64,
    pub stop: f64,
    pub nstep: u64,
}
impl Universe {
    pub fn linspace(&self) -> FloatIterator {
        FloatIterator::new(self.start, self.stop, self.nstep)
    }
}

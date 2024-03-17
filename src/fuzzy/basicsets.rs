#[derive(Debug)]
pub struct Triangular {
    pub a: f64,
    pub m: f64,
    pub b: f64,
    pub h: f64,
    pub term: String
}
impl Triangular {
    pub fn mu(&self, x: &f64) -> f64 {
		if *x > self.a && *x <= self.m {
            self.h*(x-self.a)/(self.m-self.a)
        }
        else if *x > self.m && *x <= self.b{
            self.h*(self.b-x)/(self.b-self.m)
        }
        else {
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
    pub term: String
}
impl Trapezoidal {
    pub fn mu(&self, x: &f64) -> f64 {
		if *x > self.a && *x <= self.m {
            self.h*(x-self.a)/(self.m-self.a)
        }
        else if *x > self.m && *x <= self.n {
            self.h
        }
        else if *x > self.m && *x <= self.b{
            self.h*(self.b-x)/(self.b-self.m)
        }
        else {
            0.0
        }
    }
}

pub struct Gaussian {
    pub m: f64,
    pub sigma: f64,
    pub h: f64,
    pub term: String
}
impl Gaussian {
    pub fn mu(&self, x: &f64) -> f64 {
    	self.h*(-self.sigma*(*x-self.m).powf(2.0)).exp()
    }
}

pub struct Singleton {
    pub m: f64,
    pub h: f64,
    pub term: String
}
impl Singleton {
    pub fn mu(&self, x: &f64) -> f64 {
    	if *x == self.m {
    		self.h
    	}else{
    		0.0
    	}
    	
    }
}

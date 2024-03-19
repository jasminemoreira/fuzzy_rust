pub fn max<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn min<T: std::cmp::PartialOrd>(a: T, b: T) -> T {
    if a > b {
        b
    } else {
        a
    }
}

pub fn t1(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. / (1. + (((1. - a) / a).powf(*pt) + ((1. - b) / b).powf(*pt)).powf(1. / pt))
}

pub fn t2(a: &f64, b: &f64, pt: &f64) -> f64 {
    max(0., (1. + pt) * (a + b - 1.) - pt * a * b)
}

pub fn t3(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. - min(1., ((1. - a).powf(*pt) + (1. - b).powf(*pt)).powf(1. / pt))
}

pub fn t4(a: &f64, b: &f64) -> f64 {
    a * b
}

pub fn t5(a: &f64, b: &f64, pt: &f64) -> f64 {
    a * b / (pt + (1. - pt) * (a + b - a * b))
}

pub fn t6(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. / ((1. / a).powf(*pt) + (1. / b).powf(*pt) - 1.).powf(1. / pt)
}

pub fn t7(a: &f64, b: &f64, pt: &f64) -> f64 {
    (max(0., a.powf(*pt) + b.powf(*pt) - 1.)).powf(1. / pt)
}

pub fn t8(a: &f64, b: &f64) -> f64 {
    if *b == 1. {
        *a
    } else if *a == 1. {
        *b
    } else {
        0.
    }
}

pub fn t9(a: &f64, b: &f64) -> f64 {
    min(*a, *b)
}

pub fn s1(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. / (1. + ((a / (1. - a)).powf(*pt) + (b / (1. - b)).powf(*pt)).powf(1. / pt))
}

pub fn s2(a: &f64, b: &f64, pt: &f64) -> f64 {
    min(1., a + b - pt * a * b)
}

pub fn s3(a: &f64, b: &f64, pt: &f64) -> f64 {
    min(1., (a.powf(*pt) + b.powf(*pt)).powf(1. / pt))
}

pub fn s4(a: &f64, b: &f64) -> f64 {
    a + b - a * b
}

pub fn s5(a: &f64, b: &f64, pt: &f64) -> f64 {
    (a + b - a * b - (1. - pt) * a * b) / (1. - (1. - pt) * a * b)
}

pub fn s6(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. - 1. / (1. / (1. - a).powf(*pt) + 1. / (1. - b).powf(*pt) - 1.).powf(1. / pt)
}

pub fn s7(a: &f64, b: &f64, pt: &f64) -> f64 {
    1. - max(
        0.,
        ((1. - a).powf(*pt) + (1. - b).powf(*pt) - 1.).powf(1. / pt),
    )
}

pub fn s8(a: &f64, b: &f64) -> f64 {
    if *b == 0. {
        *a
    } else if *a == 0. {
        *b
    } else {
        1.
    }
}

pub fn s9(a: &f64, b: &f64) -> f64 {
    max(*a, *b)
}

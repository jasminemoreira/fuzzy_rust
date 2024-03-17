use crate::fuzzy::basicsets::Triangular;
use crate::fuzzy::universe::Universe;

pub mod fuzzy;

fn main() {
    // txProdPlan partition
    let txp_gd: &Triangular = &Triangular {
        a: -1.0e100,
        m: -2.,
        b: -1.,
        h: 1.,
        term: String::from("GD"),
    };
    let txp_md: &Triangular = &Triangular {
        a: -2.,
        m: -1.,
        b: 0.,
        h: 1.,
        term: String::from("MD"),
    };
    let txp_ok: &Triangular = &Triangular {
        a: -1.,
        m: 0.,
        b: 1.0,
        h: 1.,
        term: String::from("OK"),
    };
    let txp_me: &Triangular = &Triangular {
        a: 0.,
        m: 1.,
        b: 2.,
        h: 1.,
        term: String::from("ME"),
    };
    let txp_ge: &Triangular = &Triangular {
        a: 1.,
        m: 2.,
        b: 1.0e10,
        h: 1.,
        term: String::from("GE"),
    };

    // mse partition
    let mse_pe: &Triangular = &Triangular {
        a: -1.0e-100,
        m: 0.,
        b: 0.2,
        h: 1.,
        term: String::from("PE"),
    };
    let mse_me: &Triangular = &Triangular {
        a: 0.,
        m: 0.2,
        b: 0.4,
        h: 1.,
        term: String::from("ME"),
    };
    let mse_ge: &Triangular = &Triangular {
        a: 0.2,
        m: 0.4,
        b: 1.0e100,
        h: 1.,
        term: String::from("GE"),
    };

    // attention partition
    let att_ba: &Triangular = &Triangular {
        a: -1.0e-100,
        m: 0.,
        b: 5.,
        h: 1.,
        term: String::from("BA"),
    };
    let att_ma: &Triangular = &Triangular {
        a: 0.,
        m: 5.,
        b: 10.,
        h: 1.,
        term: String::from("MA"),
    };
    let att_ga: &Triangular = &Triangular {
        a: 5.,
        m: 10.,
        b: 1.0e100,
        h: 1.,
        term: String::from("GA"),
    };

    /*
    rules
    */
    let rules = [
        [txp_gd, mse_pe, att_ga],
        [txp_md, mse_pe, att_ma],
        [txp_ok, mse_pe, att_ba],
        [txp_me, mse_pe, att_ma],
        [txp_ge, mse_pe, att_ga],
        [txp_gd, mse_me, att_ga],
        [txp_md, mse_me, att_ma],
        [txp_ok, mse_me, att_ba],
        [txp_me, mse_me, att_ma],
        [txp_ge, mse_me, att_ga],
        [txp_gd, mse_ge, att_ga],
        [txp_md, mse_ge, att_ga],
        [txp_ok, mse_ge, att_ga],
        [txp_me, mse_ge, att_ga],
        [txp_ge, mse_ge, att_ga],
    ];

    let u1 = Universe {
        start: -10.0,
        stop: 10.0,
        nstep: 5,
        term: String::from("universe"),
    };
    for x in u1.linspace() {
        //println!("{}", norms::s9(&(txProdPlanPart[0].mu(&x)), &0.0))
        println!("{}", rules[0][0].mu(&x))
    }
    println!("Jasmine")
}

use crate::fuzzy::sets::{Singleton, Triangular, Universe};
use fuzzy::system::infer;

pub mod fuzzy;

fn main() {
    // txProdPlan partition
    let txp_gd: &Triangular = &Triangular {
        a: -1.0e100,
        m: -2.,
        b: -1.,
        h: 1.,
    };
    let txp_md: &Triangular = &Triangular {
        a: -2.,
        m: -1.,
        b: 0.,
        h: 1.,
    };
    let txp_ok: &Triangular = &Triangular {
        a: -1.,
        m: 0.,
        b: 1.,
        h: 1.,
    };
    let txp_me: &Triangular = &Triangular {
        a: 0.,
        m: 1.,
        b: 2.,
        h: 1.,
    };
    let txp_ge: &Triangular = &Triangular {
        a: 1.,
        m: 2.,
        b: 1.0e100,
        h: 1.,
    };

    // mse partition
    let mse_pe: &Triangular = &Triangular {
        a: -1.0e-100,
        m: 0.,
        b: 0.2,
        h: 1.,
    };
    let mse_me: &Triangular = &Triangular {
        a: 0.,
        m: 0.2,
        b: 0.4,
        h: 1.,
    };
    let mse_ge: &Triangular = &Triangular {
        a: 0.2,
        m: 0.4,
        b: 1.0e100,
        h: 1.,
    };

    // attention partition
    let att_ba: &Triangular = &Triangular {
        a: -1.0e-100,
        m: 0.,
        b: 5.,
        h: 1.,
    };
    let att_ma: &Triangular = &Triangular {
        a: 0.,
        m: 5.,
        b: 10.,
        h: 1.,
    };
    let att_ga: &Triangular = &Triangular {
        a: 5.,
        m: 10.,
        b: 1.0e100,
        h: 1.,
    };

    /*
    rules
    */
    let rules: [&[&Triangular; 3]; 15] = [
        &[txp_gd, mse_pe, att_ga],
        &[txp_md, mse_pe, att_ma],
        &[txp_ok, mse_pe, att_ba],
        &[txp_me, mse_pe, att_ma],
        &[txp_ge, mse_pe, att_ga],
        &[txp_gd, mse_me, att_ga],
        &[txp_md, mse_me, att_ga],
        &[txp_ok, mse_me, att_ba],
        &[txp_me, mse_me, att_ga],
        &[txp_ge, mse_me, att_ga],
        &[txp_gd, mse_ge, att_ga],
        &[txp_md, mse_ge, att_ga],
        &[txp_ok, mse_ge, att_ga],
        &[txp_me, mse_ge, att_ga],
        &[txp_ge, mse_ge, att_ga],
    ];

    let u_att: &Universe = &Universe {
        start: 0.,
        stop: 10.,
        nstep: 100000,
    };

    // inputs
    let txp_in: &Singleton = &Singleton { m: -0.9, h: 1. };
    let mse_in: &Singleton = &Singleton { m: 0.19, h: 1. };

    // para outros tipos de conjunto de entrada é preciso implementar a função de possibilidade
    let inp: [&Singleton; 2] = [txp_in, mse_in];
    let conc = infer(rules, inp, u_att);
    println!("Conclusão: {}", conc)
}

use crate::fuzzy::norms::{min, t9};
use crate::fuzzy::sets::{Set, Singleton, Triangular, Universe};

use super::sets::get_triangular;

pub fn intersection(seta: &dyn Set, setb: &dyn Set, x: &f64) -> f64 {
    min(seta.mu(x), setb.mu(x))
}

pub fn infer(txp: f64, mse: f64) -> f64 {
    /*
    Inferência:
    1 - matching das entradas;
    2 - agregação dos antecedentes;
    3 - conclusão individual;
    4 - defuzzificação;
    */
    // txProdPlan partition
    let txp_gd = &get_triangular(-1.0e100, -2., -1.);
    let txp_md = &get_triangular(-2., -1., 0.);
    let txp_ok = &get_triangular(-1., 0., 1.);
    let txp_me = &get_triangular(0., 1., 2.);
    let txp_ge = &get_triangular(1., 2., 1.0e100);

    // mse partition
    let mse_pe = &get_triangular(-1.0e-100, 0., 0.2);
    let mse_me = &get_triangular(0., 0.2, 0.4);
    let mse_ge = &get_triangular(0.2, 0.4, 1.0e100);

    // attention partition
    let att_ba = &get_triangular(-1.0e-100, 0., 5.);
    let att_ma = &get_triangular(0., 5., 10.);
    let att_ga = &get_triangular(5., 10., 1.0e100);

    // Attention Universe
    let univ: &Universe = &Universe {
        start: 0.,
        stop: 10.,
        nstep: 500,
    };

    // inputs
    let txp_in: &Singleton = &Singleton { m: txp, h: 1. };
    let mse_in: &Singleton = &Singleton { m: mse, h: 1. };
    // para outros tipos de conjunto de entrada é preciso implementar a função de possibilidade

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
    let mut aggregation = vec![];
    for rule in rules {
        let matching1 = intersection(rule[0], txp_in, &txp); // possibilidade para o caso de singleton
        let matching2 = intersection(rule[1], mse_in, &mse); // possibilidade para o caso de singleton
        aggregation.push(t9(&matching1, &matching2)); // agregação pelo mínimo
    }
    // conclusão total
    let mut max = 0.0;
    let mut att1 = 0.0;
    let mut att2 = -1.;
    for x in univ.linspace() {
        let mut xconc = 0.;
        for (i, rule) in rules.iter().enumerate() {
            let rconc = min(rule[2].mu(&x), aggregation[i]); // semântica Mamdani - vector implementa copy trait ;-)
            xconc = if rconc > xconc { rconc } else { xconc }; // união dos conjuntos (max)
        }
        if xconc > max {
            max = xconc;
            att1 = x;
        } else if xconc == max {
            att2 = x
        }
    }
    // defuzzificar MoM
    if att2 == -1. {
        att2 = att1;
    }
    (att1 + att2) / 2.
}

use crate::fuzzy::sets::{Singleton, Triangular, Universe};
use fuzzy::norms::{min, t9};
use fuzzy::sets::Set;
use fuzzy::system::intersection;

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
        b: 1.,
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
        b: 1.0e100,
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

    let u1 = &Universe {
        start: 0.,
        stop: 10.,
        nstep: 100000,
        term: String::from("universe"),
    };

    // inputs
    let txp_in: &Singleton = &Singleton {
        m: -0.9,
        h: 1.,
        term: String::from("IN"),
    };
    let mse_in: &Singleton = &Singleton {
        m: 0.19,
        h: 1.,
        term: String::from("IN"),
    };

    // para outros tipos de conjunto de entrada é preciso implementar a função de possibilidade
    let inp = [txp_in, mse_in];

    /*
    Inferência:
    1 - matching das entradas;
    2 - agregação dos antecedentes;
    3 - conclusão individual;
    4 - defuzzificação;
    */
    let mut aggregation = vec![];
    for rule in rules {
        let matching1 = intersection(rule[0], inp[0], &inp[0].m); // possibilidade para o caso de singleton
        let matching2 = intersection(rule[1], inp[1], &inp[1].m); // possibilidade para o caso de singleton
        aggregation.push(t9(&matching1, &matching2)); // agregação pelo mínimo
    }
    // conclusão total
    let mut max = 0.0;
    let mut att1 = 0.0;
    let mut att2 = -1.;
    for x in u1.linspace() {
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
    println!("Conclusão: {}", (att1 + att2) / 2.)
}

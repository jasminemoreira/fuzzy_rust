use crate::fuzzy::norms::{min, t9};
use crate::fuzzy::sets::{Set, Singleton, Triangular, Universe};

pub fn intersection<T1: Set, T2: Set>(seta: &T1, setb: &T2, x: &f64) -> f64 {
    min(seta.mu(x), setb.mu(x))
}

pub fn infer(rules: [&[&Triangular; 3]; 15], inp: [&Singleton; 2], univ: &Universe) -> f64 {
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

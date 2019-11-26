use crate::computation;
use crate::computation::Input;
use crate::newton;
use crate::relation::Retailer;
use crate::rrgame;
use computation::TVR_constraint;
use computation::{da_NP, da_TVR_constraint};
use computation::{dp_NP, dp_TVR_constraint};
use ndarray::arr1;
use ndarray::Array1;

pub fn rrgame_input_to_array(input: &Input, m: Retailer, lambda: f64) -> Array1<f64> {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let p_mg = &input.rrgame.parameter.p_mg;
    let a_mg = &input.rrgame.parameter.a_mg;

    let len = relation.products(m, decision).len();
    let mut result = Array1::zeros(len * 2 + 1);

    let mut index = 0;
    for g in relation.products(m, decision) {
        result[index] = p_mg[m][g];
        index += 1;
    }

    for g in relation.products(m, decision) {
        result[index] = a_mg[m][g];
        index += 1;
    }

    result[index] = lambda;

    result
}

pub fn rrgame_array_to_parameter(
    input: &Input,
    m: Retailer,
    array: &Array1<f64>,
) -> (rrgame::Parameter, f64) {
    let mut new_parameter = input.rrgame.parameter.clone();
    let p_mg = &mut new_parameter.p_mg;
    let a_mg = &mut new_parameter.a_mg;

    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let mut index = 0;
    for g in relation.products(m, decision) {
        p_mg[m][g] = array[index];
        index += 1;
    }

    for g in relation.products(m, decision) {
        a_mg[m][g] = array[index];
        index += 1;
    }

    let lambda = array[index];

    (new_parameter, lambda)
}

pub fn rrgame_f(old_input: &Input, m: Retailer, array: &Array1<f64>) -> Array1<f64> {
    let (parameter, lambda) = rrgame_array_to_parameter(old_input, m, array);
    let rrgame = rrgame::RRGame {
        parameter,
        ..(*old_input.rrgame)
    };

    let input = Input {
        rrgame: &rrgame,
        ..(*old_input)
    };

    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let products = relation.products(m, decision);
    let len = products.len();
    let mut result = Array1::zeros(len * 2 + 1);

    let mut index = 0;
    for j in products.iter() {
        result[index] = dp_NP(&input, m, *j) - lambda * dp_TVR_constraint(&input, m, *j);
        index += 1;
    }

    for j in products.iter() {
        result[index] = da_NP(&input, m, *j) - lambda * da_TVR_constraint(&input, m, *j);
        index += 1;
    }

    result[index] = TVR_constraint(&input, m);

    result
}

pub fn rrgame_solve(input: &Input, m: Retailer) -> Option<rrgame::Parameter> {
    let f = |a: &Array1<f64>| rrgame_f(input, m, a);
    let x0 = rrgame_input_to_array(input, m, 1.0);
    let len = input.relation.products(m, &input.mrgame.decision).len();
    let arr: Vec<f64> = (0..(len * 2 + 1)).map(|_| 0.000001).collect();
    let dx0 = arr1(&arr);

    let x = newton::newton_method(&f, &x0, &dx0)?;

    let (parameter, _) = rrgame_array_to_parameter(input, m, &x);

    Some(parameter)
}

pub mod solver_mrgame;

use crate::computation;
use crate::computation::Input;
use crate::newton;
use crate::relation::Retailer;
use crate::rrgame;
use crate::rrgame::RRGame;
use computation::{da_NP, da_TVR_constraint, da_Ta_constraint};
use computation::{dp_NP, dp_TVR_constraint};
use computation::{TVR_constraint, Ta_constraint};
use ndarray::arr1;
use ndarray::Array1;
pub use solver_mrgame::*;

#[derive(Copy, Clone)]
pub struct RRGameConstraints {
    pub TVR_active: bool,
    pub Ta_active: bool,
}

#[derive(Copy, Clone)]
pub struct RRGameLambdas {
    pub TVR: f64,
    pub Ta: f64,
}

impl RRGameConstraints {
    fn array_len(&self, product_count: usize) -> usize {
        let mut result = product_count * 2;

        if self.TVR_active {
            result += 1;
        }

        if self.Ta_active {
            result += 1;
        }

        result
    }

    fn append_lambdas(&self, array: &mut Array1<f64>, index: usize, lambdas: RRGameLambdas) {
        let mut index = index;
        if self.TVR_active {
            array[index] = lambdas.TVR;
            index += 1;
        }

        if self.Ta_active {
            array[index] = lambdas.Ta;
        }
    }

    fn get_lambdas(&self, array: &Array1<f64>, index: usize) -> RRGameLambdas {
        let mut index = index;
        let mut TVR = 0.0;
        let mut Ta = 0.0;

        if self.TVR_active {
            TVR = array[index];
            index += 1;
        }

        if self.Ta_active {
            Ta = array[index];
        }

        RRGameLambdas { TVR, Ta }
    }

    fn append_constraints(
        &self,
        input: &Input,
        m: Retailer,
        array: &mut Array1<f64>,
        index: usize,
    ) {
        let mut index = index;
        if self.TVR_active {
            array[index] = TVR_constraint(input, m);
            index += 1;
        }

        if self.Ta_active {
            array[index] = Ta_constraint(input, m);
        }
    }

    fn accept_result(
        &self,
        lambdas: RRGameLambdas,
        parameter: rrgame::Parameter,
    ) -> Option<rrgame::Parameter> {
        if self.TVR_active && lambdas.TVR < 0.0 {
            return None;
        }

        if self.Ta_active && lambdas.Ta < 0.0 {
            return None;
        }

        Some(parameter)
    }

    fn print(&self, lambdas: RRGameLambdas) {
        if self.TVR_active {
            println!("Lambda TVR: {}", lambdas.TVR);
        }
        if self.Ta_active {
            println!("Lambda Ta: {}", lambdas.Ta);
        }
    }
}

pub fn rrgame_input_to_array(
    input: &Input,
    m: Retailer,
    constraints: RRGameConstraints,
    lambdas: RRGameLambdas,
) -> Array1<f64> {
    let relation = input.relation;
    let decision = &input.mrgame.decision;
    let p_mg = &input.rrgame.parameter.p_mg;
    let a_mg = &input.rrgame.parameter.a_mg;

    let len = relation.products(m, decision).len();

    let mut result = Array1::zeros(constraints.array_len(len));

    let mut index = 0;
    for g in relation.products(m, decision) {
        result[index] = p_mg[m][g];
        index += 1;
    }

    for g in relation.products(m, decision) {
        result[index] = a_mg[m][g];
        index += 1;
    }

    constraints.append_lambdas(&mut result, index, lambdas);

    result
}

pub fn rrgame_array_to_parameter(
    input: &Input,
    m: Retailer,
    array: &Array1<f64>,
    constraints: RRGameConstraints,
) -> (rrgame::Parameter, RRGameLambdas) {
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

    (new_parameter, constraints.get_lambdas(array, index))
}

pub fn rrgame_f(
    old_input: &Input,
    m: Retailer,
    array: &Array1<f64>,
    constraints: RRGameConstraints,
) -> Array1<f64> {
    let (parameter, lambdas) = rrgame_array_to_parameter(old_input, m, array, constraints);
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
    let mut result = Array1::zeros(constraints.array_len(len));

    let mut index = 0;
    for j in products.iter() {
        result[index] = dp_NP(&input, m, *j);
        if constraints.TVR_active {
            result[index] -= lambdas.TVR * dp_TVR_constraint(&input, m, *j);
        }
        index += 1;
    }

    for j in products.iter() {
        result[index] = da_NP(&input, m, *j);
        if constraints.TVR_active {
            result[index] -= lambdas.TVR * da_TVR_constraint(&input, m, *j)
        }
        if constraints.Ta_active {
            result[index] -= lambdas.Ta * da_Ta_constraint(&input, m, *j);
        }
        index += 1;
    }

    constraints.append_constraints(&input, m, &mut result, index);

    result
}

fn rrgame_solve_constraints(
    input: &Input,
    m: Retailer,
    constraints: RRGameConstraints,
) -> Option<rrgame::Parameter> {
    let f = |a: &Array1<f64>| rrgame_f(input, m, a, constraints);
    let x0 = rrgame_input_to_array(input, m, constraints, RRGameLambdas { TVR: 1.0, Ta: 1.0 });
    let len = input.relation.products(m, &input.mrgame.decision).len();
    let arr: Vec<f64> = (0..(constraints.array_len(len)))
        .map(|_| 0.000001)
        .collect();
    let dx0 = arr1(&arr);

    let x = newton::newton_method(&f, &x0, &dx0, 0.2, 20)?;

    let (parameter, lambdas) = rrgame_array_to_parameter(input, m, &x, constraints);
    // constraints.print(lambdas);

    return constraints.accept_result(lambdas, parameter);
}

fn rrgame_try_constraint(
    old_parameter: Option<rrgame::Parameter>,
    input: &Input,
    m: Retailer,
    profit: &mut f64,
    TVR_active: bool,
    Ta_active: bool,
) -> Option<rrgame::Parameter> {
    let constraints = RRGameConstraints {
        TVR_active,
        Ta_active,
    };
    if let Some(parameter) = rrgame_solve_constraints(input, m, constraints) {
        let rrgame = RRGame {
            parameter: parameter,
            ..(*input.rrgame)
        };

        let (cst1, cst2, new_profit) = {
            let new_input = Input {
                rrgame: &rrgame,
                ..(*input)
            };
            (
                computation::TVR_constraint(&new_input, m),
                computation::Ta_constraint(&new_input, m),
                computation::NP(&new_input, m),
            )
        };

        // println!("TVR constraint: {}", cst1);
        // println!("Ta constraint: {}", cst2);

        let epsilon = 0.000001;
        if new_profit > *profit && cst1 <= epsilon && cst2 <= epsilon {
            *profit = new_profit;
            // println!("New profit: {}", *profit);
            return Some(rrgame.parameter);
        }
    }

    old_parameter
}

pub fn rrgame_solve(input: &Input, m: Retailer) -> Option<rrgame::Parameter> {
    let mut result: Option<rrgame::Parameter> = None;
    let mut profit = computation::NP(input, m);

    result = rrgame_try_constraint(result, input, m, &mut profit, false, false);
    // println!("----------------");
    result = rrgame_try_constraint(result, input, m, &mut profit, true, false);
    // println!("----------------");
    result = rrgame_try_constraint(result, input, m, &mut profit, false, true);
    // println!("----------------");
    result = rrgame_try_constraint(result, input, m, &mut profit, true, true);
    // println!("----------------");

    result
}

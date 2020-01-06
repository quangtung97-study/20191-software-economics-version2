use crate::computation;
use crate::computation::Input;
use crate::mrgame;
use crate::newton;
use ndarray::{arr1, Array1};

#[derive(Copy, Clone)]
pub struct MRGameConstraints {}

#[derive(Copy, Clone)]
pub struct MRGameLambdas {}

impl MRGameConstraints {
    fn array_len(&self) -> usize {
        0
    }

    fn append_lambdas(&self, array: &mut Array1<f64>, index: usize, lambdas: MRGameLambdas) {}

    fn print(&self, lambdas: MRGameLambdas) {}
}

pub fn mrgame_to_array(
    input: &Input,
    constraints: MRGameConstraints,
    lambdas: MRGameLambdas,
) -> Array1<f64> {
    let relation = &input.relation;
    let A_g = &input.mrgame.parameter.A_g;

    let len = relation.all_products().count();
    let mut result = Array1::zeros(len);

    let mut index = 0;
    for g in relation.all_products() {
        result[index] = A_g[g];
        index += 1;
    }

    result
}

pub fn mrgame_array_to_parameter(
    input: &Input,
    array: &Array1<f64>,
    constraints: MRGameConstraints,
) -> (mrgame::Parameter, MRGameLambdas) {
    let relation = &input.relation;

    let lambdas = MRGameLambdas {};
    let mut parameter = input.mrgame.parameter.clone();

    let mut index = 0;
    for g in relation.all_products() {
        parameter.A_g[g] = array[index];
        index += 1;
    }

    (parameter, lambdas)
}

pub fn mrgame_f(
    old_input: &Input,
    array: &Array1<f64>,
    constraints: MRGameConstraints,
) -> Array1<f64> {
    let (parameter, lambdas) = mrgame_array_to_parameter(old_input, array, constraints);

    let decision = old_input.mrgame.decision.clone();

    let mrgame = mrgame::MRGame {
        parameter: parameter,
        decision: decision,
    };

    let input = Input {
        mrgame: &mrgame,
        ..(*old_input)
    };

    let relation = input.relation;

    let len = array.len();
    let mut result = Array1::zeros(len);

    let mut index = 0;
    for j in relation.all_products() {
        result[index] = computation::dA_NP0(&input, j);
        index += 1;
    }

    result
}

pub fn mrgame_solve_constraints(
    input: &Input,
    constraints: MRGameConstraints,
) -> Option<mrgame::Parameter> {
    let f = |a: &Array1<f64>| mrgame_f(input, a, constraints);
    let lambdas = MRGameLambdas {};
    let x0 = mrgame_to_array(input, constraints, lambdas);
    let len = x0.len();

    let arr: Vec<f64> = (0..len).map(|_| 0.000001).collect();
    let dx0 = arr1(&arr);

    let x = newton::newton_method(&f, &x0, &dx0)?;

    let (parameter, lambdas) = mrgame_array_to_parameter(input, &x, constraints);

    Some(parameter)
}

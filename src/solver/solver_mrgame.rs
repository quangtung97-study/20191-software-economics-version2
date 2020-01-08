use crate::computation;
use crate::computation::Input;
use crate::mrgame;
use crate::newton;
use crate::relation::{MaterialMap, Relation};
use ndarray::{arr1, Array1};

#[derive(Copy, Clone)]
pub struct MRGameConstraints {}

#[derive(Clone)]
pub struct MRGameLambdas {
    // bom_l: MaterialMap<f64>,
// TVP: f64,
}

impl MRGameConstraints {
    fn print(&self, relation: &Relation, lambdas: &MRGameLambdas) {
        // println!("BOM_l");
        // for l in relation.all_materials() {
        //     print!("{}\t", lambdas.bom_l[l]);
        // }
        // println!("");
    }
}

impl MRGameLambdas {
    pub fn new(relation: &Relation) -> Self {
        Self {
            // bom_l: MaterialMap::new(relation, 1.0),
            // TVP: 2.0,
        }
    }
}

pub fn mrgame_to_array(
    input: &Input,
    constraints: MRGameConstraints,
    lambdas: &MRGameLambdas,
) -> Array1<f64> {
    let relation = &input.relation;
    let A_g = &input.mrgame.parameter.A_g;
    let c_m = &input.mrgame.parameter.c_m;
    let crm_s = &input.mrgame.parameter.crm_s;

    let len = relation.all_products().count()
        + relation.initial_retailers().count()
        + relation.all_suppliers().count();

    let mut result = Array1::zeros(len);

    let mut index = 0;
    for g in relation.all_products() {
        result[index] = A_g[g];
        index += 1;
    }

    for m in relation.initial_retailers() {
        result[index] = c_m[m];
        index += 1;
    }

    for s in relation.all_suppliers() {
        result[index] = crm_s[s];
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

    let mut lambdas = MRGameLambdas::new(relation);
    let mut parameter = input.mrgame.parameter.clone();

    let mut index = 0;
    for g in relation.all_products() {
        parameter.A_g[g] = array[index];
        index += 1;
    }

    for m in relation.initial_retailers() {
        parameter.c_m[m] = array[index];
        index += 1;
    }

    for s in relation.all_suppliers() {
        parameter.crm_s[s] = array[index];
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

    for m in relation.initial_retailers() {
        result[index] = computation::dc_NP0(&input, m);
        index += 1;
    }

    for s in relation.all_suppliers() {
        result[index] = computation::dcrm_NP0(&input, s);
        index += 1;
    }

    result
}

pub fn mrgame_solve_constraints(
    input: &Input,
    constraints: MRGameConstraints,
) -> Option<mrgame::Parameter> {
    let f = |a: &Array1<f64>| mrgame_f(input, a, constraints);
    let lambdas = MRGameLambdas::new(input.relation);
    let x0 = mrgame_to_array(input, constraints, &lambdas);
    let len = x0.len();

    let arr: Vec<f64> = (0..len).map(|_| 0.000001).collect();
    let dx0 = arr1(&arr);
    println!("{}", x0);

    let x = newton::newton_method(&f, &x0, &dx0, 0.2, 20)?;

    let (parameter, lambdas) = mrgame_array_to_parameter(input, &x, constraints);

    Some(parameter)
}

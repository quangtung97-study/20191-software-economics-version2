use crate::relation::{ProductMap, Relation, RetailerMap};

#[derive(Clone)]
pub struct Parameter {
    pub p_mg: RetailerMap<ProductMap<f64>>,
    pub a_mg: RetailerMap<ProductMap<f64>>,
}

#[derive(Clone)]
pub struct RRGame {
    pub parameter: Parameter,
}

impl Parameter {
    pub fn new(relation: &Relation) -> Self {
        Self {
            p_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
            a_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
        }
    }

    #[allow(dead_code)]
    pub fn input_p_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                self.p_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    #[allow(dead_code)]
    pub fn input_a_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                self.a_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_p_mg(&self, relation: &Relation) {
        println!("p_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.p_mg[m][g]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_a_mg(&self, relation: &Relation) {
        println!("a_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.a_mg[m][g]);
            }
            println!("");
        }
    }
}

impl RRGame {
    pub fn new(relation: &Relation) -> Self {
        Self {
            parameter: Parameter::new(relation),
        }
    }
}

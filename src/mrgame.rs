use crate::relation::{MaterialMap, ProductMap, Relation};
use crate::relation::{RetailerMap, SupplierMap};

pub struct Decision {
    pub product: ProductMap<bool>,
}

pub struct Parameter {
    pub A_g: ProductMap<f64>,
    pub c_m: RetailerMap<f64>,
    pub rho_g: ProductMap<f64>,
    pub crm_s: SupplierMap<f64>,
    pub drm_sl: SupplierMap<MaterialMap<f64>>,
}

pub struct MRGame {
    pub decision: Decision,
    pub parameter: Parameter,
}

impl Decision {
    pub fn new(relation: &Relation) -> Self {
        Self {
            product: ProductMap::new(relation, true),
        }
    }

    #[allow(dead_code)]
    pub fn input(&mut self, relation: &Relation, data: &[bool]) {
        for g in relation.all_products() {
            self.product[g] = data[g.id];
        }
    }

    #[allow(dead_code)]
    pub fn show(&self, relation: &Relation) {
        println!("Product Decision");
        for g in relation.all_products() {
            print!("{}\t", self.product[g]);
        }
        println!("");
    }
}

impl Parameter {
    pub fn new(relation: &Relation) -> Self {
        Self {
            A_g: ProductMap::new(relation, 0.0),
            c_m: RetailerMap::new(relation, 0.0),
            rho_g: ProductMap::new(relation, 0.0),
            crm_s: SupplierMap::new(relation, 0.0),
            drm_sl: SupplierMap::new(relation, MaterialMap::new(relation, 0.0)),
        }
    }

    #[allow(dead_code)]
    pub fn input_A_g_c_m(&mut self, relation: &Relation, A_g: &[f64], c_m: &[f64]) {
        for g in relation.all_products() {
            self.A_g[g] = A_g[g.id];
        }

        for m in relation.initial_retailers() {
            self.c_m[m] = c_m[m.id];
        }
    }

    #[allow(dead_code)]
    pub fn input_crm_s(&mut self, relation: &Relation, data: &[f64]) {
        for s in relation.all_suppliers() {
            self.crm_s[s] = data[s.id];
        }
    }

    #[allow(dead_code)]
    pub fn show(&self, relation: &Relation) {
        println!("A_g");
        for g in relation.all_products() {
            print!("{}\t", self.A_g[g]);
        }
        println!("");

        println!("c_m");
        for m in relation.initial_retailers() {
            print!("{}\t", self.c_m[m]);
        }
        println!("");

        println!("crm_s");
        for s in relation.all_suppliers() {
            print!("{}\t", self.crm_s[s]);
        }
        println!("");
    }
}

impl MRGame {
    pub fn new(relation: &Relation) -> Self {
        Self {
            decision: Decision::new(relation),
            parameter: Parameter::new(relation),
        }
    }
}

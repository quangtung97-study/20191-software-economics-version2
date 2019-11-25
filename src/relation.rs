use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Supplier {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct Material {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct Alternative {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct Module {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct Product {
    id: usize,
}

#[derive(Copy, Clone)]
pub struct Retailer {
    id: usize,
}

pub struct Relation {
    supplier_count: usize,
    material_count: usize,
    product_count: usize,
    retailer_count: usize,
    alternative_count: usize,
    module_count: usize,
    supplier_materials: Vec<(Supplier, Material)>,
    retailer_products: Vec<(Retailer, Product)>,
    alternative_modules: Vec<(Alternative, Module)>,
    material_alternatives: Vec<(Material, Alternative)>,
    alternative_products: Vec<(Alternative, Product)>,
}

impl Relation {
    pub fn new() -> Self {
        Relation {
            supplier_count: 0,
            material_count: 0,
            product_count: 0,
            retailer_count: 0,
            alternative_count: 0,
            module_count: 0,

            supplier_materials: Vec::new(),
            retailer_products: Vec::new(),
            alternative_modules: Vec::new(),
            material_alternatives: Vec::new(),
            alternative_products: Vec::new(),
        }
    }

    pub fn supplier_material_pairs(&mut self, pairs: &[(usize, usize)]) {
        self.supplier_count = pairs.iter().map(|p| p.0).max().unwrap() + 1;
        self.material_count = pairs.iter().map(|p| p.1).max().unwrap() + 1;
        self.supplier_materials = pairs
            .iter()
            .map(|p| (Supplier { id: p.0 }, Material { id: p.1 }))
            .collect();
    }

    pub fn retailer_product_pairs(&mut self, pairs: &[(usize, usize)]) {
        self.product_count = pairs.iter().map(|p| p.1).max().unwrap() + 1;
        self.retailer_count = pairs.iter().map(|p| p.0).max().unwrap() + 1;
        self.retailer_products = pairs
            .iter()
            .map(|p| (Retailer { id: p.0 }, Product { id: p.1 }))
            .collect();
    }

    pub fn alternative_module_pairs(&mut self, pairs: &[(usize, usize)]) {
        self.module_count = pairs.iter().map(|p| p.1).max().unwrap() + 1;
        self.alternative_count = pairs.iter().map(|p| p.0).max().unwrap() + 1;
        self.alternative_modules = pairs
            .iter()
            .map(|p| (Alternative { id: p.0 }, Module { id: p.1 }))
            .collect();
    }

    pub fn material_alternative_pairs(&mut self, pairs: &[(usize, usize)]) {
        self.material_alternatives = pairs
            .iter()
            .map(|p| (Material { id: p.0 }, Alternative { id: p.1 }))
            .collect();
    }

    pub fn alternative_product_pairs(&mut self, pairs: &[(usize, usize)]) {
        self.alternative_products = pairs
            .iter()
            .map(|p| (Alternative { id: p.0 }, Product { id: p.1 }))
            .collect();
    }

    fn initial_products(&self, retailer: Retailer) -> impl Iterator<Item = Product> + '_ {
        self.retailer_products
            .iter()
            .filter(move |p| p.0.id == retailer.id)
            .map(|p| p.1)
    }

    pub fn all_products(&self) -> impl Iterator<Item = Product> {
        (0..self.product_count).map(|id| Product { id })
    }

    fn initial_retailers(&self) -> impl Iterator<Item = Retailer> {
        (0..self.retailer_count).map(|id| Retailer { id: id })
    }
}

#[derive(Clone)]
pub struct RetailerMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> RetailerMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.retailer_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Retailer> for RetailerMap<T> {
    type Output = T;

    fn index(&self, index: Retailer) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Retailer> for RetailerMap<T> {
    fn index_mut(&mut self, index: Retailer) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

#[derive(Clone)]
pub struct ProductMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> ProductMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.product_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Product> for ProductMap<T> {
    type Output = T;

    fn index(&self, index: Product) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Product> for ProductMap<T> {
    fn index_mut(&mut self, index: Product) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

pub struct Constant {
    pub v_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub ea_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub beta_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub ep_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub K_mg: RetailerMap<ProductMap<f64>>,
    pub eA_mgy: RetailerMap<ProductMap<ProductMap<f64>>>,
    pub u_mgy: RetailerMap<ProductMap<ProductMap<f64>>>,
}

impl Constant {
    pub fn new(relation: &Relation) -> Self {
        Self {
            v_mgxy: RetailerMap::new(
                relation,
                ProductMap::new(
                    relation,
                    RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
                ),
            ),
            ea_mgxy: RetailerMap::new(
                relation,
                ProductMap::new(
                    relation,
                    RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
                ),
            ),
            beta_mgxy: RetailerMap::new(
                relation,
                ProductMap::new(
                    relation,
                    RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
                ),
            ),
            ep_mgxy: RetailerMap::new(
                relation,
                ProductMap::new(
                    relation,
                    RetailerMap::new(relation, ProductMap::new(relation, 1.0)),
                ),
            ),
            eA_mgy: RetailerMap::new(
                relation,
                ProductMap::new(relation, ProductMap::new(relation, 0.0)),
            ),
            u_mgy: RetailerMap::new(
                relation,
                ProductMap::new(relation, ProductMap::new(relation, 0.0)),
            ),
            K_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
        }
    }

    pub fn input_v_mgxy(&mut self, relation: &Relation, m: usize, g: usize, data: &[&[f64]]) {
        let m = Retailer { id: m };
        let g = Product { id: g };
        for x in relation.initial_retailers() {
            for y in relation.initial_products(x) {
                self.v_mgxy[m][g][x][y] = data[x.id][y.id];
            }
        }
    }

    pub fn input_ea_mgxy(&mut self, relation: &Relation, m: usize, g: usize, data: &[&[f64]]) {
        let m = Retailer { id: m };
        let g = Product { id: g };
        for x in relation.initial_retailers() {
            for y in relation.initial_products(x) {
                self.ea_mgxy[m][g][x][y] = data[x.id][y.id];
            }
        }
    }

    pub fn input_beta_mgxy(&mut self, relation: &Relation, m: usize, g: usize, data: &[&[f64]]) {
        let m = Retailer { id: m };
        let g = Product { id: g };
        for x in relation.initial_retailers() {
            for y in relation.initial_products(x) {
                self.beta_mgxy[m][g][x][y] = data[x.id][y.id];
            }
        }
    }

    pub fn input_K_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                self.K_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    pub fn input_aA_mgy(&mut self, relation: &Relation, data: &[&[f64]]) {
        let mut row: usize = 0;
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                for y in relation.all_products() {
                    self.eA_mgy[m][g][y] = data[row][y.id];
                }
                row += 1;
            }
        }
    }

    pub fn input_u_mgy(&mut self, relation: &Relation, data: &[&[f64]]) {
        let mut row: usize = 0;
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                for y in relation.all_products() {
                    self.u_mgy[m][g][y] = data[row][y.id];
                }
                row += 1;
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_v_mgxy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                println!("{} {}", m.id, g.id);
                for x in relation.initial_retailers() {
                    for y in relation.all_products() {
                        print!("{}\t", self.v_mgxy[m][g][x][y]);
                    }
                    println!("");
                }
                println!("-----------------------");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_ea_mgxy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                println!("{} {}", m.id, g.id);
                for x in relation.initial_retailers() {
                    for y in relation.all_products() {
                        print!("{}\t", self.ea_mgxy[m][g][x][y]);
                    }
                    println!("");
                }
                println!("-----------------------");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_beta_mgxy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                println!("{} {}", m.id, g.id);
                for x in relation.initial_retailers() {
                    for y in relation.all_products() {
                        print!("{}\t", self.beta_mgxy[m][g][x][y]);
                    }
                    println!("");
                }
                println!("-----------------------");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_ep_mgxy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                println!("{} {}", m.id, g.id);
                for x in relation.initial_retailers() {
                    for y in relation.all_products() {
                        print!("{}\t", self.ep_mgxy[m][g][x][y]);
                    }
                    println!("");
                }
                println!("-----------------------");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_K_mg(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.K_mg[m][g]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_eA_mgy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                print!("eA{}{}y | ", m.id, g.id);
                for y in relation.all_products() {
                    print!("{}\t", self.eA_mgy[m][g][y]);
                }
                println!("");
            }
        }
    }

    #[allow(dead_code)]
    pub fn show_u_mgy(&self, relation: &Relation) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                print!("u{}{}y | ", m.id, g.id);
                for y in relation.all_products() {
                    print!("{}\t", self.u_mgy[m][g][y]);
                }
                println!("");
            }
        }
    }
}

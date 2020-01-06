use crate::mrgame;
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Supplier {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Material {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Alternative {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Module {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Product {
    pub id: usize,
}

#[derive(Copy, Clone)]
pub struct Retailer {
    pub id: usize,
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

    pub fn initial_products(&self, retailer: Retailer) -> impl Iterator<Item = Product> + '_ {
        self.retailer_products
            .iter()
            .filter(move |p| p.0.id == retailer.id)
            .map(|p| p.1)
    }

    pub fn products(&self, retailer: Retailer, decision: &mrgame::Decision) -> Vec<Product> {
        self.initial_products(retailer)
            .filter(|g| decision.product[*g])
            .collect()
    }

    pub fn all_products(&self) -> impl Iterator<Item = Product> {
        (0..self.product_count).map(|id| Product { id })
    }

    pub fn initial_retailers(&self) -> impl Iterator<Item = Retailer> {
        (0..self.retailer_count).map(|id| Retailer { id: id })
    }

    pub fn all_suppliers(&self) -> impl Iterator<Item = Supplier> {
        (0..self.supplier_count).map(|id| Supplier { id })
    }

    pub fn all_materials(&self) -> impl Iterator<Item = Material> {
        (0..self.material_count).map(|id| Material { id })
    }

    pub fn all_alternatives(&self) -> impl Iterator<Item = Alternative> {
        (0..self.alternative_count).map(|id| Alternative { id })
    }

    pub fn all_modules(&self) -> impl Iterator<Item = Module> {
        (0..self.module_count).map(|id| Module { id })
    }

    pub fn materials(&self, supplier: Supplier) -> Vec<Material> {
        self.supplier_materials
            .iter()
            .filter(|p| p.0.id == supplier.id)
            .map(|p| p.1)
            .collect()
    }

    pub fn module(&self, alternative: Alternative) -> Module {
        self.alternative_modules
            .iter()
            .filter(|p| p.0.id == alternative.id)
            .map(|p| p.1)
            .next()
            .unwrap()
    }

    pub fn alternatives_of_module(&self, module: Module) -> Vec<Alternative> {
        self.alternative_modules
            .iter()
            .filter(|p| p.1.id == module.id)
            .map(|p| p.0)
            .collect()
    }

    pub fn products_for_alternative(
        &self,
        alternative: Alternative,
        decision: &mrgame::Decision,
    ) -> Vec<Product> {
        self.alternative_products
            .iter()
            .filter(|p| p.0.id == alternative.id)
            .map(|p| p.1)
            .filter(|g| decision.product[*g])
            .collect()
    }

    pub fn retailers(&self, product: Product) -> Vec<Retailer> {
        self.retailer_products
            .iter()
            .filter(|p| p.1.id == product.id)
            .map(|p| p.0)
            .collect()
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

#[derive(Clone)]
pub struct SupplierMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> SupplierMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.supplier_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Supplier> for SupplierMap<T> {
    type Output = T;

    fn index(&self, index: Supplier) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Supplier> for SupplierMap<T> {
    fn index_mut(&mut self, index: Supplier) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

#[derive(Clone)]
pub struct MaterialMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> MaterialMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.material_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Material> for MaterialMap<T> {
    type Output = T;

    fn index(&self, index: Material) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Material> for MaterialMap<T> {
    fn index_mut(&mut self, index: Material) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

#[derive(Clone)]
pub struct AlternativeMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> AlternativeMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.alternative_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Alternative> for AlternativeMap<T> {
    type Output = T;

    fn index(&self, index: Alternative) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Alternative> for AlternativeMap<T> {
    fn index_mut(&mut self, index: Alternative) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

#[derive(Clone)]
pub struct ModuleMap<T: Clone> {
    data: Vec<T>,
}

impl<T: Clone> ModuleMap<T> {
    pub fn new(relation: &Relation, v: T) -> Self {
        let mut data = Vec::new();
        data.resize_with(relation.alternative_count, || v.clone());
        Self { data }
    }
}

impl<T: Clone> Index<Module> for ModuleMap<T> {
    type Output = T;

    fn index(&self, index: Module) -> &Self::Output {
        &self.data[index.id]
    }
}

impl<T: Clone> IndexMut<Module> for ModuleMap<T> {
    fn index_mut(&mut self, index: Module) -> &mut Self::Output {
        &mut self.data[index.id]
    }
}

pub struct Constant {
    pub v_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub ea_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub beta_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub ep_mgxy: RetailerMap<ProductMap<RetailerMap<ProductMap<f64>>>>,
    pub K_mg: RetailerMap<ProductMap<f64>>,
    pub zeta_mg: RetailerMap<ProductMap<f64>>,
    pub HR_mg: RetailerMap<ProductMap<f64>>,
    pub TP_mg: RetailerMap<ProductMap<f64>>,
    pub eA_mgy: RetailerMap<ProductMap<ProductMap<f64>>>,
    pub u_mgy: RetailerMap<ProductMap<ProductMap<f64>>>,
    pub pw_g0: ProductMap<f64>,
    pub PCP_g: ProductMap<f64>,
    pub ORM_s: SupplierMap<f64>,
    pub HRM_l: MaterialMap<f64>,
    pub FCA_k: AlternativeMap<f64>,
    pub PCA_k: AlternativeMap<f64>,
    pub PCR_sl: SupplierMap<MaterialMap<f64>>,

    pub V_g: ProductMap<f64>,
    pub w_m: RetailerMap<f64>,
    pub TVR_m: RetailerMap<f64>,
    pub Ta_m: RetailerMap<f64>,

    pub delta_gk: ProductMap<AlternativeMap<usize>>,
    pub sigma_kl: AlternativeMap<MaterialMap<usize>>,
    pub FCM_j: ModuleMap<f64>,
    pub HP_g: ProductMap<f64>,
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
            zeta_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
            HR_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
            TP_mg: RetailerMap::new(relation, ProductMap::new(relation, 0.0)),
            pw_g0: ProductMap::new(relation, 0.0),
            PCP_g: ProductMap::new(relation, 0.0),
            ORM_s: SupplierMap::new(relation, 0.0),
            HRM_l: MaterialMap::new(relation, 0.0),
            FCA_k: AlternativeMap::new(relation, 0.0),
            PCA_k: AlternativeMap::new(relation, 0.0),
            PCR_sl: SupplierMap::new(relation, MaterialMap::new(relation, 0.0)),

            V_g: ProductMap::new(relation, 1.0),
            w_m: RetailerMap::new(relation, 1.0),
            TVR_m: RetailerMap::new(relation, 0.0),
            Ta_m: RetailerMap::new(relation, 0.0),

            delta_gk: ProductMap::new(relation, AlternativeMap::new(relation, 0)),
            sigma_kl: AlternativeMap::new(relation, MaterialMap::new(relation, 0)),
            FCM_j: ModuleMap::new(relation, 0.0),
            HP_g: ProductMap::new(relation, 0.0),
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

    pub fn input_zeta_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                self.zeta_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    pub fn input_HR_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.initial_products(m) {
                self.HR_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    pub fn input_TP_mg(&mut self, relation: &Relation, data: &[&[f64]]) {
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                self.TP_mg[m][g] = data[m.id][g.id];
            }
        }
    }

    pub fn input_eA_mgy(&mut self, relation: &Relation, data: &[&[f64]]) {
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

    pub fn input_pw_g0(&mut self, relation: &Relation, data: &[f64]) {
        for g in relation.all_products() {
            self.pw_g0[g] = data[g.id];
        }
    }

    pub fn input_PCP_g(&mut self, relation: &Relation, data: &[f64]) {
        for g in relation.all_products() {
            self.PCP_g[g] = data[g.id];
        }
    }

    pub fn input_ORM_s(&mut self, relation: &Relation, data: &[f64]) {
        for s in relation.all_suppliers() {
            self.ORM_s[s] = data[s.id];
        }
    }

    pub fn input_HRM_l(&mut self, relation: &Relation, data: &[f64]) {
        for l in relation.all_materials() {
            self.HRM_l[l] = data[l.id];
        }
    }

    pub fn input_FCA_k(&mut self, relation: &Relation, data: &[f64]) {
        for k in relation.all_alternatives() {
            self.FCA_k[k] = data[k.id];
        }
    }

    pub fn input_PCA_k(&mut self, relation: &Relation, data: &[f64]) {
        for k in relation.all_alternatives() {
            self.PCA_k[k] = data[k.id];
        }
    }

    pub fn input_PCR_sl(&mut self, relation: &Relation, data: &[&[f64]]) {
        for s in relation.all_suppliers() {
            for l in relation.all_materials() {
                self.PCR_sl[s][l] = data[s.id][l.id];
            }
        }
    }

    pub fn input_TVR_m(&mut self, relation: &Relation, data: &[f64]) {
        for m in relation.initial_retailers() {
            self.TVR_m[m] = data[m.id];
        }
    }

    pub fn input_Ta_m(&mut self, relation: &Relation, data: &[f64]) {
        for m in relation.initial_retailers() {
            self.Ta_m[m] = data[m.id];
        }
    }

    pub fn input_delta_gk(&mut self, relation: &Relation, data: &[&[usize]]) {
        for g in relation.all_products() {
            for k in relation.all_alternatives() {
                self.delta_gk[g][k] = data[g.id][k.id];
            }
        }
    }

    pub fn input_sigma_kl(&mut self, relation: &Relation, data: &[&[usize]]) {
        for k in relation.all_alternatives() {
            for l in relation.all_materials() {
                self.sigma_kl[k][l] = data[k.id][l.id];
            }
        }
    }

    pub fn input_FCM_j(&mut self, relation: &Relation, data: &[f64]) {
        for j in relation.all_modules() {
            self.FCM_j[j] = data[j.id];
        }
    }

    pub fn input_HP_g(&mut self, relation: &Relation, data: &[f64]) {
        for g in relation.all_products() {
            self.HP_g[g] = data[g.id];
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
        println!("K_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.K_mg[m][g]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_zeta_mg(&self, relation: &Relation) {
        println!("zeta_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.zeta_mg[m][g]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_HR_mg(&self, relation: &Relation) {
        println!("HR_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.HR_mg[m][g]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_TP_mg(&self, relation: &Relation) {
        println!("TP_mg");
        for m in relation.initial_retailers() {
            for g in relation.all_products() {
                print!("{}\t", self.TP_mg[m][g]);
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

    #[allow(dead_code)]
    pub fn show_pw_g0(&self, relation: &Relation) {
        println!("pw_g0");
        for g in relation.all_products() {
            print!("{}\t", self.pw_g0[g]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_PCP_g(&self, relation: &Relation) {
        println!("PCP_g");
        for g in relation.all_products() {
            print!("{}\t", self.PCP_g[g]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_ORM_s(&self, relation: &Relation) {
        println!("ORM_s");
        for s in relation.all_suppliers() {
            print!("{}\t", self.ORM_s[s]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_HRM_l(&self, relation: &Relation) {
        println!("HRM_l");
        for l in relation.all_materials() {
            print!("{}\t", self.HRM_l[l]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_FCA_k(&self, relation: &Relation) {
        println!("FCA_k");
        for k in relation.all_alternatives() {
            let j = relation.module(k);
            println!("{}: {}", j.id, self.FCA_k[k]);
        }
    }

    #[allow(dead_code)]
    pub fn show_PCA_k(&self, relation: &Relation) {
        println!("PCA_k");
        for k in relation.all_alternatives() {
            let j = relation.module(k);
            println!("{}: {}", j.id, self.PCA_k[k]);
        }
    }

    #[allow(dead_code)]
    pub fn show_PCR_sl(&self, relation: &Relation) {
        println!("PCR_sl");
        for s in relation.all_suppliers() {
            for l in relation.all_materials() {
                print!("{}\t", self.PCR_sl[s][l]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_TVR_m(&self, relation: &Relation) {
        println!("TVR_m");
        for m in relation.initial_retailers() {
            print!("{}\t", self.TVR_m[m]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_Ta_m(&self, relation: &Relation) {
        println!("Ta_m");
        for m in relation.initial_retailers() {
            print!("{}\t", self.Ta_m[m]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_delta_gk(&self, relation: &Relation) {
        println!("delta_gk");
        for g in relation.all_products() {
            for k in relation.all_alternatives() {
                print!("{}\t", self.delta_gk[g][k]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_sigma_kl(&self, relation: &Relation) {
        println!("sigma_kl");
        for k in relation.all_alternatives() {
            for l in relation.all_materials() {
                print!("{}\t", self.sigma_kl[k][l]);
            }
            println!("");
        }
    }

    #[allow(dead_code)]
    pub fn show_FCM_j(&self, relation: &Relation) {
        println!("FCM_j");
        for j in relation.all_modules() {
            print!("{}\t", self.FCM_j[j]);
        }
        println!("");
    }

    #[allow(dead_code)]
    pub fn show_HP_g(&self, relation: &Relation) {
        println!("HP_g");
        for g in relation.all_products() {
            print!("{}\t", self.HP_g[g]);
        }
        println!("");
    }
}

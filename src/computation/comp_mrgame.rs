use super::*;
use crate::mrgame;
use crate::relation::{Material, Supplier};

fn DA(input: &Input, k: Alternative) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let delta_gk = &input.constant.delta_gk;

    let mut sum = 0.0;
    for g in relation.products_for_alternative(k, &decision) {
        for m in relation.retailers(g) {
            sum += (delta_gk[g][k] as f64) * DP(input, m, g);
        }
    }

    sum
}

pub fn NP0(input: &Input) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let c_m = &input.mrgame.parameter.c_m;
    let HR_mg = &input.constant.HR_mg;
    let zeta_mg = &input.constant.zeta_mg;
    let TP_mg = &input.constant.TP_mg;
    let PCP_g = &input.constant.PCP_g;
    let ORM_s = &input.constant.ORM_s;
    let HRM_l = &input.constant.HRM_l;
    let PCA_k = &input.constant.PCA_k;
    let PCR_sl = &input.constant.PCR_sl;
    let FCM_j = &input.constant.FCM_j;
    let FCA_k = &input.constant.FCA_k;
    let HP_g = &input.constant.HP_g;
    let OP_m = &input.constant.OP_m;

    let crm_s = &input.mrgame.parameter.crm_s;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let A_g = &input.mrgame.parameter.A_g;

    let mut sum = 0.0;

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum += DP(input, m, g) * pw(input, m, g);
        }
    }

    sum -= {
        let mut inner_sum = 0.0;

        for m in relation.initial_retailers() {
            for g in relation.products(m, decision) {
                inner_sum += c_m[m] * DP(input, m, g) * HR_mg[m][g];
                inner_sum -= zeta_mg[m][g] * DP(input, m, g);
                inner_sum += OP_m[m] / c_m[m];
                inner_sum += c_m[m] * DP(input, m, g) * HP_g[g] / 2.0;
            }
        }

        inner_sum
    };

    sum -= {
        let mut inner_sum = 0.0;
        for s in relation.all_suppliers() {
            inner_sum += ORM_s[s] / crm_s[s];

            for l in relation.materials(s) {
                inner_sum += crm_s[s] * drm_sl[s][l] * HRM_l[l] / 2.0;
            }
        }
        inner_sum
    };

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum -= DP(input, m, g) * TP_mg[m][g];
            sum -= DP(input, m, g) * PCP_g[g];
        }
    }

    for k in relation.all_alternatives() {
        sum -= DA(input, k) * PCA_k[k];
    }

    for s in relation.all_suppliers() {
        for l in relation.materials(s) {
            sum -= drm_sl[s][l] * PCR_sl[s][l];
        }
    }

    for g in relation.all_products() {
        sum -= decision.fpp(g) * PCP_g[g];
    }

    for j in relation.all_modules() {
        sum -= decision.fpm(relation, j) * FCM_j[j];
    }

    for k in relation.all_alternatives() {
        sum -= decision.fpa(k) * FCA_k[k];
    }

    for g in relation.all_products() {
        sum -= A_g[g];
    }

    sum
}

pub fn NP0_bom_constraint(input: &Input, l: Material) -> f64 {
    let relation = input.relation;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let sigma_kl = &input.constant.sigma_kl;

    let mut sum = 0.0;
    for s in relation.suppliers_for_material(l) {
        sum += drm_sl[s][l];
    }

    for k in relation.all_alternatives() {
        sum -= sigma_kl[k][l] as f64 * DA(input, k);
    }

    sum
}

pub fn demmand_for_material(input: &Input, l: Material) -> f64 {
    let relation = input.relation;
    let sigma_kl = &input.constant.sigma_kl;

    let mut sum = 0.0;
    for k in relation.all_alternatives() {
        sum += sigma_kl[k][l] as f64 * DA(input, k);
    }

    sum
}

pub fn dA_DP(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let u_mgy = &input.constant.u_mgy;
    let eA_mgy = &input.constant.eA_mgy;
    let A_g = &input.mrgame.parameter.A_g;

    let eA = eA_mgy[m][g][j];

    u_mgy[m][g][j] * eA * safe_pow(A_g[j], eA - 1.0)
}

#[allow(dead_code)]
pub fn dA_DP_approx(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.01;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (DP(&new_input, m, g) - DP(input, m, g)) / 0.01
}

pub fn dA_pw(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let rho_g = &input.mrgame.parameter.rho_g;

    -rho_g[g] * dA_DP(input, m, g, j)
}

#[allow(dead_code)]
pub fn dA_pw_approx(input: &Input, m: Retailer, g: Product, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.01;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (pw(&new_input, m, g) - pw(input, m, g)) / 0.01
}

pub fn dA_DA(input: &Input, k: Alternative, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let delta_gk = &input.constant.delta_gk;

    let mut sum = 0.0;
    for g in relation.products_for_alternative(k, &decision) {
        for m in relation.retailers(g) {
            sum += (delta_gk[g][k] as f64) * dA_DP(input, m, g, j);
        }
    }

    sum
}

#[allow(dead_code)]
pub fn dA_DA_approx(input: &Input, k: Alternative, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.01;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (DA(&new_input, k) - DA(input, k)) / 0.01
}

pub fn dA_NP0(input: &Input, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let c_m = &input.mrgame.parameter.c_m;
    let HR_mg = &input.constant.HR_mg;
    let zeta_mg = &input.constant.zeta_mg;
    let TP_mg = &input.constant.TP_mg;
    let PCP_g = &input.constant.PCP_g;
    let ORM_s = &input.constant.ORM_s;
    let HRM_l = &input.constant.HRM_l;
    let PCA_k = &input.constant.PCA_k;
    let PCR_sl = &input.constant.PCR_sl;
    let FCM_j = &input.constant.FCM_j;
    let FCA_k = &input.constant.FCA_k;
    let HP_g = &input.constant.HP_g;

    let crm_s = &input.mrgame.parameter.crm_s;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let A_g = &input.mrgame.parameter.A_g;

    let mut sum = 0.0;

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum += dA_DP(input, m, g, j) * pw(input, m, g);
            sum += DP(input, m, g) * dA_pw(input, m, g, j);
        }
    }

    sum -= {
        let mut inner_sum = 0.0;

        for m in relation.initial_retailers() {
            for g in relation.products(m, decision) {
                inner_sum += c_m[m] * dA_DP(input, m, g, j) * HR_mg[m][g];
                inner_sum -= zeta_mg[m][g] * dA_DP(input, m, g, j);
                inner_sum += c_m[m] * dA_DP(input, m, g, j) * HP_g[g] / 2.0;
            }
        }

        inner_sum
    };

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum -= dA_DP(input, m, g, j) * TP_mg[m][g];
            sum -= dA_DP(input, m, g, j) * PCP_g[g];
        }
    }

    for k in relation.all_alternatives() {
        sum -= dA_DA(input, k, j) * PCA_k[k];
    }

    sum - 1.0
}

#[allow(dead_code)]
pub fn dA_NP0_approx(input: &Input, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.01;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0(&new_input) - NP0(input)) / 0.01
}

pub fn dc_NP0(input: &Input, m: Retailer) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let HR_mg = &input.constant.HR_mg;
    let HP_g = &input.constant.HP_g;
    let OP_m = &input.constant.OP_m;

    let c_m = &input.mrgame.parameter.c_m;

    let mut sum = 0.0;

    sum -= {
        let mut inner_sum = 0.0;

        for g in relation.products(m, decision) {
            inner_sum += DP(input, m, g) * HR_mg[m][g];
            inner_sum += -OP_m[m] / (c_m[m] * c_m[m]);
            inner_sum += DP(input, m, g) * HP_g[g] / 2.0;
        }

        inner_sum
    };

    sum
}

#[allow(dead_code)]
pub fn dc_NP0_approx(input: &Input, m: Retailer) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.c_m[m] += 0.000001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0(&new_input) - NP0(input)) / 0.000001
}

pub fn dc_TVR_constraint(input: &Input, m: Retailer) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let w_m = &input.constant.w_m;
    let V_g = &input.constant.V_g;

    let mut sum = 0.0;
    for g in relation.products(m, decision) {
        sum += w_m[m] * DP(input, m, g) * V_g[g] / 2.0;
    }

    sum
}

#[allow(dead_code)]
pub fn dc_TVR_constraint_approx(input: &Input, m: Retailer) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.c_m[m] += 0.01;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (TVR_constraint(&new_input, m) - TVR_constraint(input, m)) / 0.01
}

pub fn dcrm_NP0(input: &Input, s: Supplier) -> f64 {
    let relation = input.relation;

    let ORM_s = &input.constant.ORM_s;
    let HRM_l = &input.constant.HRM_l;

    let crm_s = &input.mrgame.parameter.crm_s;
    let drm_sl = &input.mrgame.parameter.drm_sl;

    let mut sum = 0.0;

    sum -= {
        let mut inner_sum = 0.0;
        inner_sum += -ORM_s[s] / (crm_s[s] * crm_s[s]);

        for l in relation.materials(s) {
            inner_sum += drm_sl[s][l] * HRM_l[l] / 2.0;
        }
        inner_sum
    };

    sum
}

#[allow(dead_code)]
pub fn dcrm_NP0_approx(input: &Input, s: Supplier) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.crm_s[s] += 0.000001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0(&new_input) - NP0(input)) / 0.000001
}

pub fn ddrm_NP0(input: &Input, s: Supplier, l: Material) -> f64 {
    let HRM_l = &input.constant.HRM_l;
    let PCR_sl = &input.constant.PCR_sl;

    let crm_s = &input.mrgame.parameter.crm_s;

    let mut sum = 0.0;
    sum -= crm_s[s] * HRM_l[l] / 2.0;
    sum -= PCR_sl[s][l];
    sum
}

#[allow(dead_code)]
pub fn ddrm_NP0_approx(input: &Input, s: Supplier, l: Material) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.drm_sl[s][l] += 0.0001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0(&new_input) - NP0(input)) / 0.0001
}

pub fn ddrm_NP0_bom_constraint(input: &Input, l: Material) -> f64 {
    input.relation.suppliers_for_material(l).len() as f64
}

pub fn dA_NP0_bom_constraint(input: &Input, l: Material, j: Product) -> f64 {
    let relation = input.relation;
    let sigma_kl = &input.constant.sigma_kl;

    let mut sum = 0.0;
    for k in relation.all_alternatives() {
        sum -= sigma_kl[k][l] as f64 * dA_DA(input, k, j);
    }

    sum
}

#[allow(dead_code)]
pub fn dA_NP0_bom_constraint_approx(input: &Input, l: Material, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.0001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0_bom_constraint(&new_input, l) - NP0_bom_constraint(input, l)) / 0.0001
}

pub fn NP0_TVP_constraint(input: &Input) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let crm_s = &input.mrgame.parameter.crm_s;
    let drm_sl = &input.mrgame.parameter.drm_sl;
    let c_m = &input.mrgame.parameter.c_m;

    let VRM_l = &input.constant.VRM_l;
    let V_g = &input.constant.V_g;
    let TVP = input.constant.TVP;

    let mut sum = 0.0;
    for s in relation.all_suppliers() {
        for l in relation.materials(s) {
            sum += crm_s[s] * drm_sl[s][l] * VRM_l[l] / 2.0;
        }
    }

    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum += c_m[m] * DP(input, m, g) * V_g[g] / 2.0
        }
    }

    sum - TVP
}

pub fn dA_NP0_TVP_constraint(input: &Input, j: Product) -> f64 {
    let relation = input.relation;
    let decision = &input.mrgame.decision;

    let c_m = &input.mrgame.parameter.c_m;

    let V_g = &input.constant.V_g;

    let mut sum = 0.0;
    for m in relation.initial_retailers() {
        for g in relation.products(m, decision) {
            sum += c_m[m] * dA_DP(input, m, g, j) * V_g[g] / 2.0
        }
    }

    sum
}

#[allow(dead_code)]
pub fn dA_NP0_TVP_constraint_approx(input: &Input, j: Product) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.A_g[j] += 0.0001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0_TVP_constraint(&new_input) - NP0_TVP_constraint(input)) / 0.0001
}

pub fn ddrm_NP0_TVP_constraint(input: &Input, s: Supplier, l: Material) -> f64 {
    let crm_s = &input.mrgame.parameter.crm_s;
    let VRM_l = &input.constant.VRM_l;

    crm_s[s] * VRM_l[l] / 2.0
}

#[allow(dead_code)]
pub fn ddrm_NP0_TVP_constraint_approx(input: &Input, s: Supplier, l: Material) -> f64 {
    let mut mrgame = input.mrgame.clone();
    mrgame.parameter.drm_sl[s][l] += 0.0001;

    let new_input = Input {
        mrgame: &mrgame,
        ..(*input)
    };

    (NP0_TVP_constraint(&new_input) - NP0_TVP_constraint(input)) / 0.0001
}

pub fn drm_sl_coefficients(input: &Input, s: Supplier, l: Material) -> f64 {
    let mut sum = 0.0;

    let crm_s = &input.mrgame.parameter.crm_s;

    let HRM_l = &input.constant.HRM_l;
    let PCR_sl = &input.constant.PCR_sl;

    sum -= crm_s[s] * HRM_l[l];
    sum -= PCR_sl[s][l];

    sum
}

pub fn find_best_supplier_for_material(input: &Input, l: Material) -> Supplier {
    let suppliers = input.relation.suppliers_for_material(l);
    let mut it = suppliers.iter();
    let mut result = *it.next().unwrap();
    let mut max = drm_sl_coefficients(input, result, l);

    for s in it {
        let value = drm_sl_coefficients(input, *s, l);
        if max < value {
            max = value;
            result = *s;
        }
    }

    result
}

pub fn apply_best_supplier_for_drm_sl(input: &Input) -> mrgame::Parameter {
    let relation = input.relation;

    let mut parameter = input.mrgame.parameter.clone();

    for l in relation.all_materials() {
        for s in relation.suppliers_for_material(l) {
            parameter.drm_sl[s][l] = 0.0;
        }
    }

    for l in relation.all_materials() {
        let s = find_best_supplier_for_material(input, l);
        parameter.drm_sl[s][l] = demmand_for_material(input, l);
    }

    parameter
}
